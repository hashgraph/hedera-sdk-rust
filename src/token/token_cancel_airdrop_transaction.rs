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

use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::pending_airdrop_id::PendingAirdropId;
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    BoxGrpcFuture,
    Error,
    FromProtobuf,
    ToProtobuf,
    Transaction,
    ValidateChecksums,
};

/// Token cancel airdrop
/// Remove one or more pending airdrops from state on behalf of the sender(s)
/// for each airdrop.
///
/// Each pending airdrop canceled SHALL be removed from state and SHALL NOT be available to claim.
/// Each cancellation SHALL be represented in the transaction body and SHALL NOT be restated
/// in the record file.
/// All cancellations MUST succeed for this transaction to succeed.
pub type TokenCancelAirdropTransaction = Transaction<TokenCancelAirdropTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenCancelAirdropTransactionData {
    /// The ID of the pending airdrop to cancel
    pending_airdrop_ids: Vec<PendingAirdropId>,
}

impl TokenCancelAirdropTransaction {
    /// Adds the list of pending airdrop identifiers to cancel.
    pub fn pending_airdrop_ids(
        &mut self,
        pending_airdrop_ids: impl IntoIterator<Item = PendingAirdropId>,
    ) -> &mut Self {
        self.data_mut().pending_airdrop_ids = pending_airdrop_ids.into_iter().collect();
        self
    }

    /// Returns the list of pending airdrop identifiers to cancel.
    pub fn get_pending_airdrop_ids(&self) -> Vec<PendingAirdropId> {
        self.data().pending_airdrop_ids.clone()
    }

    /// Adds a pending airdrop identifier to the list of pending airdrop identifiers to cancel.
    pub fn add_pending_airdrop_id(&mut self, pending_airdrop_id: PendingAirdropId) -> &mut Self {
        self.data_mut().pending_airdrop_ids.push(pending_airdrop_id);
        self
    }
}

impl TransactionData for TokenCancelAirdropTransactionData {}

impl TransactionExecute for TokenCancelAirdropTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).cancel_airdrop(request).await })
    }
}

impl ValidateChecksums for TokenCancelAirdropTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.pending_airdrop_ids
            .iter()
            .try_for_each(|pending_airdrop_id| pending_airdrop_id.validate_checksums(ledger_id))?;
        Ok(())
    }
}

impl ToTransactionDataProtobuf for TokenCancelAirdropTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenCancelAirdrop(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenCancelAirdropTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenCancelAirdrop(self.to_protobuf())
    }
}

impl From<TokenCancelAirdropTransactionData> for AnyTransactionData {
    fn from(transaction: TokenCancelAirdropTransactionData) -> Self {
        Self::TokenCancelAirdrop(transaction)
    }
}

impl ToProtobuf for TokenCancelAirdropTransactionData {
    type Protobuf = services::TokenCancelAirdropTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenCancelAirdropTransactionBody {
            pending_airdrops: self.pending_airdrop_ids.iter().map(|id| id.to_protobuf()).collect(),
        }
    }
}

impl FromProtobuf<services::TokenCancelAirdropTransactionBody>
    for TokenCancelAirdropTransactionData
{
    fn from_protobuf(pb: services::TokenCancelAirdropTransactionBody) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pending_airdrop_ids = pb
            .pending_airdrops
            .into_iter()
            .map(|id: services::PendingAirdropId| PendingAirdropId::from_protobuf(id))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { pending_airdrop_ids })
    }
}
