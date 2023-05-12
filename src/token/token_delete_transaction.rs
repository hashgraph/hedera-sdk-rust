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
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// Marks a token as deleted, though it will remain in the ledger.
///
/// The operation must be signed by the specified Admin Key of the Token.
///
/// Once deleted update, mint, burn, wipe, freeze, unfreeze, grant kyc, revoke
/// kyc and token transfer transactions will resolve to `TOKEN_WAS_DELETED`.
///
/// - If admin key is not set, Transaction will result in `TOKEN_IS_IMMUTABlE`.
/// - If invalid token is specified, transaction will result in `INVALID_TOKEN_ID`
pub type TokenDeleteTransaction = Transaction<TokenDeleteTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenDeleteTransactionData {
    /// The token to be deleted.
    token_id: Option<TokenId>,
}

impl TokenDeleteTransaction {
    /// Returns the token to be deleted.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token to be deleted.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }
}

impl TransactionData for TokenDeleteTransactionData {}

impl TransactionExecute for TokenDeleteTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).delete_token(request).await })
    }
}

impl ValidateChecksums for TokenDeleteTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenDeletion(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenDeleteTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenDeletion(self.to_protobuf())
    }
}

impl From<TokenDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: TokenDeleteTransactionData) -> Self {
        Self::TokenDelete(transaction)
    }
}

impl FromProtobuf<services::TokenDeleteTransactionBody> for TokenDeleteTransactionData {
    fn from_protobuf(pb: services::TokenDeleteTransactionBody) -> crate::Result<Self> {
        Ok(Self { token_id: Option::from_protobuf(pb.token)? })
    }
}

impl ToProtobuf for TokenDeleteTransactionData {
    type Protobuf = services::TokenDeleteTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenDeleteTransactionBody { token: self.token_id.to_protobuf() }
    }
}
