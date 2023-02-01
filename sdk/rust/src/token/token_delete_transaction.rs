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
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    LedgerId,
    TokenId,
    Transaction,
    TransactionId,
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

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
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
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token = self.token_id.to_protobuf();

        services::transaction_body::Data::TokenDeletion(services::TokenDeleteTransactionBody {
            token,
        })
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
#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use assert_matches::assert_matches;

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            TokenDeleteTransaction,
            TokenId,
        };

        //language=JSON
        const TOKEN_DELETE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenDelete",
  "tokenId": "0.0.1002"
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TokenDeleteTransaction::new();

            transaction.token_id(TokenId::from(1002));

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_DELETE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOKEN_DELETE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TokenDelete(transaction) => transaction);

            assert_eq!(data.token_id.unwrap(), TokenId::from(1002));

            Ok(())
        }
    }
}
