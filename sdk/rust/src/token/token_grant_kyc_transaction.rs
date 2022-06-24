use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, TokenId, Transaction, TransactionId};

/// Grants KYC to the account for the given token. Must be signed by the Token's kycKey.
///
/// Once executed the Account is marked as KYC Granted.
///
/// - If the provided account is not found, the transaction will resolve to INVALID_ACCOUNT_ID.
/// - If the provided account has been deleted, the transaction will resolve to ACCOUNT_DELETED.
/// - If the provided token is not found, the transaction will resolve to INVALID_TOKEN_ID.
/// - If the provided token has been deleted, the transaction will resolve to TOKEN_WAS_DELETED.
/// - If an Association between the provided token and account is not found, the transaction will
/// resolve to TOKEN_NOT_ASSOCIATED_TO_ACCOUNT.
/// - If no KYC Key is defined, the transaction will resolve to TOKEN_HAS_NO_KYC_KEY.
pub type TokenGrantKycTransaction = Transaction<TokenGrantKycTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenGrantKycTransactionData {
    /// The account to be granted KYC.
    account_id: Option<AccountAddress>,

    /// The token for which this account will be granted KYC.
    token_id: Option<TokenId>,
}

impl TokenGrantKycTransaction {
    /// Sets the account to be granted KYC.
    pub fn account_id(&mut self, account_id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.account_id = Some(account_id.into());
        self
    }

    /// Sets the token for which this account will be granted KYC.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenGrantKycTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).grant_kyc_to_token_account(request).await
    }
}

impl ToTransactionDataProtobuf for TokenGrantKycTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.as_ref().map(AccountAddress::to_protobuf);
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);

        services::transaction_body::Data::TokenGrantKyc(services::TokenGrantKycTransactionBody {
            account,
            token,
        })
    }
}

impl From<TokenGrantKycTransactionData> for AnyTransactionData {
    fn from(transaction: TokenGrantKycTransactionData) -> Self {
        Self::TokenGrantKyc(transaction)
    }
}
