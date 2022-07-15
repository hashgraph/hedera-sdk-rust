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
    AccountAddress,
    AccountId,
    TokenId,
    Transaction,
    TransactionId,
};

/// Revokes KYC from the account for the given token.
///
/// Must be signed by the Token's kycKey.
///
/// Once executed the Account is marked as KYC Revoked.
///
/// - If the provided account is not found, the transaction will resolve to INVALID_ACCOUNT_ID.
/// - If the provided account has been deleted, the transaction will resolve to ACCOUNT_DELETED.
/// - If the provided token is not found, the transaction will resolve to INVALID_TOKEN_ID.
/// - If the provided token has been deleted, the transaction will resolve to TOKEN_WAS_DELETED.
/// - If an Association between the provided token and account is not found, the transaction will
/// resolve to TOKEN_NOT_ASSOCIATED_TO_ACCOUNT.
/// - If no KYC Key is defined, the transaction will resolve to TOKEN_HAS_NO_KYC_KEY.
pub type TokenRevokeKycTransaction = Transaction<TokenRevokeKycTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRevokeKycTransactionData {
    /// The account to have their KYC revoked.
    account_id: Option<AccountAddress>,

    /// The token for which this account will have their KYC revoked.
    token_id: Option<TokenId>,
}

impl TokenRevokeKycTransaction {
    /// Sets the account to have their KYC revoked.
    pub fn account_id(&mut self, account_id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.account_id = Some(account_id.into());
        self
    }

    /// Sets the token for which this account will have their KYC revoked.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenRevokeKycTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).revoke_kyc_from_token_account(request).await
    }
}

impl ToTransactionDataProtobuf for TokenRevokeKycTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.as_ref().map(AccountAddress::to_protobuf);
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);

        services::transaction_body::Data::TokenRevokeKyc(services::TokenRevokeKycTransactionBody {
            account,
            token,
        })
    }
}

impl From<TokenRevokeKycTransactionData> for AnyTransactionData {
    fn from(transaction: TokenRevokeKycTransactionData) -> Self {
        Self::TokenRevokeKyc(transaction)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use crate::{TokenId, TokenRevokeKycTransaction, AccountId, AccountAddress};
    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // language=JSON
    const TOKEN_REVOKE_KYC_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenRevokeKyc",
  "accountId": "0.0.1001",
  "tokenId": "0.0.1002"
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = TokenRevokeKycTransaction::new();

        transaction
            .account_id(AccountId::from(1001))
            .token_id(TokenId::from(1002));

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, TOKEN_REVOKE_KYC_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(TOKEN_REVOKE_KYC_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::TokenRevokeKyc(transaction) => transaction);

        assert_eq!(data.token_id.unwrap(), TokenId::from(1002));

        let account_id = assert_matches!(data.account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(account_id, AccountId::from(1001));

        Ok(())
    }
}
