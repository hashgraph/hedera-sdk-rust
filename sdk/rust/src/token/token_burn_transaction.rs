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

/// Burns tokens from the Token's treasury Account.
///
/// The operation decreases the Total Supply of the Token. Total supply cannot go below zero.
///
/// The amount provided must be in the lowest denomination possible. Example:
/// Token A has 2 decimals. In order to burn 100 tokens, one must provide amount of 10000. In order
/// to burn 100.55 tokens, one must provide amount of 10055.
///
/// For non-fungible tokens the transaction body accepts a `serials` list of integers as a parameter.
///
/// - If no Supply Key is defined, the transaction will resolve to `TOKEN_HAS_NO_SUPPLY_KEY`.
///
/// - If neither the amount nor the `serials` get filled, a `INVALID_TOKEN_BURN_AMOUNT` response code
/// will be returned.
///
/// - If both amount and `serials` get filled, a `INVALID_TRANSACTION_BODY` response code will be
/// returned.
///
/// - If the `serials` list count is greater than the batch size limit global dynamic property, a
/// `BATCH_SIZE_LIMIT_EXCEEDED` response code will be returned.
///
/// - If the `serials` list contains a non-positive integer as a serial number, a `INVALID_NFT_ID`
/// response code will be returned.
pub type TokenBurnTransaction = Transaction<TokenBurnTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct TokenBurnTransactionData {
    /// The token for which to burn tokens.
    token_id: Option<TokenId>,

    /// The amount of a fungible token to burn from the treasury account.
    amount: u64,

    /// The serial numbers of a non-fungible token to burn from the treasury account.
    serials: Vec<i64>,
}

impl TokenBurnTransaction {
    /// Returns the token for which to burn tokens.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token for which to burn tokens.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }

    /// Returns the amount of a fungible token to burn from the treasury account.
    #[must_use]
    pub fn get_amount(&self) -> u64 {
        self.data().amount
    }

    /// Sets the amount of a fungible token to burn from the treasury account.
    pub fn amount(&mut self, amount: impl Into<u64>) -> &mut Self {
        self.data_mut().amount = amount.into();
        self
    }

    /// Returns the serial numbers of a non-fungible token to burn from the treasury account.
    #[must_use]
    pub fn get_serials(&self) -> &[i64] {
        &self.data().serials
    }

    /// Sets the serial numbers of a non-fungible token to burn from the treasury account.
    pub fn serials(&mut self, serials: impl IntoIterator<Item = i64>) -> &mut Self {
        self.data_mut().serials = serials.into_iter().collect();
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenBurnTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).burn_token(request).await
    }
}

impl ValidateChecksums for TokenBurnTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenBurnTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token = self.token_id.to_protobuf();
        let amount = self.amount;
        let serial_numbers = self.serials.clone();

        services::transaction_body::Data::TokenBurn(services::TokenBurnTransactionBody {
            token,
            amount,
            serial_numbers,
        })
    }
}

impl From<TokenBurnTransactionData> for AnyTransactionData {
    fn from(transaction: TokenBurnTransactionData) -> Self {
        Self::TokenBurn(transaction)
    }
}

impl FromProtobuf<services::TokenBurnTransactionBody> for TokenBurnTransactionData {
    fn from_protobuf(pb: services::TokenBurnTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            token_id: Option::from_protobuf(pb.token)?,
            amount: pb.amount,
            serials: pb.serial_numbers,
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
            TokenBurnTransaction,
            TokenId,
        };

        // language=JSON
        const TOKEN_BURN_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenBurn",
  "tokenId": "0.0.1002",
  "amount": 100,
  "serials": [
    1,
    2,
    3
  ]
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TokenBurnTransaction::new();

            transaction.token_id(TokenId::from(1002)).amount(100u64).serials([1, 2, 3]);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_BURN_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOKEN_BURN_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TokenBurn(transaction) => transaction);

            assert_eq!(data.token_id, Some(TokenId::from(1002)));
            assert_eq!(data.amount, 100);
            assert_eq!(data.serials, vec![1, 2, 3]);

            Ok(())
        }
    }
}
