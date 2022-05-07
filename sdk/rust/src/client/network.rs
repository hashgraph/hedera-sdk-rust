use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use http::uri::Authority;
use parking_lot::RwLock;
use tonic::transport::{Channel, Endpoint, Uri};

use crate::{AccountId, Client};

const TESTNET: &'static [(u64, &'static [&'static str])] = &[
    (3, &["0.testnet.hedera.com", "34.94.106.61", "50.18.132.211", "138.91.142.219"]),
    (4, &["1.testnet.hedera.com", "35.237.119.55", "3.212.6.13", "52.168.76.241"]),
    (5, &["2.testnet.hedera.com", "35.245.27.193", "52.20.18.86", "40.79.83.124"]),
    (6, &["3.testnet.hedera.com", "34.83.112.116", "54.70.192.33", "52.183.45.65"]),
    (7, &["4.testnet.hedera.com", "34.94.160.4", "54.176.199.109", "13.64.181.136"]),
    (8, &["5.testnet.hedera.com", "34.106.102.218", "35.155.49.147", "13.78.238.32"]),
    (9, &["6.testnet.hedera.com", "34.133.197.230", "52.14.252.207", "52.165.17.231"]),
];

impl Client {
    pub fn for_testnet() -> Self {
        Client { network: Arc::new(Network::from_static(TESTNET)) }
    }
}

#[derive(Clone)]
pub struct NetworkChannel {
    id: AccountId,
    channel: Channel,
}

impl NetworkChannel {
    pub(crate) fn id(&self) -> AccountId {
        self.id
    }

    pub(crate) fn crypto(self) -> CryptoServiceClient<Channel> {
        CryptoServiceClient::new(self.channel)
    }
}

pub(crate) struct Network {
    map: HashMap<AccountId, usize>,
    nodes: Vec<AccountId>,
    addresses: Vec<Vec<Cow<'static, str>>>,
    channels: Vec<RwLock<Option<NetworkChannel>>>,
}

impl Network {
    pub(crate) fn from_static(network: &'static [(u64, &'static [&'static str])]) -> Self {
        let mut map = HashMap::with_capacity(network.len());
        let mut nodes = Vec::with_capacity(network.len());
        let mut addresses = Vec::with_capacity(network.len());
        let mut channels = Vec::with_capacity(network.len());

        for (i, (num, address)) in network.iter().enumerate() {
            let node_account_id = AccountId::from(*num);

            map.insert(node_account_id, i);
            nodes.push(node_account_id);
            addresses.push(address.into_iter().map(|address| Cow::Borrowed(*address)).collect());
            channels.push(RwLock::new(None));
        }

        Self { map, nodes, addresses, channels }
    }

    pub(crate) fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn channel_nth(&self, index: usize) -> NetworkChannel {
        self.channel(self.nodes[index])
    }

    pub(crate) fn channel(&self, node: AccountId) -> NetworkChannel {
        let index = self.map.get(&node);
        let index = *index.unwrap();

        if let Some(channel) = &*self.channels[index].read_recursive() {
            return channel.clone();
        }

        let mut slot = self.channels[index].write();

        let addresses = &self.addresses[index];

        let endpoints = addresses.iter().map(|address| {
            let uri = format!("tcp://{}:50211", address);
            let endpoint = Endpoint::from_shared(uri).unwrap();

            endpoint
        });

        let channel = NetworkChannel { id: node, channel: Channel::balance_list(endpoints) };

        *slot = Some(channel.clone());

        channel
    }
}
