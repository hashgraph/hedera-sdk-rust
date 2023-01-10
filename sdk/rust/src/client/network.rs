/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{
    AtomicI64,
    Ordering,
};
use std::time::Duration;

use parking_lot::RwLock;
use time::OffsetDateTime;
use tonic::transport::{
    Channel,
    Endpoint,
};

use crate::{
    AccountId,
    Error,
};

pub(crate) const MAINNET: &[(u64, &[&str])] = &[
    (
        3,
        &[
            "35.237.200.180",
            "34.239.82.6",
            "13.82.40.153",
            "13.124.142.126",
            "15.164.44.66",
            "15.165.118.251",
        ],
    ),
    (4, &["35.186.191.247", "3.130.52.236", "137.116.36.18"]),
    (
        5,
        &[
            "35.192.2.25",
            "3.18.18.254",
            "104.43.194.202",
            "23.111.186.250",
            "74.50.117.35",
            "107.155.64.98",
        ],
    ),
    (
        6,
        &[
            "35.199.161.108",
            "13.52.108.243",
            "13.64.151.232",
            "13.235.15.32",
            "104.211.205.124",
            "13.71.90.154",
        ],
    ),
    (7, &["35.203.82.240", "3.114.54.4", "23.102.74.34"]),
    (8, &["35.236.5.219", "35.183.66.150", "23.96.185.18"]),
    (9, &["35.197.192.225", "35.181.158.250", "23.97.237.125", "31.214.8.131"]),
    (10, &["35.242.233.154", "3.248.27.48", "65.52.68.254", "179.190.33.184"]),
    (
        11,
        &[
            "35.240.118.96",
            "13.53.119.185",
            "23.97.247.27",
            "69.87.222.61",
            "96.126.72.172",
            "69.87.221.231",
        ],
    ),
    (12, &["35.204.86.32", "35.177.162.180", "51.140.102.228"]),
    (13, &["35.234.132.107", "34.215.192.104", "13.77.158.252"]),
    (14, &["35.236.2.27", "52.8.21.141", "40.114.107.85"]),
    (15, &["35.228.11.53", "3.121.238.26", "40.89.139.247"]),
    (16, &["34.91.181.183", "18.157.223.230", "13.69.120.73"]),
    (
        17,
        &[
            "34.86.212.247",
            "18.232.251.19",
            "40.114.92.39",
            "34.86.212.247",
            "18.232.251.19",
            "40.114.92.39",
        ],
    ),
    (18, &["172.105.247.67", "172.104.150.132", "139.162.156.222"]),
    (19, &["34.89.87.138", "18.168.4.59", "51.140.43.81", "13.246.51.42", "13.244.166.210"]),
    (20, &["34.82.78.255", "13.77.151.212"]),
    (21, &["34.76.140.109", "13.36.123.209"]),
    (22, &["34.64.141.166", "52.78.202.34"]),
    (23, &["35.232.244.145", "3.18.91.176"]),
    (24, &["34.89.103.38", "18.135.7.211"]),
    (25, &["34.93.112.7", "13.232.240.207"]),
    (26, &["34.87.150.174", "13.228.103.14"]),
    (27, &["34.125.200.96", "13.56.4.96"]),
    (28, &["35.198.220.75", "18.139.47.5"]),
];

pub(crate) const TESTNET: &[(u64, &[&str])] = &[
    (3, &["0.testnet.hedera.com", "34.94.106.61", "50.18.132.211", "138.91.142.219"]),
    (4, &["1.testnet.hedera.com", "35.237.119.55", "3.212.6.13", "52.168.76.241"]),
    (5, &["2.testnet.hedera.com", "35.245.27.193", "52.20.18.86", "40.79.83.124"]),
    (6, &["3.testnet.hedera.com", "34.83.112.116", "54.70.192.33", "52.183.45.65"]),
    (7, &["4.testnet.hedera.com", "34.94.160.4", "54.176.199.109", "13.64.181.136"]),
    (8, &["5.testnet.hedera.com", "34.106.102.218", "35.155.49.147", "13.78.238.32"]),
    (9, &["6.testnet.hedera.com", "34.133.197.230", "52.14.252.207", "52.165.17.231"]),
];

pub(crate) const PREVIEWNET: &[(u64, &[&str])] = &[
    (3, &["0.previewnet.hedera.com", "35.231.208.148", "3.211.248.172", "40.121.64.48"]),
    (4, &["1.previewnet.hedera.com", "35.199.15.177", "3.133.213.146", "40.70.11.202"]),
    (5, &["2.previewnet.hedera.com", "35.225.201.195", "52.15.105.130", "104.43.248.63"]),
    (6, &["3.previewnet.hedera.com", "35.247.109.135", "54.241.38.1", "13.88.22.47"]),
    (7, &["4.previewnet.hedera.com", "35.235.65.51", "54.177.51.127", "13.64.170.40"]),
    (8, &["5.previewnet.hedera.com", "34.106.247.65", "35.83.89.171", "13.78.232.192"]),
    (9, &["6.previewnet.hedera.com", "34.125.23.49", "50.18.17.93", "20.150.136.89"]),
];

pub(crate) struct Network {
    map: HashMap<AccountId, usize>,
    nodes: Vec<AccountId>,
    addresses: Vec<Vec<Cow<'static, str>>>,
    channels: Vec<RwLock<Option<Channel>>>,
    healthy: Vec<AtomicI64>,
}

impl Network {
    pub(super) fn mainnet() -> Self {
        Self::from_static(MAINNET)
    }

    pub(super) fn testnet() -> Self {
        Self::from_static(TESTNET)
    }

    pub(super) fn previewnet() -> Self {
        Self::from_static(PREVIEWNET)
    }

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
            addresses.push(address.iter().map(|address| Cow::Borrowed(*address)).collect());
            channels.push(RwLock::new(None));
            healthy.push(AtomicI64::new(0));
        }

        Self { map, nodes, addresses, channels, healthy }
    }

    pub(crate) fn node_ids(&self) -> &[AccountId] {
        &self.nodes
    }

    pub(crate) fn node_indexes_for_ids(&self, ids: &[AccountId]) -> crate::Result<Vec<usize>> {
        let mut indexes = Vec::new();
        for id in ids {
            indexes.push(self.map.get(id).copied().ok_or(Error::NodeAccountUnknown(*id))?);
        }

        Ok(indexes)
    }

    pub(crate) fn mark_node_unhealthy(&self, node_index: usize) {
        self.healthy[node_index].store(
            (OffsetDateTime::now_utc() + time::Duration::minutes(30)).unix_timestamp(),
            Ordering::Relaxed,
        );
    }

    pub(crate) fn healthy_node_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        let now = OffsetDateTime::now_utc().unix_timestamp();

        (0..self.nodes.len()).filter(move |index| {
            // a healthy node has a healthiness of 0
            self.healthy[*index].load(Ordering::Relaxed) < now
        })
    }

    pub(crate) fn healthy_node_ids(&self) -> impl Iterator<Item = AccountId> + '_ {
        self.healthy_node_indexes().map(|it| self.nodes[it])
    }

    pub(crate) fn channel(&self, index: usize) -> (AccountId, Channel) {
        let id = self.nodes[index];

        // Double lock check: We'd really rather not take a write lock if possible.
        // (paired with the below comment)
        if let Some(channel) = &*self.channels[index].read_recursive() {
            return (id, channel.clone());
        }

        let mut slot = self.channels[index].write();

        // Double lock check: We'd rather not replace the channel if one exists already, they aren't free.
        // (paired with the above comment)
        // Between returning `None` in the above `read` and getting
        // the `WriteGuard` some *other* write to this channel could've happened
        // causing the channel to be `Some` here, despite this thread not
        // changing it.
        if let Some(channel) = &*slot {
            return (id, channel.clone());
        }

        let addresses = &self.addresses[index];

        let endpoints = addresses.iter().map(|address| {
            let uri = format!("tcp://{address}:50211");
            Endpoint::from_shared(uri)
                .unwrap()
                .keep_alive_timeout(Duration::from_secs(10))
                .keep_alive_while_idle(true)
                .tcp_keepalive(Some(Duration::from_secs(10)))
                .connect_timeout(Duration::from_secs(10))
        });

        let channel = Channel::balance_list(endpoints);

        *slot = Some(channel.clone());

        (id, channel)
    }
}
