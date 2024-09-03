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
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
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
    Transaction,
    ValidateChecksums,
};

/// Token claim airdrop
/// Complete one or more pending transfers on behalf of the
/// recipient(s) for an airdrop.
///
/// The sender MUST have sufficient balance to fulfill the airdrop at the
/// time of claim. If the sender does not have sufficient balance, the
/// claim SHALL fail.
/// Each pending airdrop successfully claimed SHALL be removed from state and
/// SHALL NOT be available to claim again.
/// Each claim SHALL be represented in the transaction body and
/// SHALL NOT be restated in the record file.
/// All claims MUST succeed for this transaction to succeed.
///
/// ### Record Stream Effects
/// The completed transfers SHALL be present in the transfer list.
///
pub type TokenClaimAirdropTransaction = Transaction<TokenClaimAirdropTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenClaimAirdropTransactionData {
    /// A list of one or more pending airdrop identifiers.
    ///
    /// This transaction MUST be signed by the account identified by
    /// the `receiver_id` for each entry in this list.
    /// This list MUST contain between 1 and 10 entries, inclusive.
    /// This list MUST NOT have any duplicate entries.
    pending_airdrop_ids: Vec<PendingAirdropId>,
}

impl TokenClaimAirdropTransaction {
    /// Adds the list of pending airdrop identifiers to claim.
    pub fn pending_airdrop_ids(
        &mut self,
        pending_airdrop_ids: impl IntoIterator<Item = PendingAirdropId>,
    ) -> &mut Self {
        self.data_mut().pending_airdrop_ids = pending_airdrop_ids.into_iter().collect();
        self
    }

    /// Returns the list of pending airdrop identifiers to claim.
    #[must_use]
    pub fn get_pending_airdrop_ids(&self) -> Vec<PendingAirdropId> {
        self.data().pending_airdrop_ids.clone()
    }

    /// Adds a pending airdrop identifier to the list of pending airdrop identifiers.
    pub fn add_pending_airdrop_id(&mut self, pending_airdrop_id: PendingAirdropId) -> &mut Self {
        self.data_mut().pending_airdrop_ids.push(pending_airdrop_id);
        self
    }
}

impl TransactionData for TokenClaimAirdropTransactionData {}

impl TransactionExecute for TokenClaimAirdropTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).claim_airdrop(request).await })
    }
}

impl ValidateChecksums for TokenClaimAirdropTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.pending_airdrop_ids
            .iter()
            .try_for_each(|pending_airdrop_id| pending_airdrop_id.validate_checksums(ledger_id))?;
        Ok(())
    }
}

impl ToTransactionDataProtobuf for TokenClaimAirdropTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenClaimAirdrop(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenClaimAirdropTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenClaimAirdrop(self.to_protobuf())
    }
}

impl From<TokenClaimAirdropTransactionData> for AnyTransactionData {
    fn from(transaction: TokenClaimAirdropTransactionData) -> Self {
        Self::TokenClaimAirdrop(transaction)
    }
}

impl ToProtobuf for TokenClaimAirdropTransactionData {
    type Protobuf = services::TokenClaimAirdropTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenClaimAirdropTransactionBody {
            pending_airdrops: self.pending_airdrop_ids.iter().map(|id| id.to_protobuf()).collect(),
        }
    }
}

impl FromProtobuf<services::TokenClaimAirdropTransactionBody> for TokenClaimAirdropTransactionData {
    fn from_protobuf(pb: services::TokenClaimAirdropTransactionBody) -> crate::Result<Self>
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
