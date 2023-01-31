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
use time::Duration;
use tonic::transport::Channel;

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
    Hbar,
    Key,
    LedgerId,
    PublicKey,
    Transaction,
    ValidateChecksums,
};

/// Create a new Hedera™ account.
pub type AccountCreateTransaction = Transaction<AccountCreateTransactionData>;

// TODO: shard_id: Option<ShardId>
// TODO: realm_id: Option<RealmId>
// TODO: new_realm_admin_key: Option<Key>,

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct AccountCreateTransactionData {
    /// The key that must sign each transfer out of the account.
    ///
    /// If `receiver_signature_required` is true, then it must also sign any transfer
    /// into the account.
    key: Option<Key>,

    /// The initial number of Hbar to put into the account.
    initial_balance: Hbar,

    /// If true, this account's key must sign any transaction depositing into this account.
    receiver_signature_required: bool,

    /// The account is charged to extend its expiration date every this many seconds.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    auto_renew_period: Option<Duration>,

    /// The account to be used at this account's expiration time to extend the
    /// life of the account.  If `None`, this account pays for its own auto renewal fee.
    auto_renew_account_id: Option<AccountId>,

    /// The memo associated with the account.
    account_memo: String,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    max_automatic_token_associations: u16,

    /// A key to be used as the account's alias.
    alias: Option<PublicKey>,

    /// A 20-byte EVM address to be used as the account's evm address.
    evm_address: Option<[u8; 20]>,

    /// ID of the account or node to which this account is staking, if any.
    #[cfg_attr(feature = "ffi", serde(flatten))]
    staked_id: Option<StakedId>,

    /// If true, the account declines receiving a staking reward. The default value is false.
    decline_staking_reward: bool,
}

impl Default for AccountCreateTransactionData {
    fn default() -> Self {
        Self {
            key: None,
            initial_balance: Hbar::ZERO,
            receiver_signature_required: false,
            auto_renew_period: Some(Duration::days(90)),
            auto_renew_account_id: None,
            account_memo: String::new(),
            max_automatic_token_associations: 0,
            alias: None,
            evm_address: None,
            staked_id: None,
            decline_staking_reward: false,
        }
    }
}

impl AccountCreateTransaction {
    /// Get the key this account will be created with.
    ///
    /// Returns `Some(key)` if previously set, `None` otherwise.
    #[must_use]
    pub fn get_key(&self) -> Option<&Key> {
        self.data().key.as_ref()
    }

    /// Sets the key for this account.
    pub fn key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().key = Some(key.into());
        self
    }

    /// Get the balance that will be transferred to this account on creation.
    ///
    /// Returns `initial_balance` if previously set, `0` otherwise.
    #[must_use]
    pub fn get_initial_balance(&self) -> Hbar {
        self.data().initial_balance
    }

    /// Sets the balance that will be transferred to this account on creation.
    pub fn initial_balance(&mut self, balance: Hbar) -> &mut Self {
        self.data_mut().initial_balance = balance;
        self
    }

    /// Returns `true` if this account must sign any transfer of hbars _to_ itself.
    #[must_use]
    pub fn get_receiver_signature_required(&self) -> bool {
        self.data().receiver_signature_required
    }

    /// Sets to true to require this account to sign any transfer of hbars to this account.
    pub fn receiver_signature_required(&mut self, required: bool) -> &mut Self {
        self.data_mut().receiver_signature_required = required;
        self
    }

    /// Returns the auto renew period for this account.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the auto renew period for this account.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(period);
        self
    }

    /// Gets the account to be used at this account's expiration time to extend the
    /// life of the account.  If `None`, this account pays for its own auto renewal fee.
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

    /// Get the memo associated with the account
    #[must_use]
    pub fn get_account_memo(&self) -> &str {
        &self.data().account_memo
    }

    /// Sets the memo associated with the account.
    pub fn account_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().account_memo = memo.into();
        self
    }

    /// Get the maximum number of tokens that an Account can be implicitly associated with.
    ///
    /// Defaults to `0`. Allows up to a maximum value of `1000`.
    #[must_use]
    pub fn get_max_automatic_token_associations(&self) -> u16 {
        self.data().max_automatic_token_associations
    }

    /// Sets the maximum number of tokens that an Account can be implicitly associated with.
    pub fn max_automatic_token_associations(&mut self, amount: u16) -> &mut Self {
        self.data_mut().max_automatic_token_associations = amount;
        self
    }

    /// Returns the public key to be used as the account's alias.
    #[must_use]
    pub fn get_alias(&self) -> Option<&PublicKey> {
        self.data().alias.as_ref()
    }

    /// The bytes to be used as the account's alias.
    ///
    /// A given alias can map to at most one account on the network at a time. This uniqueness will be enforced
    /// relative to aliases currently on the network at alias assignment.
    ///
    /// If a transaction creates an account using an alias, any further crypto transfers to that alias will
    /// simply be deposited in that account, without creating anything, and with no creation fee being charged.
    pub fn alias(&mut self, key: PublicKey) -> &mut Self {
        self.data_mut().alias = Some(key);
        self
    }

    /// Returns the evm address the account will be created with.
    #[must_use]
    pub fn get_evm_address(&self) -> Option<[u8; 20]> {
        self.data().evm_address
    }

    /// The last 20 bytes of the keccak-256 hash of a ECDSA_SECP256K1 primitive key.
    pub fn evm_address(&mut self, evm_address: [u8; 20]) -> &mut Self {
        self.data_mut().evm_address = Some(evm_address);
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
        self.data_mut().staked_id = Some(StakedId::AccountId(id));
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
        self.data_mut().staked_id = Some(StakedId::NodeId(id));
        self
    }

    /// Returns `true` if the account should decline receiving staking rewards, `false` otherwise.
    #[must_use]
    pub fn get_decline_staking_reward(&self) -> bool {
        self.data().decline_staking_reward
    }

    /// If `true`, the account declines receiving a staking reward. The default value is false.
    pub fn decline_staking_reward(&mut self, decline: bool) -> &mut Self {
        self.data_mut().decline_staking_reward = decline;
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

impl ValidateChecksums for AccountCreateTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.staked_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for AccountCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let key = self.key.to_protobuf();
        let auto_renew_period = self.auto_renew_period.to_protobuf();
        let auto_renew_account = self.auto_renew_account_id.to_protobuf();
        let staked_id = self.staked_id.map(|it| match it {
            StakedId::NodeId(id) => {
                services::crypto_create_transaction_body::StakedId::StakedNodeId(id as i64)
            }
            StakedId::AccountId(id) => {
                services::crypto_create_transaction_body::StakedId::StakedAccountId(
                    id.to_protobuf(),
                )
            }
        });

        services::transaction_body::Data::CryptoCreateAccount(
            #[allow(deprecated)]
            services::CryptoCreateTransactionBody {
                key,
                initial_balance: self.initial_balance.to_tinybars() as u64,
                proxy_account_id: None,
                send_record_threshold: i64::MAX as u64,
                receive_record_threshold: i64::MAX as u64,
                receiver_sig_required: self.receiver_signature_required,
                auto_renew_period,
                auto_renew_account,
                shard_id: None,
                realm_id: None,
                new_realm_admin_key: None,
                memo: self.account_memo.clone(),
                max_automatic_token_associations: i32::from(self.max_automatic_token_associations),
                alias: self.alias.map_or(vec![], |key| key.to_bytes_raw()),
                evm_address: self.evm_address.map_or(vec![], Vec::from),
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

impl FromProtobuf<services::CryptoCreateTransactionBody> for AccountCreateTransactionData {
    fn from_protobuf(pb: services::CryptoCreateTransactionBody) -> crate::Result<Self> {
        let evm_address = (!pb.evm_address.is_empty())
            .then(|| pb.evm_address.as_slice().try_into())
            .transpose()
            .map_err(Error::basic_parse)?;

        Ok(Self {
            key: Option::from_protobuf(pb.key)?,
            initial_balance: Hbar::from_tinybars(pb.initial_balance as i64),
            receiver_signature_required: pb.receiver_sig_required,
            auto_renew_period: pb.auto_renew_period.map(Into::into),
            auto_renew_account_id: Option::from_protobuf(pb.auto_renew_account)?,
            account_memo: pb.memo,
            max_automatic_token_associations: pb.max_automatic_token_associations as u16,
            alias: PublicKey::from_alias_bytes(&pb.alias)?,
            evm_address,
            staked_id: Option::from_protobuf(pb.staked_id)?,
            decline_staking_reward: pb.decline_reward,
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use std::str::FromStr;

        use assert_matches::assert_matches;
        use time::Duration;

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            AccountCreateTransaction,
            Hbar,
            Key,
            PublicKey,
        };

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
  "stakedNodeId": 7,
  "declineStakingReward": false
}"#;

        const KEY: &str =
        "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";

        #[test]
        #[ignore = "auto renew period is `None`"]
        fn it_should_deserialize_empty() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(ACCOUNT_CREATE_EMPTY)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::AccountCreate(transaction) => transaction);

            assert_eq!(data.auto_renew_period, Some(Duration::days(90)));

            Ok(())
        }

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = AccountCreateTransaction::new();

            transaction
                .key(PublicKey::from_str(KEY)?)
                .initial_balance(Hbar::from_tinybars(1000))
                .receiver_signature_required(true)
                .auto_renew_period(Duration::days(90))
                .account_memo("An account memo")
                .max_automatic_token_associations(256)
                .staked_node_id(7)
                .decline_staking_reward(false);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, ACCOUNT_CREATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction =
                serde_json::from_str(ACCOUNT_CREATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::AccountCreate(transaction) => transaction);

            assert_eq!(data.initial_balance.to_tinybars(), 1000);
            assert_eq!(data.receiver_signature_required, true);
            assert_eq!(data.auto_renew_period.unwrap(), Duration::days(90));
            assert_eq!(data.account_memo, "An account memo");
            assert_eq!(data.max_automatic_token_associations, 256);
            assert_eq!(data.staked_id, Some(7.into()));
            assert_eq!(data.decline_staking_reward, false);

            let key = assert_matches!(data.key, Some(Key::Single(public_key)) => public_key);

            assert_eq!(key, PublicKey::from_str(KEY)?);

            Ok(())
        }
    }
}
