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

use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    LedgerId,
    ToProtobuf,
    TokenId,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

/// Associates the provided account with the provided tokens. Must be signed by the provided Account's key.
///
/// - If the provided account is not found, the transaction will resolve to `INVALID_ACCOUNT_ID`.
/// - If the provided account has been deleted, the transaction will resolve to `ACCOUNT_DELETED`.
/// - If any of the provided tokens are not found, the transaction will resolve to `INVALID_TOKEN_REF`.
/// - If any of the provided tokens have been deleted, the transaction will resolve to
/// `TOKEN_WAS_DELETED`.
/// - If an association between the provided account and any of the tokens already exists, the
/// transaction will resolve to `TOKEN_ALREADY_ASSOCIATED_TO_ACCOUNT`.
/// - If the provided account's associations count exceed the constraint of maximum token associations
/// per account, the transaction will resolve to `TOKENS_PER_ACCOUNT_LIMIT_EXCEEDED`.
/// - On success, associations between the provided account and tokens are made and the account is
/// ready to interact with the tokens.
pub type TokenAssociateTransaction = Transaction<TokenAssociateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct TokenAssociateTransactionData {
    /// The account to be associated with the provided tokens.
    account_id: Option<AccountId>,

    /// The tokens to be associated with the provided account.
    token_ids: Vec<TokenId>,
}

impl TokenAssociateTransaction {
    /// Returns the account to be associated with the provided tokens.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data().account_id
    }

    /// Sets the account to be associated with the provided tokens.
    pub fn account_id(&mut self, account_id: AccountId) -> &mut Self {
        self.data_mut().account_id = Some(account_id);
        self
    }

    /// Returns the tokens to be associated with the provided account.
    #[must_use]
    pub fn get_token_ids(&self) -> &[TokenId] {
        &self.data().token_ids
    }

    /// Sets the tokens to be associated with the provided account.
    pub fn token_ids(&mut self, token_ids: impl IntoIterator<Item = TokenId>) -> &mut Self {
        self.data_mut().token_ids = token_ids.into_iter().collect();
        self
    }
}

impl TransactionExecute for TokenAssociateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).associate_tokens(request).await })
    }
}

impl ValidateChecksums for TokenAssociateTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.account_id.validate_checksums(ledger_id)?;
        for token_id in &self.token_ids {
            token_id.validate_checksums(ledger_id)?;
        }
        Ok(())
    }
}

impl ToTransactionDataProtobuf for TokenAssociateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.to_protobuf();
        let tokens = self.token_ids.to_protobuf();

        services::transaction_body::Data::TokenAssociate(services::TokenAssociateTransactionBody {
            account,
            tokens,
        })
    }
}

impl From<TokenAssociateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenAssociateTransactionData) -> Self {
        Self::TokenAssociate(transaction)
    }
}

impl FromProtobuf<services::TokenAssociateTransactionBody> for TokenAssociateTransactionData {
    fn from_protobuf(pb: services::TokenAssociateTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            account_id: Option::from_protobuf(pb.account)?,
            token_ids: Vec::from_protobuf(pb.tokens)?,
        })
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
            AccountId,
            TokenAssociateTransaction,
            TokenId,
        };

        // language=JSON
        const TOKEN_ASSOCIATE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenAssociate",
  "accountId": "0.0.1001",
  "tokenIds": [
    "0.0.1002"
  ]
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TokenAssociateTransaction::new();

            transaction.account_id(AccountId::from(1001)).token_ids([TokenId::from(1002)]);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_ASSOCIATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction =
                serde_json::from_str(TOKEN_ASSOCIATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TokenAssociate(transaction) => transaction);

            assert_eq!(data.token_ids[0], TokenId::from(1002));
            assert_eq!(data.account_id, Some(AccountId::from(1001)));

            Ok(())
        }
    }
}
