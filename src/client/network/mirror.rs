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
use std::ops::Deref;
use std::time::Duration;

use once_cell::sync::OnceCell;
use tonic::transport::{
    Channel,
    ClientTlsConfig,
    Endpoint,
};
use triomphe::Arc;

use crate::ArcSwap;

pub(crate) const MAINNET: &str = "mainnet-public.mirrornode.hedera.com:443";

pub(crate) const TESTNET: &str = "hcs.testnet.mirrornode.hedera.com:5600";

pub(crate) const PREVIEWNET: &str = "hcs.previewnet.mirrornode.hedera.com:5600";

#[derive(Default)]
pub(crate) struct MirrorNetwork(ArcSwap<MirrorNetworkData>);

impl Deref for MirrorNetwork {
    type Target = ArcSwap<MirrorNetworkData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MirrorNetwork {
    pub(crate) fn mainnet() -> Self {
        let tls_config =
            Some(ClientTlsConfig::new().domain_name(MAINNET.split_once(':').unwrap().0));

        Self(ArcSwap::new(Arc::new(MirrorNetworkData::from_static(&[MAINNET], tls_config))))
    }

    pub(crate) fn testnet() -> Self {
        Self(ArcSwap::new(Arc::new(MirrorNetworkData::from_static(&[TESTNET], None))))
    }

    pub(crate) fn previewnet() -> Self {
        Self(ArcSwap::new(Arc::new(MirrorNetworkData::from_static(&[PREVIEWNET], None))))
    }

    #[cfg(feature = "serde")]
    pub(crate) fn from_addresses(addresses: Vec<Cow<'static, str>>) -> Self {
        Self(ArcSwap::new(Arc::new(MirrorNetworkData::from_addresses(addresses))))
    }
}

#[derive(Clone, Default)]
pub(crate) struct MirrorNetworkData {
    addresses: Vec<Cow<'static, str>>,
    channel: OnceCell<Channel>,
    tls_config: Option<ClientTlsConfig>,
}

impl MirrorNetworkData {
    pub(crate) fn from_addresses(addresses: Vec<Cow<'static, str>>) -> Self {
        Self { addresses, channel: OnceCell::new(), tls_config: None }
    }

    pub(crate) fn from_static(
        network: &[&'static str],
        tls_config: Option<ClientTlsConfig>,
    ) -> Self {
        let mut addresses = Vec::with_capacity(network.len());

        for address in network {
            addresses.push(Cow::Borrowed(*address));
        }

        Self { addresses, channel: OnceCell::new(), tls_config }
    }

    pub(crate) fn channel(&self) -> Channel {
        self.channel
            .get_or_init(|| {
                let endpoints = self.addresses.iter().map(|address| {
                    let uri = format!("tcp://{address}");
                    let endpoint = Endpoint::from_shared(uri)
                        .unwrap()
                        .keep_alive_timeout(Duration::from_secs(10));

                    let endpoint = if let Some(tls_config) = self.tls_config.clone() {
                        endpoint.tls_config(tls_config).unwrap()
                    } else {
                        endpoint
                    };

                    endpoint
                        .keep_alive_while_idle(true)
                        .tcp_keepalive(Some(Duration::from_secs(10)))
                        .connect_timeout(Duration::from_secs(10))
                });

                Channel::balance_list(endpoints)
            })
            .clone()
    }

    pub(crate) fn addresses(&self) -> impl Iterator<Item = String> + '_ {
        self.addresses.iter().cloned().map(Cow::into_owned)
    }
}
