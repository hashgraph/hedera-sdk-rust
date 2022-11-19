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
use std::time::Duration;

use parking_lot::RwLock;
use tonic::transport::{
    Channel,
    Endpoint,
};

pub(crate) const MAINNET: &str = "mainnet-public.mirrornode.hedera.com:443";

pub(crate) const TESTNET: &str = "hcs.testnet.mirrornode.hedera.com:5600";

pub(crate) const PREVIEWNET: &str = "hcs.previewnet.mirrornode.hedera.com:5600";

pub(crate) struct MirrorNetwork {
    addresses: Vec<Cow<'static, str>>,
    channel: RwLock<Option<Channel>>,
}

impl MirrorNetwork {
    pub(crate) fn from_static(network: &[&'static str]) -> Self {
        let mut addresses = Vec::with_capacity(network.len());

        for address in network {
            addresses.push(Cow::Borrowed(*address));
        }

        Self { addresses, channel: RwLock::new(None) }
    }

    pub(crate) fn channel(&self) -> Channel {
        if let Some(channel) = &*self.channel.read_recursive() {
            return channel.clone();
        }

        let mut slot = self.channel.write();

        let endpoints = self.addresses.iter().map(|address| {
            let uri = format!("tcp://{address}");
            Endpoint::from_shared(uri)
                .unwrap()
                .keep_alive_timeout(Duration::from_secs(10))
                .keep_alive_while_idle(true)
                .tcp_keepalive(Some(Duration::from_secs(10)))
                .connect_timeout(Duration::from_secs(10))
        });

        let channel = Channel::balance_list(endpoints);

        *slot = Some(channel.clone());

        channel
    }
}
