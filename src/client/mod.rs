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

use std::fmt;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    Ordering,
};
use std::time::Duration;

pub(crate) use network::{
    Network,
    NetworkData,
};
pub(crate) use operator::Operator;
use tokio::sync::watch;
use triomphe::Arc;

use self::network::managed::ManagedNetwork;
pub(crate) use self::network::mirror::MirrorNetwork;
use crate::ping_query::PingQuery;
use crate::signer::AnySigner;
use crate::{
    AccountId,
    ArcSwapOption,
    Error,
    LedgerId,
    PrivateKey,
};

mod network;
mod operator;

struct ClientInner {
    network: ManagedNetwork,
    operator: ArcSwapOption<Operator>,
    max_transaction_fee_tinybar: AtomicU64,
    ledger_id: ArcSwapOption<LedgerId>,
    auto_validate_checksums: AtomicBool,
    regenerate_transaction_ids: AtomicBool,
    network_update_tx: watch::Sender<Option<Duration>>,
}

/// Managed client for use on the Hedera network.
#[derive(Clone)]
pub struct Client(Arc<ClientInner>);

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // todo: put anything important here.
        f.debug_struct("Client").finish_non_exhaustive()
    }
}

impl Client {
    fn with_network(network: ManagedNetwork, ledger_id: impl Into<Option<LedgerId>>) -> Self {
        let network_update_tx = network::managed::spawn_network_update(
            network.clone(),
            Some(Duration::from_secs(24 * 60 * 60)),
        );

        Self(Arc::new(ClientInner {
            network,
            operator: ArcSwapOption::new(None),
            max_transaction_fee_tinybar: AtomicU64::new(0),
            ledger_id: ArcSwapOption::new(ledger_id.into().map(Arc::new)),
            auto_validate_checksums: AtomicBool::new(false),
            regenerate_transaction_ids: AtomicBool::new(true),
            network_update_tx,
        }))
    }

    /// Construct a Hedera client pre-configured for mainnet access.
    #[must_use]
    pub fn for_mainnet() -> Self {
        Self::with_network(ManagedNetwork::mainnet(), LedgerId::mainnet())
    }

    /// Construct a Hedera client pre-configured for testnet access.
    #[must_use]
    pub fn for_testnet() -> Self {
        Self::with_network(ManagedNetwork::testnet(), LedgerId::testnet())
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    #[must_use]
    pub fn for_previewnet() -> Self {
        Self::with_network(ManagedNetwork::previewnet(), LedgerId::previewnet())
    }

    /// Construct a hedera client pre-configured for access to the given network.
    ///
    /// Currently supported network names are `"mainnet"`, `"testnet"`, and `"previewnet"`.
    ///
    /// # Errors
    /// - [`Error::BasicParse`] if the network name is not a supported network name.
    pub fn for_name(name: &str) -> crate::Result<Self> {
        match name {
            "mainnet" => Ok(Self::for_mainnet()),
            "testnet" => Ok(Self::for_testnet()),
            "previewnet" => Ok(Self::for_previewnet()),
            _ => Err(Error::basic_parse(format!("Unknown network name {name}"))),
        }
    }

    // optimized function to avoid allocations/pointer chasing.
    // this shouldn't be exposed because it exposes repr.
    pub(crate) fn ledger_id_internal(&self) -> arc_swap::Guard<Option<Arc<LedgerId>>> {
        self.0.ledger_id.load()
    }

    /// Sets the ledger ID for the Client's network.
    pub fn set_ledger_id(&self, ledger_id: Option<LedgerId>) {
        self.0.ledger_id.store(ledger_id.map(Arc::new));
    }

    /// Returns true if checksums should be automatically validated.
    #[must_use]
    pub fn auto_validate_checksums(&self) -> bool {
        self.0.auto_validate_checksums.load(Ordering::Relaxed)
    }

    /// Enable or disable automatic entity ID checksum validation.
    pub fn set_auto_validate_checksums(&self, value: bool) {
        self.0.auto_validate_checksums.store(value, Ordering::Relaxed);
    }

    /// Returns true if transaction IDs should be automatically regenerated.
    ///
    /// This is `true` by default.
    #[must_use]
    pub fn default_regenerate_transaction_id(&self) -> bool {
        self.0.regenerate_transaction_ids.load(Ordering::Relaxed)
    }

    /// Enable or disable transaction ID regeneration.
    pub fn set_default_regenerate_transaction_id(&self, value: bool) {
        self.0.regenerate_transaction_ids.store(value, Ordering::Relaxed);
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
        self.0
            .operator
            .store(Some(Arc::new(Operator { account_id: id, signer: AnySigner::PrivateKey(key) })));
    }

    /// Gets a reference to the configured network.
    pub(crate) fn network(&self) -> &Network {
        &self.0.network.primary
    }

    /// Gets a reference to the configured mirror network.
    pub(crate) fn mirror_network(&self) -> &MirrorNetwork {
        &self.0.network.mirror
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

    // keep this internal (repr)
    pub(crate) fn load_operator(&self) -> arc_swap::Guard<Option<Arc<Operator>>> {
        self.0.operator.load()
    }

    // keep this internal (repr)
    pub(crate) fn full_load_operator(&self) -> Option<Arc<Operator>> {
        self.0.operator.load_full()
    }

    /// Send a ping to the given node.
    pub async fn ping(&self, node_account_id: AccountId) -> crate::Result<()> {
        PingQuery::new(node_account_id).execute(self, None).await
    }

    /// Send a ping to the given node, canceling the ping after `timeout` has elapsed.
    pub async fn ping_with_timeout(
        &self,
        node_account_id: AccountId,
        timeout: Duration,
    ) -> crate::Result<()> {
        PingQuery::new(node_account_id).execute(self, Some(timeout)).await
    }

    /// Send a ping to all nodes.
    pub async fn ping_all(&self) -> crate::Result<()> {
        futures_util::future::try_join_all(
            self.network().0.load().node_ids().iter().map(|it| self.ping(*it)),
        )
        .await?;

        Ok(())
    }

    /// Send a ping to all nodes, canceling the ping after `timeout` has elapsed.
    pub async fn ping_all_with_timeout(&self, timeout: Duration) -> crate::Result<()> {
        futures_util::future::try_join_all(
            self.network()
                .0
                .load()
                .node_ids()
                .iter()
                .map(|it| self.ping_with_timeout(*it, timeout)),
        )
        .await?;

        Ok(())
    }

    /// Returns the frequency at which the network will update (if it will update at all).
    #[must_use = "this function has no side-effects"]
    pub fn network_update_period(&self) -> Option<Duration> {
        *self.0.network_update_tx.borrow()
    }

    /// Sets the frequency at which the network will update.
    ///
    /// Note that network updates will not affect any in-flight requests.
    pub fn set_network_update_period(&self, period: Option<Duration>) {
        self.0.network_update_tx.send_if_modified(|place| {
            let changed = *place == period;
            if changed {
                *place = period;
            }

            changed
        });
    }
}
