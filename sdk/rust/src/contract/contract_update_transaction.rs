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

use hedera_proto::services;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
use crate::staked_id::StakedId;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    ContractId,
    Error,
    Key,
    LedgerId,
    ToProtobuf,
    Transaction,
    ValidateChecksums,
};

/// Updates the fields of a smart contract to the given values.
pub type ContractUpdateTransaction = Transaction<ContractUpdateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct ContractUpdateTransactionData {
    contract_id: Option<ContractId>,

    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,

    admin_key: Option<Key>,

    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    auto_renew_period: Option<Duration>,

    contract_memo: Option<String>,

    max_automatic_token_associations: Option<u32>,

    auto_renew_account_id: Option<AccountId>,

    proxy_account_id: Option<AccountId>,

    /// ID of the account or node to which this contract is staking, if any.
    #[cfg_attr(feature = "ffi", serde(flatten))]
    staked_id: Option<StakedId>,

    decline_staking_reward: Option<bool>,
}

impl ContractUpdateTransaction {
    /// Returns the contract to be updated.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data().contract_id
    }

    /// Sets the contract to be updated.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data_mut().contract_id = Some(contract_id);
        self
    }

    /// Returns the new admin key.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the new admin key.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(key.into());
        self
    }

    /// Returns the new expiration time to extend to (ignored if equal to or before the current one).
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    pub fn expiration_time(&mut self, at: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(at);
        self
    }

    /// Returns the auto renew period for this smart contract.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the auto renew period for this smart contract.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(period);
        self
    }

    /// Returns the new memo for the smart contract.
    #[must_use]
    pub fn get_contract_memo(&self) -> Option<&str> {
        self.data().contract_memo.as_deref()
    }

    /// Sets the new memo for the smart contract.
    pub fn contract_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().contract_memo = Some(memo.into());
        self
    }

    /// Returns the maximum number of tokens that this contract can be automatically associated with.
    #[must_use]
    pub fn get_max_automatic_token_associations(&self) -> Option<u32> {
        self.data().max_automatic_token_associations
    }

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    pub fn max_automatic_token_associations(&mut self, max: u32) -> &mut Self {
        self.data_mut().max_automatic_token_associations = Some(max);
        self
    }

    /// Returns the account to be used at the contract's expiration time to extend the
    /// life of the contract.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the account to be used at the contract's expiration time to extend the
    /// life of the contract.
    pub fn auto_renew_account_id(&mut self, account_id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(account_id);
        self
    }

    /// Returns the ID of the account to which this contract is proxy staked.
    #[must_use]
    pub fn get_proxy_account_id(&self) -> Option<AccountId> {
        self.data().proxy_account_id
    }

    /// Sets the ID of the account to which this contract is proxy staked.
    pub fn proxy_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().proxy_account_id = Some(id);
        self
    }

    /// Returns the ID of the account to which this contract is staking.
    #[must_use]
    pub fn get_staked_account_id(&self) -> Option<AccountId> {
        self.data().staked_id.and_then(|id| id.to_account_id())
    }

    /// Sets the ID of the account to which this contract is staking.
    /// This is mutually exclusive with `staked_node_id`.
    pub fn staked_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().staked_id = Some(id.into());
        self
    }

    /// Returns the ID of the node to which this contract is staking.
    #[must_use]
    pub fn get_staked_node_id(&self) -> Option<u64> {
        self.data().staked_id.and_then(|id| id.to_node_id())
    }

    /// Sets the ID of the node to which this contract is staking.
    /// This is mutually exclusive with `staked_account_id`.
    pub fn staked_node_id(&mut self, id: u64) -> &mut Self {
        self.data_mut().staked_id = Some(id.into());
        self
    }

    /// Returns `true` if the contract will be updated decline staking rewards,
    /// `false` if it will be updated to _not_,
    /// and `None` if it will not be updated.
    #[must_use]
    pub fn get_decline_staking_reward(&self) -> Option<bool> {
        self.data().decline_staking_reward
    }

    /// Sets to true, the contract declines receiving a staking reward. The default value is false.
    pub fn decline_staking_reward(&mut self, decline: bool) -> &mut Self {
        self.data_mut().decline_staking_reward = Some(decline);
        self
    }
}

impl TransactionData for ContractUpdateTransactionData {}

impl TransactionExecute for ContractUpdateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { SmartContractServiceClient::new(channel).update_contract(request).await })
    }
}

impl ValidateChecksums for ContractUpdateTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksums(ledger_id)?;
        self.auto_renew_account_id.validate_checksums(ledger_id)?;
        self.staked_id.validate_checksums(ledger_id)?;
        self.proxy_account_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for ContractUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.to_protobuf();
        let expiration_time = self.expiration_time.map(Into::into);
        let admin_key = self.admin_key.to_protobuf();
        let auto_renew_period = self.auto_renew_period.map(Into::into);
        let auto_renew_account_id = self.auto_renew_account_id.to_protobuf();

        let staked_id = self.staked_id.map(|id| match id {
            StakedId::NodeId(id) => {
                services::contract_update_transaction_body::StakedId::StakedNodeId(id as i64)
            }

            StakedId::AccountId(id) => {
                services::contract_update_transaction_body::StakedId::StakedAccountId(
                    id.to_protobuf(),
                )
            }
        });

        let memo_field = self
            .contract_memo
            .clone()
            .map(services::contract_update_transaction_body::MemoField::MemoWrapper);

        services::transaction_body::Data::ContractUpdateInstance(
            #[allow(deprecated)]
            services::ContractUpdateTransactionBody {
                contract_id,
                expiration_time,
                admin_key,
                proxy_account_id: self.proxy_account_id.to_protobuf(),
                auto_renew_period,
                max_automatic_token_associations: self
                    .max_automatic_token_associations
                    .map(|max| max as i32),
                auto_renew_account_id,
                decline_reward: self.decline_staking_reward,
                staked_id,
                file_id: None,
                memo_field,
            },
        )
    }
}

impl FromProtobuf<services::ContractUpdateTransactionBody> for ContractUpdateTransactionData {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::ContractUpdateTransactionBody) -> crate::Result<Self> {
        use services::contract_update_transaction_body::MemoField;

        Ok(Self {
            contract_id: Option::from_protobuf(pb.contract_id)?,
            expiration_time: pb.expiration_time.map(Into::into),
            admin_key: Option::from_protobuf(pb.admin_key)?,
            auto_renew_period: pb.auto_renew_period.map(Into::into),
            contract_memo: pb.memo_field.map(|it| match it {
                MemoField::Memo(it) => it,
                MemoField::MemoWrapper(it) => it,
            }),
            max_automatic_token_associations: pb
                .max_automatic_token_associations
                .map(|it| it as u32),
            auto_renew_account_id: Option::from_protobuf(pb.auto_renew_account_id)?,
            proxy_account_id: Option::from_protobuf(pb.proxy_account_id)?,
            staked_id: Option::from_protobuf(pb.staked_id)?,
            decline_staking_reward: pb.decline_reward,
        })
    }
}

impl From<ContractUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: ContractUpdateTransactionData) -> Self {
        Self::ContractUpdate(transaction)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use std::str::FromStr;

        use assert_matches::assert_matches;
        use time::{
            Duration,
            OffsetDateTime,
        };

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            AccountId,
            ContractId,
            ContractUpdateTransaction,
            Key,
            PublicKey,
        };

        // language=JSON
        const CONTRACT_UPDATE_TRANSACTION_JSON: &str = r#"{
  "$type": "contractUpdate",
  "contractId": "0.0.1001",
  "expirationTime": 1656352251277559886,
  "adminKey": {
    "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
  },
  "autoRenewPeriod": 7776000,
  "contractMemo": "A contract memo",
  "maxAutomaticTokenAssociations": 1024,
  "autoRenewAccountId": "0.0.1002",
  "stakedAccountId": "0.0.1003",
  "declineStakingReward": true
}"#;

        const ADMIN_KEY: &str =
        "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = ContractUpdateTransaction::new();

            transaction
                .contract_id(ContractId::from(1001))
                .expiration_time(OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?)
                .admin_key(PublicKey::from_str(ADMIN_KEY)?)
                .auto_renew_period(Duration::days(90))
                .contract_memo("A contract memo")
                .max_automatic_token_associations(1024)
                .auto_renew_account_id(AccountId::from(1002))
                .staked_account_id(AccountId::from(1003))
                .decline_staking_reward(true);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, CONTRACT_UPDATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction =
                serde_json::from_str(CONTRACT_UPDATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.into_body().data, AnyTransactionData::ContractUpdate(transaction) => transaction);

            assert_eq!(data.contract_id.unwrap(), ContractId::from(1001));
            assert_eq!(
                data.expiration_time.unwrap(),
                OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?
            );
            assert_eq!(data.auto_renew_period.unwrap(), Duration::days(90));
            assert_eq!(data.contract_memo.unwrap(), "A contract memo");
            assert_eq!(data.max_automatic_token_associations.unwrap(), 1024);
            assert_eq!(data.decline_staking_reward.unwrap(), true);

            let admin_key =
                assert_matches!(data.admin_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(admin_key, PublicKey::from_str(ADMIN_KEY)?);

            assert_eq!(data.auto_renew_account_id, Some(AccountId::from(1002)));
            assert_eq!(data.staked_id, Some(AccountId::from(1003).into()));

            Ok(())
        }
    }
}
