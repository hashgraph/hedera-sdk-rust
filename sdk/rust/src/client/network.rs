use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

use parking_lot::RwLock;
use time::OffsetDateTime;
use tonic::transport::{Channel, Endpoint};

use crate::{AccountId, Error};

pub(crate) const TESTNET: &[(u64, &[&str])] = &[
    (3, &["0.testnet.hedera.com", "34.94.106.61", "50.18.132.211", "138.91.142.219"]),
    (4, &["1.testnet.hedera.com", "35.237.119.55", "3.212.6.13", "52.168.76.241"]),
    (5, &["2.testnet.hedera.com", "35.245.27.193", "52.20.18.86", "40.79.83.124"]),
    (6, &["3.testnet.hedera.com", "34.83.112.116", "54.70.192.33", "52.183.45.65"]),
    (7, &["4.testnet.hedera.com", "34.94.160.4", "54.176.199.109", "13.64.181.136"]),
    (8, &["5.testnet.hedera.com", "34.106.102.218", "35.155.49.147", "13.78.238.32"]),
    (9, &["6.testnet.hedera.com", "34.133.197.230", "52.14.252.207", "52.165.17.231"]),
];

pub(crate) struct Network {
    map: HashMap<AccountId, usize>,
    nodes: Vec<AccountId>,
    addresses: Vec<Vec<Cow<'static, str>>>,
    channels: Vec<RwLock<Option<Channel>>>,
    healthy: Vec<AtomicI64>,
}

impl Network {
    pub(crate) fn from_static(network: &'static [(u64, &'static [&'static str])]) -> Self {
        let mut map = HashMap::with_capacity(network.len());
        let mut nodes = Vec::with_capacity(network.len());
        let mut addresses = Vec::with_capacity(network.len());
        let mut channels = Vec::with_capacity(network.len());
        let mut healthy = Vec::with_capacity(network.len());

        for (i, (num, address)) in network.iter().enumerate() {
            let node_account_id = AccountId::from(*num);

            map.insert(node_account_id, i);
            nodes.push(node_account_id);
            addresses.push(address.into_iter().map(|address| Cow::Borrowed(*address)).collect());
            channels.push(RwLock::new(None));
            healthy.push(AtomicI64::new(0));
        }

        Self { map, nodes, addresses, channels, healthy }
    }

    pub(crate) fn node_indexes_for_ids(&self, ids: &[AccountId]) -> crate::Result<Vec<usize>> {
        let mut indexes = Vec::new();
        for id in ids {
            indexes.push(self.map.get(id).copied().ok_or_else(|| Error::NodeAccountUnknown(*id))?);
        }

        Ok(indexes)
    }

    pub(crate) fn mark_node_unhealthy(&self, node_index: usize) {
        self.healthy[node_index].store(
            (OffsetDateTime::now_utc() + time::Duration::minutes(30)).unix_timestamp(),
            Ordering::Relaxed,
        );
    }

    pub(crate) fn healthy_node_indexes(&self) -> Vec<usize> {
        let now = OffsetDateTime::now_utc().unix_timestamp();

        (0..self.nodes.len())
            .filter(|index| {
                // a healthy node has a healthiness of 0
                self.healthy[*index].load(Ordering::Relaxed) < now
            })
            .collect()
    }

    pub(crate) fn channel(&self, index: usize) -> (AccountId, Channel) {
        let id = self.nodes[index];

        if let Some(channel) = &*self.channels[index].read_recursive() {
            return (id, channel.clone());
        }

        let mut slot = self.channels[index].write();

        let addresses = &self.addresses[index];

        let endpoints = addresses.iter().map(|address| {
            let uri = format!("tcp://{}:50211", address);
            let endpoint =
                Endpoint::from_shared(uri).unwrap().connect_timeout(Duration::from_secs(5));

            endpoint
        });

        let channel = Channel::balance_list(endpoints);

        *slot = Some(channel.clone());

        (id, channel)
    }
}
