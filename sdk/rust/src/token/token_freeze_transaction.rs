use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{skip_serializing_none};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, TokenId, Transaction, TransactionId};

/// Freezes transfers of the specified token for the account. Must be signed by the Token's freezeKey.
///
/// Once executed the Account is marked as Frozen and will not be able to receive or send tokens
/// unless unfrozen. The operation is idempotent.
///
/// - If the provided account is not found, the transaction will resolve to INVALID_ACCOUNT_ID.
/// - If the provided account has been deleted, the transaction will resolve to ACCOUNT_DELETED.
/// - If the provided token is not found, the transaction will resolve to INVALID_TOKEN_ID.
/// - If the provided token has been deleted, the transaction will resolve to TOKEN_WAS_DELETED.
/// - If an Association between the provided token and account is not found, the transaction will
/// resolve to TOKEN_NOT_ASSOCIATED_TO_ACCOUNT.
/// - If no Freeze Key is defined, the transaction will resolve to TOKEN_HAS_NO_FREEZE_KEY.
pub type TokenFreezeTransaction = Transaction<TokenFreezeTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenFreezeTransactionData {
    /// The account to be frozen.
    account_id: Option<AccountAddress>,

    /// The token for which this account will be frozen.
    token_id: Option<TokenId>,
}

impl TokenFreezeTransaction {
    /// Sets the account to be frozen.
    pub fn account_id(&mut self, account_id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.account_id = Some(account_id.into());
        self
    }

    /// Sets the token for which this account will be frozen.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenFreezeTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).freeze_token_account(request).await
    }
}

impl ToTransactionDataProtobuf for TokenFreezeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.as_ref().map(AccountAddress::to_protobuf);
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);

        services::transaction_body::Data::TokenFreeze(services::TokenFreezeAccountTransactionBody {
            account,
            token,
        })
    }
}

impl From<TokenFreezeTransactionData> for AnyTransactionData {
    fn from(transaction: TokenFreezeTransactionData) -> Self {
        Self::TokenFreeze(transaction)
    }
}
