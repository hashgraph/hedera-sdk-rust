use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{
    serde_as,
    skip_serializing_none,
};
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

/// Unpauses the Token. Must be signed with the Token's pause key.
///
/// Once executed the Token is marked as Unpaused and can be used in Transactions.
///
/// The operation is idempotent - becomes a no-op if the Token is already unpaused.
///
/// - If the provided token is not found, the transaction will resolve to INVALID_TOKEN_ID.
/// - If the provided token has been deleted, the transaction will resolve to TOKEN_WAS_DELETED.
/// - If no Pause Key is defined, the transaction will resolve to TOKEN_HAS_NO_PAUSE_KEY.
pub type TokenUnpauseTransaction = Transaction<TokenUnpauseTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenUnpauseTransactionData {
    /// The token to be unpaused.
    token_id: Option<TokenId>,
}

impl TokenUnpauseTransaction {
    /// Sets the token to be unpaused.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenUnpauseTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).unpause_token(request).await
    }
}

impl ToTransactionDataProtobuf for TokenUnpauseTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);

        services::transaction_body::Data::TokenUnpause(services::TokenUnpauseTransactionBody {
            token,
        })
    }
}

impl From<TokenUnpauseTransactionData> for AnyTransactionData {
    fn from(transaction: TokenUnpauseTransactionData) -> Self {
        Self::TokenUnpause(transaction)
    }
}
