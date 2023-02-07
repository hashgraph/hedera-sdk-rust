/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;

use crate::entity_id::AutoValidateChecksum;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::staked_id::StakedId;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    Key,
    LedgerId,
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

#[derive(Debug, Clone, Default)]
pub struct AccountUpdateTransactionData {
    /// The account ID which is being updated in this transaction.
    account_id: Option<AccountId>,

    /// The new key.
    key: Option<Key>,

    /// If true, this account's key must sign any transaction depositing into this account.
    receiver_signature_required: Option<bool>,

    /// The account is charged to extend its expiration date every this many seconds.
    auto_renew_period: Option<Duration>,

    auto_renew_account_id: Option<AccountId>,

    /// The ID of the account to which this account is proxy staked.
    ///
    /// If `proxy_account_id` is `None`, or is an invalid account, or is an account
    /// that isn't a node, then this account is automatically proxy staked to
    /// a node chosen by the network, but without earning payments.
    ///
    /// If the `proxy_account_id` account refuses to accept proxy staking, or
    /// if it is not currently running a node, then it
    /// will behave as if `proxy_account_id` was `None`.
    #[deprecated]
    proxy_account_id: Option<AccountId>,

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    expiration_time: Option<OffsetDateTime>,

    /// The memo associated with the account.
    account_memo: Option<String>,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    ///
    max_automatic_token_associations: Option<u16>,

    /// ID of the account or node to which this account is staking, if any.
    staked_id: Option<StakedId>,

    /// If true, the account declines receiving a staking reward. The default value is false.
    decline_staking_reward: Option<bool>,
}

impl AccountUpdateTransaction {
    /// Returns the ID for the account that is being updated.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data().account_id
    }

    /// Sets the ID for the account that is being updated.
    pub fn account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().account_id = Some(id);
        self
    }

    /// Gets the new expiration time to extend to (ignored if equal to or before the current one).
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    pub fn expiration_time(&mut self, at: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(at);
        self
    }

    /// Returns the key that the account will be updated to.
    #[must_use]
    pub fn get_key(&self) -> Option<&Key> {
        self.data().key.as_ref()
    }

    /// Sets the key for this account.
    pub fn key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().key = Some(key.into());
        self
    }

    /// If true, this account's key must sign any transaction depositing hbar into this account.
    #[must_use]
    pub fn get_receiver_signature_required(&self) -> Option<bool> {
        self.data().receiver_signature_required
    }

    /// Set to true to require this account to sign any transfer of hbars to this account.
    pub fn receiver_signature_required(&mut self, required: bool) -> &mut Self {
        self.data_mut().receiver_signature_required = Some(required);
        self
    }

    /// Gets the ID of the account to which this account will be updated to be proxy staked to.
    #[deprecated]
    #[allow(deprecated)]
    #[must_use]
    pub fn get_proxy_account_id(&self) -> Option<AccountId> {
        self.data().proxy_account_id
    }

    /// Sets the proxy account ID for this account.
    ///
    /// If `proxy_account_id` is `None`, or is an invalid account, or is an account
    /// that isn't a node, then this account is automatically proxy staked to
    /// a node chosen by the network, but without earning payments.
    ///
    /// If the `proxy_account_id` account refuses to accept proxy staking, or
    /// if it is not currently running a node, then it
    /// will behave as if `proxy_account_id` was `None`.
    #[deprecated]
    #[allow(deprecated)]
    pub fn proxy_account_id(&mut self, proxy_account_id: AccountId) -> &mut Self {
        self.data_mut().proxy_account_id = Some(proxy_account_id);
        self
    }

    /// Returns the new auto renew period.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the auto renew period for this account.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(period);
        self
    }

    /// Returns the new auto renew account id.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the account to be used at this account's expiration time to extend the
    /// life of the account.  If `None`, this account pays for its own auto renewal fee.
    pub fn auto_renew_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(id);
        self
    }

    /// Returns the memo associated with the account.
    #[must_use]
    pub fn get_account_memo(&self) -> Option<&str> {
        self.data().account_memo.as_deref()
    }

    /// Sets the memo associated with the account.
    pub fn account_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().account_memo = Some(memo.into());
        self
    }

    /// Returns the maximum number of tokens that an Account can be implicitly associated with.
    #[must_use]
    pub fn get_max_automatic_token_associations(&self) -> Option<u16> {
        self.data().max_automatic_token_associations
    }

    /// Sets the maximum number of tokens that an Account can be implicitly associated with.
    pub fn max_automatic_token_associations(&mut self, amount: u16) -> &mut Self {
        self.data_mut().max_automatic_token_associations = Some(amount);
        self
    }

    /// Returns the ID of the account to which this account is staking.
    /// This is mutually exclusive with `staked_node_id`.
    #[must_use]
    pub fn get_staked_account_id(&self) -> Option<AccountId> {
        self.data().staked_id.and_then(|it| it.to_account_id())
    }

    /// Sets the ID of the account to which this account is staking.
    /// This is mutually exclusive with `staked_node_id`.
    pub fn staked_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().staked_id = Some(id.into());
        self
    }

    /// Returns the ID of the node to which this account is staking.
    /// This is mutually exclusive with `staked_account_id`.
    #[must_use]
    pub fn get_staked_node_id(&self) -> Option<u64> {
        self.data().staked_id.and_then(|it| it.to_node_id())
    }

    /// Sets the ID of the node to which this account is staking.
    /// This is mutually exclusive with `staked_account_id`.
    pub fn staked_node_id(&mut self, id: u64) -> &mut Self {
        self.data_mut().staked_id = Some(id.into());
        self
    }

    /// Returns `true` if this account should decline receiving a staking reward,
    /// `false` if it should _not_,
    /// and `None` if the value should remain unchanged.
    #[must_use]
    pub fn get_decline_staking_reward(&self) -> Option<bool> {
        self.data().decline_staking_reward
    }

    /// If set to true, the account declines receiving a staking reward. The default value is false.
    pub fn decline_staking_reward(&mut self, decline: bool) -> &mut Self {
        self.data_mut().decline_staking_reward = Some(decline);
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountUpdateTransactionData {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.account_id.validate_checksum_for_ledger_id(ledger_id)?;
        self.staked_id.validate_checksum_for_ledger_id(ledger_id)
    }

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
        let account_id = self.account_id.to_protobuf();
        let key = self.key.to_protobuf();
        let auto_renew_period = self.auto_renew_period.to_protobuf();
        let auto_renew_account = self.auto_renew_account_id.to_protobuf();
        let expiration_time = self.expiration_time.to_protobuf();

        let receiver_signature_required = self.receiver_signature_required.map(|required| {
            services::crypto_update_transaction_body::ReceiverSigRequiredField::ReceiverSigRequiredWrapper(required)
        });

        let staked_id = self.staked_id.map(|id| match id {
            StakedId::NodeId(id) => {
                services::crypto_update_transaction_body::StakedId::StakedNodeId(id as i64)
            }
            StakedId::AccountId(id) => {
                services::crypto_update_transaction_body::StakedId::StakedAccountId(
                    id.to_protobuf(),
                )
            }
        });

        services::transaction_body::Data::CryptoUpdateAccount(
            #[allow(deprecated)]
            services::CryptoUpdateTransactionBody {
                account_id_to_update: account_id,
                key,
                proxy_account_id: self.proxy_account_id.to_protobuf(),
                proxy_fraction: 0,
                auto_renew_period,
                auto_renew_account,
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
                virtual_address_update: None, // TODO
            },
        )
    }
}

impl From<AccountUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: AccountUpdateTransactionData) -> Self {
        Self::AccountUpdate(transaction)
    }
}

impl FromProtobuf<services::CryptoUpdateTransactionBody> for AccountUpdateTransactionData {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::CryptoUpdateTransactionBody) -> crate::Result<Self> {
        use services::crypto_update_transaction_body::ReceiverSigRequiredField;

        let receiver_signature_required = pb.receiver_sig_required_field.map(|it| match it {
            ReceiverSigRequiredField::ReceiverSigRequired(it) => it,
            ReceiverSigRequiredField::ReceiverSigRequiredWrapper(it) => it,
        });

        Ok(Self {
            account_id: Option::from_protobuf(pb.account_id_to_update)?,
            key: Option::from_protobuf(pb.key)?,
            receiver_signature_required,
            auto_renew_period: pb.auto_renew_period.map(Into::into),
            auto_renew_account_id: Option::from_protobuf(pb.auto_renew_account)?,
            proxy_account_id: Option::from_protobuf(pb.proxy_account_id)?,
            expiration_time: pb.expiration_time.map(Into::into),
            account_memo: pb.memo,
            max_automatic_token_associations: pb
                .max_automatic_token_associations
                .map(|it| it as u16),
            staked_id: Option::from_protobuf(pb.staked_id)?,
            decline_staking_reward: pb.decline_reward,
        })
    }
}
