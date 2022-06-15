use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use itertools::Itertools;
use serde_with::{serde_as, skip_serializing_none};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::token::custom_fees::CustomFee;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, TokenId, Transaction, TransactionId};

/// At consensus, updates a token type's fee schedule to the given list of custom fees.
///
/// If the target token type has no `fee_schedule_key`, resolves to `TokenHasNoFeeScheduleKey`.
/// Otherwise this transaction must be signed to the `fee_schedule_key`, or the transaction will
/// resolve to `InvalidSignature`.
///
/// If the `custom_fees` list is empty, clears the fee schedule or resolves to
/// `CustomScheduleAlreadyHasNoFees` if the fee schedule was already empty.
pub type TokenFeeScheduleUpdateTransaction = Transaction<TokenFeeScheduleUpdateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenFeeScheduleUpdateTransactionData {
    /// The token whose fee schedule is to be updated
    token_id: Option<TokenId>,

    /// The new custom fees to be assessed during a transfer
    custom_fees: Vec<CustomFee>,
}

impl TokenFeeScheduleUpdateTransaction {
    /// Sets the token whose fee schedule is to be updated
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }

    /// Sets the new custom fees to be assessed during a transfer
    pub fn custom_fees(&mut self, custom_fees: impl IntoIterator<Item = CustomFee>) -> &mut Self {
        self.body.data.custom_fees = custom_fees.into_iter().collect();
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

impl ToTransactionDataProtobuf for TokenFeeScheduleUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token_id = self.token_id.as_ref().map(TokenId::to_protobuf);
        let custom_fees = self.custom_fees.iter().map(CustomFee::to_protobuf).collect_vec();

        services::transaction_body::Data::TokenFeeScheduleUpdate(services::TokenFeeScheduleUpdateTransactionBody {
            token_id,
            custom_fees
        })
    }
}

impl From<TokenFeeScheduleUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenFeeScheduleUpdateTransactionData) -> Self {
        Self::TokenFeeScheduleUpdate(transaction)
    }
}
