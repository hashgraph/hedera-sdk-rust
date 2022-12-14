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

use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    Ordering,
};
use std::sync::Arc;

use tokio::sync::RwLock as AsyncRwLock;
use tokio::task::block_in_place;

use self::mirror_network::MirrorNetwork;
use crate::client::network::Network;
use crate::error::BoxStdError;
use crate::{
    AccountId,
    LedgerId,
    PrivateKey,
    PublicKey,
    TransactionId,
};

mod mirror_network;
mod network;

struct Operator {
    account_id: AccountId,
    signer: PrivateKey,
}

struct ClientInner {
    network: Network,
    mirror_network: MirrorNetwork,
    operator: AsyncRwLock<Option<Operator>>,
    max_transaction_fee_tinybar: AtomicU64,
    ledger_id: AsyncRwLock<Option<LedgerId>>,
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
            operator: AsyncRwLock::new(None),
            max_transaction_fee_tinybar: AtomicU64::new(0),
            ledger_id: AsyncRwLock::new(ledger_id.into()),
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

    pub(crate) async fn ledger_id(&self) -> Option<LedgerId> {
        self.0.ledger_id.read().await.clone()
    }

    /// Sets the ledger ID for the Client's network.
    pub fn set_ledger_id(&self, ledger_id: Option<LedgerId>) {
        block_in_place(|| *self.0.ledger_id.blocking_write() = ledger_id);
    }

    pub(crate) fn auto_validate_checksums(&self) -> bool {
        self.0.auto_validate_checksums.load(Ordering::Relaxed)
    }

    /// Enable or disable automatic entity ID checksum validation.
    pub fn set_auto_validate_checksums(&self, value: bool) {
        self.0.auto_validate_checksums.store(value, Ordering::Relaxed)
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
        block_in_place(|| {
            *self.0.operator.blocking_write() = Some(Operator { account_id: id, signer: key });
        });
    }

    /// Generate a new transaction ID from the stored operator account ID, if present.
    pub(crate) async fn generate_transaction_id(&self) -> Option<TransactionId> {
        self.0.operator.read().await.as_ref().map(|it| it.account_id).map(TransactionId::generate)
    }

    pub(crate) async fn sign_with_operator(
        &self,
        body_bytes: &[u8],
    ) -> Result<(PublicKey, Vec<u8>), BoxStdError> {
        if let Some(operator) = &*self.0.operator.read().await {
            Ok((operator.signer.public_key(), operator.signer.sign(body_bytes)))
        } else {
            unreachable!()
        }
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
    pub(crate) fn get_request_timeout(&self) -> Option<std::time::Duration> {
        // todo: implement this.
        None
    }
}
