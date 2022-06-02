use async_trait::async_trait;
use itertools::Itertools;
use tonic::transport::Channel;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::base64::Base64;
use serde_with::{serde_as, skip_serializing_none, TimestampNanoSeconds};

use crate::{AccountId, TokenId, ToProtobuf, Transaction, TransactionId};
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};

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
pub type TokenAssociateTransaction = Transaction<TokenAssociateTransaction>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAssociateTransactionData {
    /// The account to be associated with the provided tokens.
    account_id: Option<AccountId>,

    /// The tokens to be associated with the provided account. In the case of NON_FUNGIBLE_UNIQUE
    /// Type, once an account is associated, it can hold any number of NFTs (serial numbers) of that
    /// account type.
    tokens: Option<Vec<TokenId>>,
}

impl TokenAssociateTransaction {
    /// Sets the account to be associated with the provided tokens.
    pub fn account_id(&mut self, account_id: impl Into<AccountId>) -> &mut Self {
        self.body.data.account_id = Some(account_id.into());
        self
    }

    /// Sets the tokens to be associated with the provided account.
    pub fn tokens(&mut self, tokens: impl Into<Vec<Token>>) -> &mut Self {
        self.body.data.tokens = Some(tokens);
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenAssociateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction
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
        let account_id = self.account_id.as_ref().map(AccountId::to_protobuf);
        let tokens =
            self.tokens
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(TokenId::to_protobuf)
                .collect_vec();

        services::transaction_body::Data::TokenAssociate(services::TokenAssociateTransactionBody {
            account_id,
            tokens: Some(tokens),
        })
    }
}

impl From<TokenAssociateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenAssociateTransactionData) -> Self {
        Self::TokenAssociate(transaction)
    }
}
