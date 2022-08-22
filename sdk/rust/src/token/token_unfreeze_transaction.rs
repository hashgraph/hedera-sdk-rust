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
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    TokenId,
    Transaction,
    TransactionId,
};

/// Unfreezes transfers of the specified token for the account. Must be signed by the Token's freezeKey.
///
/// Once executed the Account is marked as Unfrozen and will be able to receive or send tokens.
/// The operation is idempotent.
///
/// - If the provided account is not found, the transaction will resolve to `INVALID_ACCOUNT_ID`.
/// - If the provided account has been deleted, the transaction will resolve to `ACCOUNT_DELETED`.
/// - If the provided token is not found, the transaction will resolve to `INVALID_TOKEN_ID`.
/// - If the provided token has been deleted, the transaction will resolve to `TOKEN_WAS_DELETED`.
/// - If an Association between the provided token and account is not found, the transaction will
/// resolve to `TOKEN_NOT_ASSOCIATED_TO_ACCOUNT`.
/// - If no Freeze Key is defined, the transaction will resolve to `TOKEN_HAS_NO_FREEZE_KEY`.
pub type TokenUnfreezeTransaction = Transaction<TokenUnfreezeTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenUnfreezeTransactionData {
    /// The account to be unfrozen.
    account_id: Option<AccountId>,

    /// The token for which this account will be unfrozen.
    token_id: Option<TokenId>,
}

impl TokenUnfreezeTransaction {
    /// Sets the account to be unfrozen.
    pub fn account_id(&mut self, account_id: AccountId) -> &mut Self {
        self.body.data.account_id = Some(account_id);
        self
    }

    /// Sets the token for which this account will be unfrozen.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenUnfreezeTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).unfreeze_token_account(request).await
    }
}

impl ToTransactionDataProtobuf for TokenUnfreezeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.as_ref().map(AccountId::to_protobuf);
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);

        services::transaction_body::Data::TokenUnfreeze(
            services::TokenUnfreezeAccountTransactionBody { token, account },
        )
    }
}

impl From<TokenUnfreezeTransactionData> for AnyTransactionData {
    fn from(transaction: TokenUnfreezeTransactionData) -> Self {
        Self::TokenUnfreeze(transaction)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::transaction::{
        AnyTransaction,
        AnyTransactionData,
    };
    use crate::{
        AccountId,
        TokenId,
        TokenUnfreezeTransaction,
    };

    // language=JSON
    const TOKEN_UNFREEZE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenUnfreeze",
  "accountId": "0.0.1001",
  "tokenId": "0.0.1002"
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = TokenUnfreezeTransaction::new();

        transaction.account_id(AccountId::from(1001)).token_id(TokenId::from(1002));

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, TOKEN_UNFREEZE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(TOKEN_UNFREEZE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::TokenUnfreeze(transaction) => transaction);

        assert_eq!(data.token_id.unwrap(), TokenId::from(1002));
        assert_eq!(data.account_id, Some(AccountId::from(1001)));

        Ok(())
    }
}
