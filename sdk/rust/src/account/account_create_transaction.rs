use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::{serde_as, skip_serializing_none, DurationSeconds};
use time::Duration;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, Key, Transaction};

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
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub auto_renew_period: Option<Duration>,

    /// The memo associated with the account.
    pub account_memo: String,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    ///
    pub max_automatic_token_associations: u16,

    /// ID of the account to which this account is staking.
    /// This is mutually exclusive with `staked_node_id`.
    pub staked_account_id: Option<AccountAddress>,

    /// ID of the node this account is staked to.
    /// This is mutually exclusive with `staked_account_id`.
    pub staked_node_id: Option<u64>,

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
            staked_node_id: None,
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
    /// This is mutually exclusive with `staked_node_id`.
    pub fn staked_account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.staked_account_id = Some(id.into());
        self
    }

    /// Set the ID of the node to which this account is staking.
    /// This is mutually exclusive with `staked_account_id`.
    pub fn staked_node_id(&mut self, id: u64) -> &mut Self {
        self.body.data.staked_node_id = Some(id);
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

        let staked_id = match (&self.staked_account_id, self.staked_node_id) {
            (_, Some(node_id)) => Some(
                services::crypto_create_transaction_body::StakedId::StakedNodeId(node_id as i64),
            ),

            (Some(account_id), _) => {
                Some(services::crypto_create_transaction_body::StakedId::StakedAccountId(
                    account_id.to_protobuf(),
                ))
            }

            _ => None,
        };

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
    use std::str::FromStr;
    use assert_matches::assert_matches;
    use time::Duration;
    use crate::{AccountAddress, AccountCreateTransaction, AccountId, Key, PublicKey};

    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // language=JSON
    const ACCOUNT_CREATE_EMPTY: &str = r#"{
  "$type": "accountCreate"
}"#;

    // language=JSON
    const ACCOUNT_CREATE_TRANSACTION_JSON: &str = r#"{
  "$type": "accountCreate",
  "key": {
    "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
  },
  "initialBalance": 1000,
  "receiverSignatureRequired": true,
  "autoRenewPeriod": 7776000,
  "accountMemo": "An account memo",
  "maxAutomaticTokenAssociations": 256,
  "stakedAccountId": "0.0.1001",
  "stakedNodeId": 7,
  "declineStakingReward": false
}"#;

    const KEY: &str = "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";

    #[test]
    fn it_should_deserialize_empty() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(ACCOUNT_CREATE_EMPTY)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::AccountCreate(transaction) => transaction);

        assert_eq!(data.auto_renew_period, Some(Duration::days(90)));

        Ok(())
    }

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = AccountCreateTransaction::new();

        transaction
            .key(PublicKey::from_str(KEY)?)
            .initial_balance(1000)
            .receiver_signature_required(true)
            .auto_renew_period(Duration::days(90))
            .account_memo("An account memo")
            .max_automatic_token_associations(256)
            .staked_account_id(AccountId::from(1001))
            .staked_node_id(7)
            .decline_staking_reward(false);

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, ACCOUNT_CREATE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(ACCOUNT_CREATE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::AccountCreate(transaction) => transaction);

        assert_eq!(data.initial_balance, 1000);
        assert_eq!(data.receiver_signature_required, true);
        assert_eq!(data.auto_renew_period.unwrap(), Duration::days(90));
        assert_eq!(data.account_memo, "An account memo");
        assert_eq!(data.max_automatic_token_associations, 256);
        assert_eq!(data.staked_node_id.unwrap(), 7);
        assert_eq!(data.decline_staking_reward, false);

        let key = assert_matches!(data.key.unwrap(), Key::Single(public_key) => public_key);
        assert_eq!(key, PublicKey::from_str(KEY)?);

        let staked_account_id = assert_matches!(data.staked_account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(staked_account_id, AccountId::from(1001));

        Ok(())
    }
}
