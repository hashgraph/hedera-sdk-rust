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

use std::iter;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    Ordering,
};
use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwapOption;
pub(crate) use operator::Operator;
use rand::thread_rng;

use self::mirror_network::MirrorNetwork;
use crate::client::network::Network;
use crate::{
    AccountId,
    LedgerId,
    PrivateKey,
    TransactionId,
};

mod mirror_network;
mod network;
mod operator;

struct ClientInner {
    network: Network,
    mirror_network: MirrorNetwork,
    operator: ArcSwapOption<Operator>,
    max_transaction_fee_tinybar: AtomicU64,
    ledger_id: ArcSwapOption<LedgerId>,
    auto_validate_checksums: AtomicBool,
}

/// Managed client for use on the Hedera network.
#[derive(Clone)]
pub struct Client(Arc<ClientInner>);

impl Client {
    fn with_network(
        network: Network,
        mirror_network: MirrorNetwork,
        ledger_id: impl Into<Option<LedgerId>>,
    ) -> Self {
        Self(Arc::new(ClientInner {
            network,
            mirror_network,
            operator: ArcSwapOption::new(None),
            max_transaction_fee_tinybar: AtomicU64::new(0),
            ledger_id: ArcSwapOption::new(ledger_id.into().map(Arc::new)),
            auto_validate_checksums: AtomicBool::new(false),
        }))
    }

    /// Construct a Hedera client pre-configured for mainnet access.
    #[must_use]
    pub fn for_mainnet() -> Self {
        Self::with_network(Network::mainnet(), MirrorNetwork::mainnet(), LedgerId::mainnet())
    }

    /// Construct a Hedera client pre-configured for testnet access.
    #[must_use]
    pub fn for_testnet() -> Self {
        Self::with_network(Network::testnet(), MirrorNetwork::testnet(), LedgerId::testnet())
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    #[must_use]
    pub fn for_previewnet() -> Self {
        Self::with_network(
            Network::previewnet(),
            MirrorNetwork::previewnet(),
            LedgerId::previewnet(),
        )
    }

    /// Construct a hedera client pre-configured for access to the given network.
    pub fn for_name(name: &str) -> crate::Result<Self> {
        match name {
            "mainnet" => Ok(Self::for_mainnet()),
            "testnet" => Ok(Self::for_testnet()),
            "previewnet" => Ok(Self::for_previewnet()),
            _ => Err(crate::Error::basic_parse(format!("Unknown network name {name}"))),
        }
    }

    // optimized function to avoid allocations/pointer chasing.
    // this shouldn't be exposed because it exposes repr.o
    pub(crate) fn ledger_id_internal(&self) -> arc_swap::Guard<Option<Arc<LedgerId>>> {
        self.0.ledger_id.load()
    }

    /// Sets the ledger ID for the Client's network.
    pub fn set_ledger_id(&self, ledger_id: Option<LedgerId>) {
        self.0.ledger_id.store(ledger_id.map(Arc::new))
    }

    pub(crate) fn random_node_ids(&self) -> Vec<AccountId> {
        let node_ids: Vec<_> = self.network().healthy_node_ids().collect();

        let node_sample_amount = (node_ids.len() + 2) / 3;

        let node_id_indecies =
            rand::seq::index::sample(&mut thread_rng(), node_ids.len(), node_sample_amount);

        node_id_indecies.into_iter().map(|index| node_ids[index]).collect()
    }

    pub(crate) fn auto_validate_checksums(&self) -> bool {
        self.0.auto_validate_checksums.load(Ordering::Relaxed)
    }

    /// Enable or disable automatic entity ID checksum validation.
    pub fn set_auto_validate_checksums(&self, value: bool) {
        self.0.auto_validate_checksums.store(value, Ordering::Relaxed);
    }

    /// Sets the account that will, by default, be paying for transactions and queries built with
    /// this client.
    ///
    /// The operator account ID is used to generate the default transaction ID for all transactions
    /// executed with this client.
    ///
    /// The operator private key is used to sign all transactions executed by this client.
    ///
    pub fn set_operator(&self, id: AccountId, key: PrivateKey) {
        self.0.operator.store(Some(Arc::new(Operator { account_id: id, signer: key })));
    }

    /// Generate a new transaction ID from the stored operator account ID, if present.
    pub(crate) async fn generate_transaction_id(&self) -> Option<TransactionId> {
        self.0.operator.load().as_deref().map(Operator::generate_transaction_id)
    }

    /// Gets a reference to the configured network.
    pub(crate) fn network(&self) -> &Network {
        &self.0.network
    }

    /// Gets a reference to the configured mirror network.
    pub(crate) fn mirror_network(&self) -> &MirrorNetwork {
        &self.0.mirror_network
    }

    /// Gets the maximum transaction fee the paying account is willing to pay.
    pub(crate) fn max_transaction_fee(&self) -> &AtomicU64 {
        &self.0.max_transaction_fee_tinybar
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn request_timeout(&self) -> Option<Duration> {
        // todo: implement this.
        None
    }

    pub(crate) fn operator_internal(&self) -> arc_swap::Guard<Option<Arc<Operator>>> {
        self.0.operator.load()
    }

    /// Send a ping to the given node.
    pub async fn ping(&self, node_account_id: AccountId) -> crate::Result<()> {
        crate::AccountBalanceQuery::new()
            .account_id(node_account_id)
            .node_account_ids(iter::once(node_account_id))
            .execute(self)
            .await?;

        Ok(())
    }

    /// Send a ping to the given node, canceling the ping after `timeout` has elapsed.
    pub async fn ping_with_timeout(
        &self,
        node_account_id: AccountId,
        timeout: Duration,
    ) -> crate::Result<()> {
        crate::AccountBalanceQuery::new()
            .account_id(node_account_id)
            .node_account_ids(iter::once(node_account_id))
            .execute_with_timeout(self, timeout)
            .await?;

        Ok(())
    }

    /// Send a ping to all nodes.
    pub async fn ping_all(&self) -> crate::Result<()> {
        futures_util::future::try_join_all(
            self.network().node_ids().iter().map(|it| self.ping(dbg!(*it))),
        )
        .await?;

        Ok(())
    }

    /// Send a ping to all nodes, canceling the ping after `timeout` has elapsed.
    pub async fn ping_all_with_timeout(&self, timeout: Duration) -> crate::Result<()> {
        futures_util::future::try_join_all(
            self.network().node_ids().iter().map(|it| self.ping_with_timeout(*it, timeout)),
        )
        .await?;

        Ok(())
    }
}
