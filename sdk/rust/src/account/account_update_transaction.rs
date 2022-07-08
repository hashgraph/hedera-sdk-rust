use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::{
    serde_as,
    skip_serializing_none,
    DurationSeconds,
    TimestampNanoSeconds,
};
use time::{
    Duration,
    OffsetDateTime,
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
    Key,
    Transaction,
};

/// Change properties for the given account.
///
/// Any null field is ignored (left unchanged). This
/// transaction must be signed by the existing key for this account. If
/// the transaction is changing the key field, then the transaction must be
/// signed by both the old key (from before the change) and the new key.
///
pub type AccountUpdateTransaction = Transaction<AccountUpdateTransactionData>;

// TODO: shard_id: Option<ShardId>
// TODO: realm_id: Option<RealmId>
// TODO: new_realm_admin_key: Option<Key>,
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateTransactionData {
    /// The account ID which is being updated in this transaction.
    pub account_id: Option<AccountAddress>,

    /// The new key.
    pub key: Option<Key>,

    /// If true, this account's key must sign any transaction depositing into this account.
    pub receiver_signature_required: Option<bool>,

    /// The account is charged to extend its expiration date every this many seconds.
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub auto_renew_period: Option<Duration>,

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    pub expires_at: Option<OffsetDateTime>,

    /// The memo associated with the account.
    pub account_memo: Option<String>,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    ///
    pub max_automatic_token_associations: Option<u16>,

    /// ID of the account to which this account is staking.
    /// This is mutually exclusive with `staked_node_id`.
    pub staked_account_id: Option<AccountAddress>,

    /// ID of the node this account is staked to.
    /// This is mutually exclusive with `staked_account_id`.
    pub staked_node_id: Option<u64>,

    /// If true, the account declines receiving a staking reward. The default value is false.
    pub decline_staking_reward: Option<bool>,
}

impl AccountUpdateTransaction {
    /// Set the account ID which is being updated.
    pub fn account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.account_id = Some(id.into());
        self
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    pub fn expires_at(&mut self, at: OffsetDateTime) -> &mut Self {
        self.body.data.expires_at = Some(at);
        self
    }

    /// Set the key for this account.
    pub fn key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.body.data.key = Some(key.into());
        self
    }

    /// Set to true to require this account to sign any transfer of hbars to this account.
    pub fn receiver_signature_required(&mut self, required: bool) -> &mut Self {
        self.body.data.receiver_signature_required = Some(required);
        self
    }

    /// Set the auto renew period for this account.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.body.data.auto_renew_period = Some(period);
        self
    }

    /// Set the memo associated with the account.
    pub fn account_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.account_memo = Some(memo.into());
        self
    }

    /// Set the maximum number of tokens that an Account can be implicitly associated with.
    pub fn max_automatic_token_associations(&mut self, amount: u16) -> &mut Self {
        self.body.data.max_automatic_token_associations = Some(amount);
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
        self.body.data.decline_staking_reward = Some(decline);
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountUpdateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).update_account(request).await
    }
}

impl ToTransactionDataProtobuf for AccountUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let account_id = self.account_id.as_ref().map(AccountAddress::to_protobuf);
        let key = self.key.as_ref().map(Key::to_protobuf);
        let auto_renew_period = self.auto_renew_period.as_ref().map(Duration::to_protobuf);
        let expiration_time = self.expires_at.as_ref().map(OffsetDateTime::to_protobuf);

        let receiver_signature_required = self.receiver_signature_required.map(|required| {
            services::crypto_update_transaction_body::ReceiverSigRequiredField::ReceiverSigRequiredWrapper(required)
        });

        let staked_id = match (&self.staked_account_id, self.staked_node_id) {
            (_, Some(node_id)) => Some(
                services::crypto_update_transaction_body::StakedId::StakedNodeId(node_id as i64),
            ),

            (Some(account_id), _) => {
                Some(services::crypto_update_transaction_body::StakedId::StakedAccountId(
                    account_id.to_protobuf(),
                ))
            }

            _ => None,
        };

        services::transaction_body::Data::CryptoUpdateAccount(
            #[allow(deprecated)]
            services::CryptoUpdateTransactionBody {
                account_id_to_update: account_id,
                key,
                proxy_account_id: None,
                proxy_fraction: 0,
                auto_renew_period,
                expiration_time,
                memo: self.account_memo.clone(),
                max_automatic_token_associations: self
                    .max_automatic_token_associations
                    .map(Into::into),
                decline_reward: self.decline_staking_reward,
                send_record_threshold_field: None,
                receive_record_threshold_field: None,
                receiver_sig_required_field: receiver_signature_required,
                staked_id,
            },
        )
    }
}

impl From<AccountUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: AccountUpdateTransactionData) -> Self {
        Self::AccountUpdate(transaction)
    }
}
