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

use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use tokio::sync::RwLock as AsyncRwLock;
use tokio::task::block_in_place;

use self::mirror_network::MirrorNetwork;
use crate::client::network::{
    Network,
    MAINNET,
    PREVIEWNET,
    TESTNET,
};
use crate::error::BoxStdError;
use crate::{
    AccountId,
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

/// Managed client for use on the Hedera network.
#[derive(Clone)]
pub struct Client {
    network: Arc<Network>,
    mirror_network: Arc<MirrorNetwork>,
    operator: Arc<AsyncRwLock<Option<Operator>>>,
    max_transaction_fee_tinybar: Arc<AtomicU64>,
}

impl Client {
    /// Construct a Hedera client pre-configured for mainnet access.
    #[must_use]
    pub fn for_mainnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(MAINNET)),
            mirror_network: Arc::new(MirrorNetwork::from_static(&[mirror_network::MAINNET])),
            operator: Arc::new(AsyncRwLock::new(None)),
            max_transaction_fee_tinybar: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Construct a Hedera client pre-configured for testnet access.
    #[must_use]
    pub fn for_testnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(TESTNET)),
            mirror_network: Arc::new(MirrorNetwork::from_static(&[mirror_network::TESTNET])),
            operator: Arc::new(AsyncRwLock::new(None)),
            max_transaction_fee_tinybar: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    #[must_use]
    pub fn for_previewnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(PREVIEWNET)),
            mirror_network: Arc::new(MirrorNetwork::from_static(&[mirror_network::PREVIEWNET])),
            operator: Arc::new(AsyncRwLock::new(None)),
            max_transaction_fee_tinybar: Arc::new(AtomicU64::new(0)),
        }
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
            *self.operator.blocking_write() = Some(Operator { account_id: id, signer: key });
        });
    }

    /// Generate a new transaction ID from the stored operator account ID, if present.
    pub(crate) fn generate_transaction_id(&self) -> Option<TransactionId> {
        self.operator.blocking_read().as_ref().map(|it| it.account_id).map(TransactionId::generate)
    }

    pub(crate) async fn sign_with_operator(
        &self,
        body_bytes: &[u8],
    ) -> Result<(PublicKey, Vec<u8>), BoxStdError> {
        if let Some(operator) = &*self.operator.read().await {
            Ok((operator.signer.public_key(), operator.signer.sign(&body_bytes)))
        } else {
            unreachable!()
        }
    }

    /// Gets a reference to the configured network.
    pub(crate) fn network(&self) -> &Network {
        &self.network
    }

    /// Gets a reference to the configured mirror network.
    pub(crate) fn mirror_network(&self) -> &MirrorNetwork {
        &self.mirror_network
    }

    /// Gets the maximum transaction fee the paying account is willing to pay.
    pub(crate) fn max_transaction_fee(&self) -> &AtomicU64 {
        &self.max_transaction_fee_tinybar
    }
}
