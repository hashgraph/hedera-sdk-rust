use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::{serde_as, skip_serializing_none, DurationSeconds};
use time::Duration;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, AccountIdOrAlias, Key, Transaction};

/// Create a new Hedera™ account.
pub type AccountCreateTransaction = Transaction<AccountCreateTransactionData>;

// TODO: shard_id: Option<ShardId>
// TODO: realm_id: Option<RealmId>
// TODO: new_realm_admin_key: Option<Key>,
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountCreateTransactionData {
    /// The key that must sign each transfer out of the account.
    ///
    /// If `receiver_signature_required` is true, then it must also sign any transfer
    /// into the account.
    pub key: Option<Key>,

    /// The initial number of Hbar to put into the account.
    // TODO: Hbar
    pub initial_balance: u64,

    /// If true, this account's key must sign any transaction depositing into this account.
    pub receiver_signature_required: bool,

    /// The account is charged to extend its expiration date every this many seconds.
    #[serde_as(as = "Option<DurationSeconds>")]
    pub auto_renew_period: Option<Duration>,

    /// The memo associated with the account.
    pub account_memo: String,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    ///
    pub max_automatic_token_associations: u16,

    /// ID of the account to which this account is staking.
    pub staked_account_id: Option<AccountIdOrAlias>,

    /// If true, the account declines receiving a staking reward. The default value is false.
    pub decline_staking_reward: bool,
}

impl Default for AccountCreateTransactionData {
    fn default() -> Self {
        Self {
            key: None,
            initial_balance: 0,
            receiver_signature_required: false,
            auto_renew_period: Some(Duration::days(90)),
            account_memo: String::new(),
            max_automatic_token_associations: 0,
            staked_account_id: None,
            decline_staking_reward: false,
        }
    }
}

impl AccountCreateTransaction {
    /// Set the key for this account.
    pub fn key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.body.data.key = Some(key.into());
        self
    }

    /// Set the initial amount to transfer into this account.
    pub fn initial_balance(&mut self, balance: u64) -> &mut Self {
        self.body.data.initial_balance = balance;
        self
    }

    /// Set to true to require this account to sign any transfer of hbars to this account.
    pub fn receiver_signature_required(&mut self, required: bool) -> &mut Self {
        self.body.data.receiver_signature_required = required;
        self
    }

    /// Set the auto renew period for this account.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.body.data.auto_renew_period = Some(period);
        self
    }

    /// Set the memo associated with the account.
    pub fn account_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.account_memo = memo.into();
        self
    }

    /// Set the maximum number of tokens that an Account can be implicitly associated with.
    pub fn max_automatic_token_associations(&mut self, amount: u16) -> &mut Self {
        self.body.data.max_automatic_token_associations = amount;
        self
    }

    /// Set the ID of the account to which this account is staking.
    pub fn staked_account_id(&mut self, id: impl Into<AccountIdOrAlias>) -> &mut Self {
        self.body.data.staked_account_id = Some(id.into());
        self
    }

    /// Set to true, the account declines receiving a staking reward. The default value is false.
    pub fn decline_staking_reward(&mut self, decline: bool) -> &mut Self {
        self.body.data.decline_staking_reward = decline;
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountCreateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).create_account(request).await
    }
}

impl ToTransactionDataProtobuf for AccountCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let key = self.key.as_ref().map(Key::to_protobuf);
        let auto_renew_period = self.auto_renew_period.as_ref().map(Duration::to_protobuf);
        let staked_id = self.staked_account_id.as_ref().map(|id| {
            services::crypto_create_transaction_body::StakedId::StakedAccountId(id.to_protobuf())
        });

        services::transaction_body::Data::CryptoCreateAccount(
            #[allow(deprecated)]
            services::CryptoCreateTransactionBody {
                key,
                initial_balance: self.initial_balance,
                proxy_account_id: None,
                send_record_threshold: i64::MAX as u64,
                receive_record_threshold: i64::MAX as u64,
                receiver_sig_required: self.receiver_signature_required,
                auto_renew_period,
                shard_id: None,
                realm_id: None,
                new_realm_admin_key: None,
                memo: self.account_memo.clone(),
                max_automatic_token_associations: self.max_automatic_token_associations as i32,
                decline_reward: self.decline_staking_reward,
                staked_id,
            },
        )
    }
}

impl From<AccountCreateTransactionData> for AnyTransactionData {
    fn from(transaction: AccountCreateTransactionData) -> Self {
        Self::AccountCreate(transaction)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use time::Duration;

    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // language=JSON
    const ACCOUNT_CREATE_EMPTY: &str = r#"{
  "accountCreate": {}
}"#;

    #[test]
    fn it_should_deserialize_empty() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(ACCOUNT_CREATE_EMPTY)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::AccountCreate(transaction) => transaction);

        assert_eq!(data.auto_renew_period, Some(Duration::days(90)));

        Ok(())
    }
}
