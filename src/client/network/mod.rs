/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the[License");
 * you]may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an[AS IS" BASIS]
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

pub(super) mod managed;
pub(super) mod mirror;

use std::collections::{
    BTreeSet,
    HashMap,
};
use std::net::Ipv4Addr;
use std::sync::atomic::{
    AtomicI64,
    Ordering,
};
use std::time::Duration;

use once_cell::sync::OnceCell;
use rand::thread_rng;
use time::OffsetDateTime;
use tonic::transport::{
    Channel,
    Endpoint,
};
use triomphe::Arc;

use crate::{
    AccountId,
    ArcSwap,
    Error,
    NodeAddressBook,
};

const fn ips<const N: usize>(ips: [[u8; 4]; N]) -> [Ipv4Addr; N] {
    let mut index: usize = 0;

    // note: this will always be filled with output values.
    let mut out_ips: [Ipv4Addr; N] = [Ipv4Addr::UNSPECIFIED; N];

    while index < N {
        let [a, b, c, d] = ips[index];
        out_ips[index] = Ipv4Addr::new(a, b, c, d);
        index += 1;
    }

    out_ips
}

pub(crate) const MAINNET: &[(u64, &[Ipv4Addr])] = &[
    (
        3,
        &ips([
            [35, 237, 200, 180],
            [34, 239, 82, 6],
            [13, 82, 40, 153],
            [13, 124, 142, 126],
            [15, 164, 44, 66],
            [15, 165, 118, 251],
        ]),
    ),
    (4, &ips([[35, 186, 191, 247], [3, 130, 52, 236], [137, 116, 36, 18]])),
    (
        5,
        &ips([
            [35, 192, 2, 25],
            [3, 18, 18, 254],
            [104, 43, 194, 202],
            [23, 111, 186, 250],
            [74, 50, 117, 35],
            [107, 155, 64, 98],
        ]),
    ),
    (
        6,
        &ips([
            [35, 199, 161, 108],
            [13, 52, 108, 243],
            [13, 64, 151, 232],
            [13, 235, 15, 32],
            [104, 211, 205, 124],
            [13, 71, 90, 154],
        ]),
    ),
    (7, &ips([[35, 203, 82, 240], [3, 114, 54, 4], [23, 102, 74, 34]])),
    (8, &ips([[35, 236, 5, 219], [35, 183, 66, 150], [23, 96, 185, 18]])),
    (9, &ips([[35, 197, 192, 225], [35, 181, 158, 250], [23, 97, 237, 125], [31, 214, 8, 131]])),
    (10, &ips([[35, 242, 233, 154], [3, 248, 27, 48], [65, 52, 68, 254], [179, 190, 33, 184]])),
    (
        11,
        &ips([
            [35, 240, 118, 96],
            [13, 53, 119, 185],
            [23, 97, 247, 27],
            [69, 87, 222, 61],
            [96, 126, 72, 172],
            [69, 87, 221, 231],
        ]),
    ),
    (12, &ips([[35, 204, 86, 32], [35, 177, 162, 180], [51, 140, 102, 228]])),
    (13, &ips([[35, 234, 132, 107], [34, 215, 192, 104], [13, 77, 158, 252]])),
    (14, &ips([[35, 236, 2, 27], [52, 8, 21, 141], [40, 114, 107, 85]])),
    (15, &ips([[35, 228, 11, 53], [3, 121, 238, 26], [40, 89, 139, 247]])),
    (16, &ips([[34, 91, 181, 183], [18, 157, 223, 230], [13, 69, 120, 73]])),
    (
        17,
        &ips([
            [34, 86, 212, 247],
            [18, 232, 251, 19],
            [40, 114, 92, 39],
            [34, 86, 212, 247],
            [18, 232, 251, 19],
            [40, 114, 92, 39],
        ]),
    ),
    (18, &ips([[172, 105, 247, 67], [172, 104, 150, 132], [139, 162, 156, 222]])),
    (
        19,
        &ips([
            [34, 89, 87, 138],
            [18, 168, 4, 59],
            [51, 140, 43, 81],
            [13, 246, 51, 42],
            [13, 244, 166, 210],
        ]),
    ),
    (20, &ips([[34, 82, 78, 255], [13, 77, 151, 212]])),
    (21, &ips([[34, 76, 140, 109], [13, 36, 123, 209]])),
    (22, &ips([[34, 64, 141, 166], [52, 78, 202, 34]])),
    (23, &ips([[35, 232, 244, 145], [3, 18, 91, 176]])),
    (24, &ips([[34, 89, 103, 38], [18, 135, 7, 211]])),
    (25, &ips([[34, 93, 112, 7], [13, 232, 240, 207]])),
    (26, &ips([[34, 87, 150, 174], [13, 228, 103, 14]])),
    (27, &ips([[34, 125, 200, 96], [13, 56, 4, 96]])),
    (28, &ips([[35, 198, 220, 75], [18, 139, 47, 5]])),
];

pub(crate) const TESTNET: &[(u64, &[Ipv4Addr])] = &[
    (3, &ips([[34, 94, 106, 61], [50, 18, 132, 211], [138, 91, 142, 219]])),
    (4, &ips([[35, 237, 119, 55], [3, 212, 6, 13], [52, 168, 76, 241]])),
    (5, &ips([[35, 245, 27, 193], [52, 20, 18, 86], [40, 79, 83, 124]])),
    (6, &ips([[34, 83, 112, 116], [54, 70, 192, 33], [52, 183, 45, 65]])),
    (7, &ips([[34, 94, 160, 4], [54, 176, 199, 109], [13, 64, 181, 136]])),
    (8, &ips([[34, 106, 102, 218], [35, 155, 49, 147], [13, 78, 238, 32]])),
    (9, &ips([[34, 133, 197, 230], [52, 14, 252, 207], [52, 165, 17, 231]])),
];

pub(crate) const PREVIEWNET: &[(u64, &[Ipv4Addr])] = &[
    (3, &ips([[35, 231, 208, 148], [3, 211, 248, 172], [40, 121, 64, 48]])),
    (4, &ips([[35, 199, 15, 177], [3, 133, 213, 146], [40, 70, 11, 202]])),
    (5, &ips([[35, 225, 201, 195], [52, 15, 105, 130], [104, 43, 248, 63]])),
    (6, &ips([[35, 247, 109, 135], [54, 241, 38, 1], [13, 88, 22, 47]])),
    (7, &ips([[35, 235, 65, 51], [54, 177, 51, 127], [13, 64, 170, 40]])),
    (8, &ips([[34, 106, 247, 65], [35, 83, 89, 171], [13, 78, 232, 192]])),
    (9, &ips([[34, 125, 23, 49], [50, 18, 17, 93], [20, 150, 136, 89]])),
];

pub(crate) struct Network(pub(crate) ArcSwap<NetworkData>);

impl Network {
    pub(super) fn mainnet() -> Self {
        Self(ArcSwap::new(Arc::new(NetworkData::from_static(MAINNET))))
    }

    pub(super) fn testnet() -> Self {
        Self(ArcSwap::new(Arc::new(NetworkData::from_static(TESTNET))))
    }

    pub(super) fn previewnet() -> Self {
        Self(ArcSwap::new(Arc::new(NetworkData::from_static(PREVIEWNET))))
    }

    pub(crate) fn update_from_address_book(&self, address_book: NodeAddressBook) {
        // todo: skip the updating whem `map` is the same and `connections` is the same.
        self.0.rcu(|old| NetworkData::with_address_book(old, &address_book));
    }
}

pub(crate) struct NetworkData {
    map: HashMap<AccountId, usize>,
    node_ids: Box<[AccountId]>,
    // Health stuff has to be in an Arc because it needs to stick around even if the map changes.
    health: Box<[Arc<NodeHealth>]>,
    connections: Box<[NodeConnection]>,
}

impl NetworkData {
    pub(crate) fn from_static(network: &'static [(u64, &'static [Ipv4Addr])]) -> Self {
        let mut map = HashMap::with_capacity(network.len());
        let mut node_ids = Vec::with_capacity(network.len());
        let mut connections = Vec::with_capacity(network.len());
        let mut health = Vec::with_capacity(network.len());

        for (i, (num, address)) in network.iter().copied().enumerate() {
            let node_account_id = AccountId::from(num);

            map.insert(node_account_id, i);
            node_ids.push(node_account_id);
            health.push(Arc::default());
            connections.push(NodeConnection::new_static(address));
        }

        Self {
            map,
            node_ids: node_ids.into_boxed_slice(),
            health: health.into_boxed_slice(),
            connections: connections.into_boxed_slice(),
        }
    }

    fn with_address_book(old: &Self, address_book: &NodeAddressBook) -> Self {
        let address_book = &address_book.node_addresses;

        let mut map = HashMap::with_capacity(address_book.len());
        let mut node_ids = Vec::with_capacity(address_book.len());
        let mut connections = Vec::with_capacity(address_book.len());
        let mut health = Vec::with_capacity(address_book.len());

        for (i, address) in address_book.iter().enumerate() {
            let new: BTreeSet<_> = address
                .service_endpoints
                .iter()
                .filter_map(|it| (it.port() == NodeConnection::PLAINTEXT_PORT).then(|| *it.ip()))
                .collect();

            // if the node is the exact same we want to reuse everything (namely the connections and `healthy`).
            // if the node has different routes then we still want to reuse `healthy` but replace the channel with a new channel.
            // if the node just flat out doesn't exist in `old`, we want to add the new node.
            // and, last but not least, if the node doesn't exist in `new` we want to get rid of it.
            let upsert = match old.map.get(&address.node_account_id) {
                Some(&account) => {
                    let connection =
                        match old.connections[account].addresses.symmetric_difference(&new).count()
                        {
                            0 => old.connections[account].clone(),
                            _ => NodeConnection { addresses: new, channel: OnceCell::new() },
                        };

                    (old.health[account].clone(), connection)
                }
                None => {
                    (Arc::default(), NodeConnection { addresses: new, channel: OnceCell::new() })
                }
            };

            map.insert(address.node_account_id, i);
            node_ids.push(address.node_account_id);
            health.push(upsert.0);
            connections.push(upsert.1);
        }

        Self {
            map,
            node_ids: node_ids.into_boxed_slice(),
            health: health.into_boxed_slice(),
            connections: connections.into_boxed_slice(),
        }
    }

    pub(crate) fn node_ids(&self) -> &[AccountId] {
        &self.node_ids
    }

    pub(crate) fn node_indexes_for_ids(&self, ids: &[AccountId]) -> crate::Result<Vec<usize>> {
        let mut indexes = Vec::new();
        for id in ids {
            indexes.push(
                self.map
                    .get(id)
                    .copied()
                    .ok_or_else(|| Error::NodeAccountUnknown(Box::new(*id)))?,
            );
        }

        Ok(indexes)
    }

    pub(crate) fn mark_node_used(&self, node_index: usize, now: OffsetDateTime) {
        self.health[node_index].last_pinged.store(now.unix_timestamp(), Ordering::Release);
    }

    pub(crate) fn mark_node_unhealthy(&self, node_index: usize) {
        let now = OffsetDateTime::now_utc();
        self.health[node_index]
            .healthy
            .store((now + time::Duration::minutes(30)).unix_timestamp(), Ordering::Relaxed);
    }

    pub(crate) fn is_node_healthy(&self, node_index: usize, now: OffsetDateTime) -> bool {
        let now = now.unix_timestamp();

        // a healthy node has a healthiness before now.
        self.health[node_index].healthy.load(Ordering::Relaxed) < now
    }

    pub(crate) fn node_recently_pinged(&self, node_index: usize, now: OffsetDateTime) -> bool {
        !self.is_node_healthy(node_index, now)
            || self.health[node_index].last_pinged.load(Ordering::Relaxed)
                > (now - time::Duration::minutes(15)).unix_timestamp()
    }

    pub(crate) fn healthy_node_indexes(
        &self,
        time: OffsetDateTime,
    ) -> impl Iterator<Item = usize> + '_ {
        (0..self.node_ids.len()).filter(move |index| self.is_node_healthy(*index, time))
    }

    pub(crate) fn healthy_node_ids(&self) -> impl Iterator<Item = AccountId> + '_ {
        self.healthy_node_indexes(OffsetDateTime::now_utc()).map(|it| self.node_ids[it])
    }

    pub(crate) fn random_node_ids(&self) -> Vec<AccountId> {
        let node_ids: Vec<_> = self.healthy_node_ids().collect();

        let node_sample_amount = (node_ids.len() + 2) / 3;

        let node_id_indecies =
            rand::seq::index::sample(&mut thread_rng(), node_ids.len(), node_sample_amount);

        node_id_indecies.into_iter().map(|index| node_ids[index]).collect()
    }

    pub(crate) fn channel(&self, index: usize) -> (AccountId, Channel) {
        let id = self.node_ids[index];

        let channel = self.connections[index].channel();

        (id, channel)
    }
}

#[derive(Default)]
pub(crate) struct NodeHealth {
    healthy: AtomicI64,
    last_pinged: AtomicI64,
}

#[derive(Clone)]
struct NodeConnection {
    addresses: BTreeSet<Ipv4Addr>,
    channel: OnceCell<Channel>,
}

impl NodeConnection {
    const PLAINTEXT_PORT: u16 = 50211;

    fn new_static(addresses: &[Ipv4Addr]) -> NodeConnection {
        Self { addresses: addresses.iter().copied().collect(), channel: OnceCell::default() }
    }

    pub(crate) fn channel(&self) -> Channel {
        let channel = self
            .channel
            .get_or_init(|| {
                let addresses = self.addresses.iter().map(|it| {
                    Endpoint::from_shared(format!("tcp://{it}:50211"))
                        .unwrap()
                        .keep_alive_timeout(Duration::from_secs(10))
                        .keep_alive_while_idle(true)
                        .tcp_keepalive(Some(Duration::from_secs(10)))
                        .connect_timeout(Duration::from_secs(10))
                });

                Channel::balance_list(addresses)
            })
            .clone();

        channel
    }
}
