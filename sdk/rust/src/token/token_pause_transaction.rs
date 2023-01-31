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

use async_trait::async_trait;
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
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    LedgerId,
    TokenId,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

/// Pauses the Token from being involved in any kind of Transaction until it is unpaused.
///
/// Must be signed with the Token's pause key.
///
/// Once executed the Token is marked as paused and will be not able to be a part of any transaction.
/// The operation is idempotent - becomes a no-op if the Token is already Paused.
///
/// - If the provided token is not found, the transaction will resolve to `INVALID_TOKEN_ID`.
/// - If the provided token has been deleted, the transaction will resolve to `TOKEN_WAS_DELETED`.
/// - If no Pause Key is defined, the transaction will resolve to `TOKEN_HAS_NO_PAUSE_KEY`.
pub type TokenPauseTransaction = Transaction<TokenPauseTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct TokenPauseTransactionData {
    /// The token to be paused.
    token_id: Option<TokenId>,
}

impl TokenPauseTransaction {
    /// Returns the token to be paused.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token to be paused.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenPauseTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).pause_token(request).await
    }
}

impl ValidateChecksums for TokenPauseTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenPauseTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token = self.token_id.to_protobuf();

        services::transaction_body::Data::TokenPause(services::TokenPauseTransactionBody { token })
    }
}

impl From<TokenPauseTransactionData> for AnyTransactionData {
    fn from(transaction: TokenPauseTransactionData) -> Self {
        Self::TokenPause(transaction)
    }
}

impl FromProtobuf<services::TokenPauseTransactionBody> for TokenPauseTransactionData {
    fn from_protobuf(pb: services::TokenPauseTransactionBody) -> crate::Result<Self> {
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
            TokenId,
            TokenPauseTransaction,
        };

        // language=JSON
        const TOKEN_PAUSE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenPause",
  "tokenId": "0.0.1001"
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TokenPauseTransaction::new();

            transaction.token_id(TokenId::from(1001));

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_PAUSE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOKEN_PAUSE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TokenPause(transaction) => transaction);

            assert_eq!(data.token_id.unwrap(), TokenId::from(1001));

            Ok(())
        }
    }
}
