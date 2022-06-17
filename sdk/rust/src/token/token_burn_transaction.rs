use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{serde_as, skip_serializing_none};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, TokenId, Transaction, TransactionId};

/// Burns tokens from the Token's treasury Account.
///
/// The operation decreases the Total Supply of the Token. Total supply cannot go below zero.
///
/// The amount provided must be in the lowest denomination possible. Example:
/// Token A has 2 decimals. In order to burn 100 tokens, one must provide amount of 10000. In order
/// to burn 100.55 tokens, one must provide amount of 10055.
///
/// For non-fungible tokens the transaction body accepts serialNumbers list of integers as a parameter.
///
/// - If no Supply Key is defined, the transaction will resolve to TOKEN_HAS_NO_SUPPLY_KEY.
///
/// - If neither the amount nor the serialNumbers get filled, a INVALID_TOKEN_BURN_AMOUNT response code
/// will be returned.
///
/// - If both amount and serialNumbers get filled, a INVALID_TRANSACTION_BODY response code will be
/// returned.
///
/// - If the serialNumbers' list count is greater than the batch size limit global dynamic property, a
/// BATCH_SIZE_LIMIT_EXCEEDED response code will be returned.
///
/// - If the serialNumbers list contains a non-positive integer as a serial number, a INVALID_NFT_ID
/// response code will be returned.
pub type TokenBurnTransaction = Transaction<TokenBurnTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBurnTransactionData {
    /// The token for which to burn tokens.
    token_id: Option<TokenId>,

    /// The amount of a fungible token to burn from the treasury account.
    amount: u64,

    /// The serial numbers of a non-fungible token to burn from the treasury account.
    serial_numbers: Vec<i64>,
}

impl TokenBurnTransaction {
    /// Sets the token for which to burn tokens.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }

    /// Sets the amount of a fungible token to burn from the treasury account.
    pub fn amount(&mut self, amount: impl Into<u64>) -> &mut Self {
        self.body.data.amount = amount.into();
        self
    }

    /// Sets the serial numbers of a non-fungible token to burn from the treasury account.
    pub fn serial_numbers(&mut self, serial_numbers: impl IntoIterator<Item = i64>) -> &mut Self {
        self.body.data.serial_numbers = serial_numbers.into_iter().collect();
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

impl ToTransactionDataProtobuf for TokenBurnTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);
        let amount = self.amount;
        let serial_numbers = self.serial_numbers.clone();

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
