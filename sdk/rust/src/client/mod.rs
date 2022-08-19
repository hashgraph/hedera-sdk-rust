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

use parking_lot::RwLock;
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
    SignaturePair,
    Signer,
    TransactionId,
};

mod mirror_network;
mod network;

/// Managed client for use on the Hedera network.
#[derive(Clone)]
pub struct Client {
    network: Arc<Network>,
    mirror_network: Arc<MirrorNetwork>,
    operator_account_id: Arc<RwLock<Option<AccountId>>>,
    operator_signer: Arc<AsyncRwLock<Option<Box<dyn Signer>>>>,
    max_transaction_fee: Arc<AtomicU64>,
}

impl Client {
    /// Construct a Hedera client pre-configured for mainnet access.
    pub fn for_mainnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(MAINNET)),
            mirror_network: Arc::new(MirrorNetwork::from_static(&[mirror_network::MAINNET])),
            operator_account_id: Arc::new(RwLock::new(None)),
            operator_signer: Arc::new(AsyncRwLock::new(None)),
            max_transaction_fee: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Construct a Hedera client pre-configured for testnet access.
    pub fn for_testnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(TESTNET)),
            mirror_network: Arc::new(MirrorNetwork::from_static(&[mirror_network::TESTNET])),
            operator_account_id: Arc::new(RwLock::new(None)),
            operator_signer: Arc::new(AsyncRwLock::new(None)),
            max_transaction_fee: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    pub fn for_previewnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(PREVIEWNET)),
            mirror_network: Arc::new(MirrorNetwork::from_static(&[mirror_network::PREVIEWNET])),
            operator_account_id: Arc::new(RwLock::new(None)),
            operator_signer: Arc::new(AsyncRwLock::new(None)),
            max_transaction_fee: Arc::new(AtomicU64::new(0)),
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
        *self.operator_account_id.write() = Some(id);

        block_in_place(|| {
            *self.operator_signer.blocking_write() = Some(Box::new(key));
        });
    }

    /// Generate a new transaction ID from the stored operator account ID, if present.
    pub(crate) fn generate_transaction_id(&self) -> Option<TransactionId> {
        self.operator_account_id.read().map(TransactionId::generate)
    }

    pub(crate) async fn sign_with_operator(
        &self,
        body_bytes: &[u8],
    ) -> Result<SignaturePair, BoxStdError> {
        if let Some(signer) = &*self.operator_signer.read().await {
            signer.sign(body_bytes).await
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
        &self.max_transaction_fee
    }
}
