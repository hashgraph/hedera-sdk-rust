use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{serde_as, skip_serializing_none};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, TokenId, Transaction, TransactionId};

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
    account_id: Option<AccountAddress>,

    /// The token for which the account will be wiped.
    token_id: Option<TokenId>,

    // TODO change type of `amount` from `Option<u64>` to `u64`
    /// The amount of a fungible token to wipe from the specified account.
    amount: Option<u64>,

    /// The serial numbers of a non-fungible token to wipe from the specified account.
    serial_numbers: Vec<u64>,
}

impl TokenWipeTransaction {
    /// Sets the account to be wiped.
    pub fn account_id(&mut self, account_id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.account_id = Some(account_id.into());
        self
    }

    /// Sets the token for which the account will be wiped.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }

    // TODO remove `impl Into<_>`
    /// Sets the amount of a fungible token to wipe from the specified account.
    pub fn amount(&mut self, amount: impl Into<u64>) -> &mut Self {
        self.body.data.amount = Some(amount.into());
        self
    }

    /// Sets the serial numbers of a non-fungible token to wipe from the specified account.
    pub fn serial_numbers(&mut self, serial_numbers: impl IntoIterator<Item = u64>) -> &mut Self {
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
        let account = self.account_id.as_ref().map(AccountAddress::to_protobuf);
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);
        let amount = self.amount.clone().unwrap_or_default();
        let serial_numbers = self.serial_numbers.iter().map(|num| *num as i64).collect();

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

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use crate::{AccountAddress, AccountId, TokenId, TokenWipeTransaction};
    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // TODO write JSON fields
    // language=JSON
    const TOKEN_WIPE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenWipe",
  "accountId": "0.0.1001",
  "tokenId": "0.0.1002",
  "amount": 123,
  "serialNumbers": [
    1,
    2,
    3
  ]
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = TokenWipeTransaction::new();

        transaction
            .account_id(AccountId::from(1001))
            .token_id(TokenId::from(1002))
            .amount(123u64)
            .serial_numbers([1,2,3]);

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, TOKEN_WIPE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(TOKEN_WIPE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::TokenWipe(transaction) => transaction);

        let account_id = assert_matches!(data.account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(account_id, AccountId::from(1001));

        assert_eq!(data.token_id.unwrap(), TokenId::from(1002));
        assert_eq!(data.amount.unwrap(), 123);
        assert_eq!(data.serial_numbers, [1,2,3]);

        Ok(())
    }
}
