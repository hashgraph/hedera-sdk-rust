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
use crate::token::custom_fees::AnyCustomFee;
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

/// At consensus, updates a token type's fee schedule to the given list of custom fees.
///
/// If the target token type has no `fee_schedule_key`, resolves to `TokenHasNoFeeScheduleKey`.
/// Otherwise this transaction must be signed to the `fee_schedule_key`, or the transaction will
/// resolve to `InvalidSignature`.
///
/// If the `custom_fees` list is empty, clears the fee schedule or resolves to
/// `CustomScheduleAlreadyHasNoFees` if the fee schedule was already empty.
pub type TokenFeeScheduleUpdateTransaction = Transaction<TokenFeeScheduleUpdateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
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

#[async_trait]
impl TransactionExecute for TokenFeeScheduleUpdateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).update_token_fee_schedule(request).await
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
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token_id = self.token_id.to_protobuf();
        let custom_fees = self.custom_fees.to_protobuf();

        services::transaction_body::Data::TokenFeeScheduleUpdate(
            services::TokenFeeScheduleUpdateTransactionBody { token_id, custom_fees },
        )
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use assert_matches::assert_matches;

        use crate::token::custom_fees::{
            CustomFee,
            FixedFeeData,
        };
        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            AccountId,
            TokenFeeScheduleUpdateTransaction,
            TokenId,
        };

        // language=JSON
        const TOKEN_FEE_SCHEDULE_UPDATE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenFeeScheduleUpdate",
  "tokenId": "0.0.1001",
  "customFees": [
    {
      "$type": "fixed",
      "amount": 1,
      "denominatingTokenId": "0.0.7",
      "feeCollectorAccountId": "0.0.8",
      "allCollectorsAreExempt": false
    }
  ]
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TokenFeeScheduleUpdateTransaction::new();

            transaction.token_id(TokenId::from(1001)).custom_fees([CustomFee {
                fee: FixedFeeData { amount: 1, denominating_token_id: TokenId::from(7) }.into(),
                fee_collector_account_id: Some(AccountId::from(8)),
                all_collectors_are_exempt: false,
            }]);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_FEE_SCHEDULE_UPDATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction =
                serde_json::from_str(TOKEN_FEE_SCHEDULE_UPDATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TokenFeeScheduleUpdate(transaction) => transaction);

            assert_eq!(data.token_id.unwrap(), TokenId::from(1001));
            assert_eq!(
                data.custom_fees,
                [CustomFee {
                    fee: FixedFeeData { amount: 1, denominating_token_id: TokenId::from(7) }.into(),
                    fee_collector_account_id: Some(AccountId::from(8)),
                    all_collectors_are_exempt: false,
                }]
            );

            Ok(())
        }
    }
}
