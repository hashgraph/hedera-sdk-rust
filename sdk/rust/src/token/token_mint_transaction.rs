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

/// Mints tokens to the Token's treasury Account.
///
/// The operation increases the Total Supply of the Token. The maximum total supply a token can have
/// is 2^63-1.
///
/// The amount provided must be in the lowest denomination possible. Example: Token A has 2 decimals.
/// In order to mint 100 tokens, one must provide amount of 10000. In order to mint 100.55 tokens,
/// one must provide amount of 10055.
///
/// - If no Supply Key is defined, the transaction will resolve to `TokenHasNoSupplyKey`.
/// - If both amount and metadata list get filled, a `InvalidTransactionBody` response code will be
/// returned.
/// - If the metadata list contains metadata which is too large, a `MetadataTooLong` response code will
/// be returned.
/// - If neither the amount nor the metadata list get filled, a `InvalidTokenMintAmount` response code
/// will be returned.
/// - If the metadata list count is greater than the batch size limit global dynamic property, a
/// `BatchSizeLimitExceeded` response code will be returned.
pub type TokenMintTransaction = Transaction<TokenMintTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct TokenMintTransactionData {
    /// The token for which to mint tokens.
    token_id: Option<TokenId>,

    /// The amount of a fungible token to mint to the treasury account.
    amount: u64,

    /// The list of metadata for a non-fungible token to mint to the treasury account.
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<Vec<serde_with::base64::Base64>>"))]
    metadata: Vec<Vec<u8>>,
}

impl TokenMintTransaction {
    /// Returns the token for which to mint tokens.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token for which to mint tokens.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }

    /// Returns the amount of a fungible token to mint to the treasury account.
    #[must_use]
    pub fn get_amount(&self) -> u64 {
        self.data().amount
    }

    /// Sets the amount of a fungible token to mint to the treasury account.
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.data_mut().amount = amount;
        self
    }

    /// Returns the list of metadata for a non-fungible token to mint to the treasury account.
    #[must_use]
    pub fn get_metadata(&self) -> &[Vec<u8>] {
        &self.data().metadata
    }

    /// Sets the list of metadata for a non-fungible token to mint to the treasury account.
    pub fn metadata<Bytes>(&mut self, metadata: impl IntoIterator<Item = Bytes>) -> &mut Self
    where
        Bytes: AsRef<[u8]>,
    {
        self.data_mut().metadata =
            metadata.into_iter().map(|bytes| bytes.as_ref().to_vec()).collect();

        self
    }
}

#[async_trait]
impl TransactionExecute for TokenMintTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).mint_token(request).await
    }
}

impl ValidateChecksums for TokenMintTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenMintTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token = self.token_id.to_protobuf();
        let amount = self.amount;
        let metadata = self.metadata.clone();

        services::transaction_body::Data::TokenMint(services::TokenMintTransactionBody {
            token,
            amount,
            metadata,
        })
    }
}

impl From<TokenMintTransactionData> for AnyTransactionData {
    fn from(transaction: TokenMintTransactionData) -> Self {
        Self::TokenMint(transaction)
    }
}

impl FromProtobuf<services::TokenMintTransactionBody> for TokenMintTransactionData {
    fn from_protobuf(pb: services::TokenMintTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            token_id: Option::from_protobuf(pb.token)?,
            amount: pb.amount,
            metadata: pb.metadata,
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
            TokenId,
            TokenMintTransaction,
        };

        // language=JSON
        const TOKEN_MINT_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenMint",
  "tokenId": "0.0.1981",
  "amount": 8675309,
  "metadata": [
    "SmVubnkgSSd2ZSBnb3QgeW91ciBudW1iZXI="
  ]
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TokenMintTransaction::new();

            transaction
                .token_id(TokenId::from(1981))
                .amount(8675309)
                .metadata(["Jenny I've got your number"]);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_MINT_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOKEN_MINT_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TokenMint(transaction) => transaction);

            assert_eq!(data.token_id.unwrap(), TokenId::from(1981));
            assert_eq!(data.amount, 8675309);

            let bytes: Vec<u8> = "Jenny I've got your number".into();
            assert_eq!(data.metadata, [bytes]);

            Ok(())
        }
    }
}
