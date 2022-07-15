use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use itertools::Itertools;
use serde_with::{
    serde_as,
    skip_serializing_none,
};
use tonic::transport::Channel;

use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountAddress,
    AccountId,
    ToProtobuf,
    TokenId,
    Transaction,
    TransactionId,
};

/// Associates the provided account with the provided tokens. Must be signed by the provided Account's key.
///
/// - If the provided account is not found, the transaction will resolve to INVALID_ACCOUNT_ID.
/// - If the provided account has been deleted, the transaction will resolve to ACCOUNT_DELETED.
/// - If any of the provided tokens are not found, the transaction will resolve to INVALID_TOKEN_REF.
/// - If any of the provided tokens have been deleted, the transaction will resolve to
/// TOKEN_WAS_DELETED.
/// - If an association between the provided account and any of the tokens already exists, the
/// transaction will resolve to TOKEN_ALREADY_ASSOCIATED_TO_ACCOUNT.
/// - If the provided account's associations count exceed the constraint of maximum token associations
/// per account, the transaction will resolve to TOKENS_PER_ACCOUNT_LIMIT_EXCEEDED.
/// - On success, associations between the provided account and tokens are made and the account is
/// ready to interact with the tokens.
pub type TokenAssociateTransaction = Transaction<TokenAssociateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAssociateTransactionData {
    /// The account to be associated with the provided tokens.
    account_id: Option<AccountAddress>,

    /// The tokens to be associated with the provided account.
    token_ids: Vec<TokenId>,
}

impl TokenAssociateTransaction {
    /// Sets the account to be associated with the provided tokens.
    pub fn account_id(&mut self, account_id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.account_id = Some(account_id.into());
        self
    }

    /// Sets the tokens to be associated with the provided account.
    pub fn token_ids(&mut self, token_ids: impl IntoIterator<Item = TokenId>) -> &mut Self {
        self.body.data.token_ids = token_ids.into_iter().collect();
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenAssociateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).associate_tokens(request).await
    }
}

impl ToTransactionDataProtobuf for TokenAssociateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.as_ref().map(AccountAddress::to_protobuf);
        let tokens = self.token_ids.iter().map(TokenId::to_protobuf).collect_vec();

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

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::transaction::{
        AnyTransaction,
        AnyTransactionData,
    };
    use crate::{
        AccountAddress,
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
        let transaction: AnyTransaction = serde_json::from_str(TOKEN_ASSOCIATE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::TokenAssociate(transaction) => transaction);

        assert_eq!(data.token_ids[0], TokenId::from(1002));

        let account_id = assert_matches!(data.account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(account_id, AccountId::from(1001));

        Ok(())
    }
}
