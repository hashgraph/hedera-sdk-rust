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

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::token::custom_fees::AnyCustomFee;
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
    LedgerId,
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// At consensus, updates a token type's fee schedule to the given list of custom fees.
///
/// If the target token type has no `fee_schedule_key`, resolves to `TokenHasNoFeeScheduleKey`.
/// Otherwise this transaction must be signed to the `fee_schedule_key`, or the transaction will
/// resolve to `InvalidSignature`.
///
/// If the `custom_fees` list is empty, clears the fee schedule or resolves to
/// `CustomScheduleAlreadyHasNoFees` if the fee schedule was already empty.
pub type TokenFeeScheduleUpdateTransaction = Transaction<TokenFeeScheduleUpdateTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenFeeScheduleUpdateTransactionData {
    /// The token whose fee schedule is to be updated.
    token_id: Option<TokenId>,

    /// The new custom fees to be assessed during a transfer.
    custom_fees: Vec<AnyCustomFee>,
}

impl TokenFeeScheduleUpdateTransaction {
    /// Returns the ID of the token that's being updated.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    // note(sr): what is being updated is implicit.
    /// Sets the ID of the token that's being updated.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }

    /// Returns the new custom fees to be assessed during a transfer.
    #[must_use]
    pub fn get_custom_fees(&self) -> &[AnyCustomFee] {
        &self.data().custom_fees
    }

    /// Sets the new custom fees to be assessed during a transfer.
    pub fn custom_fees(
        &mut self,
        custom_fees: impl IntoIterator<Item = AnyCustomFee>,
    ) -> &mut Self {
        self.data_mut().custom_fees = custom_fees.into_iter().collect();
        self
    }
}

impl TransactionData for TokenFeeScheduleUpdateTransactionData {}

impl TransactionExecute for TokenFeeScheduleUpdateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async {
            TokenServiceClient::new(channel).update_token_fee_schedule(request).await
        })
    }
}

impl ValidateChecksums for TokenFeeScheduleUpdateTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        // TODO: validate custom fees (they need an impl)
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenFeeScheduleUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenFeeScheduleUpdate(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenFeeScheduleUpdateTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenFeeScheduleUpdate(self.to_protobuf())
    }
}

impl From<TokenFeeScheduleUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenFeeScheduleUpdateTransactionData) -> Self {
        Self::TokenFeeScheduleUpdate(transaction)
    }
}

impl FromProtobuf<services::TokenFeeScheduleUpdateTransactionBody>
    for TokenFeeScheduleUpdateTransactionData
{
    fn from_protobuf(pb: services::TokenFeeScheduleUpdateTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            token_id: Option::from_protobuf(pb.token_id)?,
            custom_fees: Vec::from_protobuf(pb.custom_fees)?,
        })
    }
}

impl ToProtobuf for TokenFeeScheduleUpdateTransactionData {
    type Protobuf = services::TokenFeeScheduleUpdateTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenFeeScheduleUpdateTransactionBody {
            token_id: self.token_id.to_protobuf(),
            custom_fees: self.custom_fees.to_protobuf(),
        }
    }
}
