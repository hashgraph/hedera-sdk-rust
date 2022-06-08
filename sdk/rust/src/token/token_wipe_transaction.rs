use async_trait::async_trait;
use tonic::transport::Channel;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{serde_as, skip_serializing_none};

use crate::protobuf::ToProtobuf;
use crate::{AccountId, TokenId, Transaction, TransactionId};
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};

/// Wipes the provided amount of tokens from the specified Account. Must be signed by the Token's
/// Wipe key.
///
/// On success, tokens are removed from the account and the total supply of the token is decreased by
/// the wiped amount.
///
/// The amount provided is in the lowest denomination possible. Example:
/// Token A has 2 decimals. In order to wipe 100 tokens from account, one must provide amount of 10000.
/// In order to wipe 100.55 tokens, one must provide amount of 10055.
///
/// - If the provided account is not found, the transaction will resolve to INVALID_ACCOUNT_ID.
/// - If the provided account has been deleted, the transaction will resolve to ACCOUNT_DELETED.
/// - If the provided token is not found, the transaction will resolve to INVALID_TOKEN_ID.
/// - If the provided token has been deleted, the transaction will resolve to TOKEN_WAS_DELETED.
/// - If an Association between the provided token and account is not found, the transaction will
/// resolve to TOKEN_NOT_ASSOCIATED_TO_ACCOUNT.
/// - If Wipe Key is not present in the Token, transaction results in TOKEN_HAS_NO_WIPE_KEY.
/// - If the provided account is the Token's Treasury Account, transaction results in
/// CANNOT_WIPE_TOKEN_TREASURY_ACCOUNT
/// - If both amount and serialNumbers get filled, a INVALID_TRANSACTION_BODY response code will be
/// returned.
/// - If neither the amount nor the serialNumbers get filled, a INVALID_WIPING_AMOUNT response code
/// will be returned.
/// - If the serialNumbers list contains a non-positive integer as a serial number, a INVALID_NFT_ID
/// response code will be returned.
/// - If the serialNumbers' list count is greater than the batch size limit global dynamic property, a
/// BATCH_SIZE_LIMIT_EXCEEDED response code will be returned.
///
pub type TokenWipeTransaction = Transaction<TokenWipeTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenWipeTransactionData {
    /// The account to be wiped.
    account_id: Option<AccountId>,

    /// The token for which the account will be wiped.
    token_id: Option<TokenId>,

    /// Applicable to tokens of type FUNGIBLE_COMMON. The amount of tokens to wipe from the specified
    /// account. Amount must be a positive non-zero number in the lowest denomination possible, not
    /// bigger than the token balance of the account (0; balance].
    amount: Option<u64>,

    /// Applicable to tokens of type NON_FUNGIBLE_UNIQUE. The list of serial numbers to be wiped.
    serial_numbers: Vec<i64>,
}

impl TokenWipeTransaction {
    /// Sets the account to be wiped.
    pub fn account_id(&mut self, account_id: impl Into<AccountId>) -> &mut Self {
        self.body.data.account_id = Some(account_id.into());
        self
    }

    /// Sets the token for which the account will be wiped.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }

    /// Applicable to tokens of type FUNGIBLE_COMMON. Sets the amount of tokens to wipe from the specified
    /// account. Amount must be a positive non-zero number in the lowest denomination possible, not
    /// bigger than the token balance of the account (0; balance].
    pub fn amount(&mut self, amount: impl Into<u64>) -> &mut Self {
        self.body.data.amount = Some(amount.into());
        self
    }

    /// Applicable to tokens of type NON_FUNGIBLE_UNIQUE. Sets the list of serial numbers to be wiped.
    pub fn serial_numbers(&mut self, serial_numbers: impl IntoIterator<Item = i64>) -> &mut Self {
        self.body.data.serial_numbers = serial_numbers.into_iter().collect();
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenWipeTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).wipe_token_account(request).await
    }
}

impl ToTransactionDataProtobuf for TokenWipeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.as_ref().map(AccountId::to_protobuf);
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);
        let amount = self.amount.clone().unwrap_or_default();
        let serial_numbers = self.serial_numbers.clone();

        services::transaction_body::Data::TokenWipe(services::TokenWipeAccountTransactionBody {
            account,
            token,
            amount,
            serial_numbers,
        })
    }
}

impl From<TokenWipeTransactionData> for AnyTransactionData {
    fn from(transaction: TokenWipeTransactionData) -> Self {
        Self::TokenWipe(transaction)
    }
}
