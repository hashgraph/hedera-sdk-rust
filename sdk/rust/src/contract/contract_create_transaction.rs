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
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use time::Duration;
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
use crate::staked_id::StakedId;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    FileId,
    Hbar,
    Key,
    LedgerId,
    ToProtobuf,
    Transaction,
    ValidateChecksums,
};

/// Start a new smart contract instance.
pub type ContractCreateTransaction = Transaction<ContractCreateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct ContractCreateTransactionData {
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::base64::Base64>>")
    )]
    bytecode: Option<Vec<u8>>,

    bytecode_file_id: Option<FileId>,

    admin_key: Option<Key>,

    gas: u64,

    initial_balance: Hbar,

    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<serde_with::DurationSeconds<i64>>")
    )]
    auto_renew_period: Duration,

    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    constructor_parameters: Vec<u8>,

    contract_memo: String,

    max_automatic_token_associations: u32,

    auto_renew_account_id: Option<AccountId>,

    /// ID of the account or node to which this contract is staking, if any.
    #[cfg_attr(feature = "ffi", serde(flatten))]
    staked_id: Option<StakedId>,

    decline_staking_reward: bool,
}

impl Default for ContractCreateTransactionData {
    fn default() -> Self {
        Self {
            bytecode: None,
            bytecode_file_id: None,
            admin_key: None,
            gas: 0,
            initial_balance: Hbar::ZERO,
            auto_renew_period: Duration::days(90),
            constructor_parameters: Vec::new(),
            contract_memo: String::new(),
            max_automatic_token_associations: 0,
            auto_renew_account_id: None,
            staked_id: None,
            decline_staking_reward: false,
        }
    }
}

impl ContractCreateTransaction {
    /// Returns the `FileId` to be used as the bytecode for this smart contract.
    #[must_use]
    pub fn get_bytecode_file_id(&self) -> Option<FileId> {
        self.data().bytecode_file_id
    }

    /// Sets the file to use as the bytes for the smart contract.
    pub fn bytecode_file_id(&mut self, file_id: FileId) -> &mut Self {
        self.data_mut().bytecode_file_id = Some(file_id);
        self
    }

    /// Returns the bytecode for the smart contract.
    #[must_use]
    pub fn get_bytecode(&self) -> Option<&[u8]> {
        self.data().bytecode.as_deref()
    }

    /// Sets the bytes of the smart contract.
    pub fn bytecode(&mut self, bytecode: impl AsRef<[u8]>) -> &mut Self {
        self.data_mut().bytecode = Some(bytecode.as_ref().to_vec());
        self
    }

    /// Returns the admin key.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the admin key.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(key.into());
        self
    }

    /// Returns the gas limit to deploy the smart contract.
    #[must_use]
    pub fn get_gas(&self) -> u64 {
        self.data().gas
    }

    /// Sets the gas limit to deploy the smart contract.
    pub fn gas(&mut self, gas: u64) -> &mut Self {
        self.data_mut().gas = gas;
        self
    }

    /// Returns the initial balance to put into the cryptocurrency account associated with the new smart contract.
    #[must_use]
    pub fn get_initial_balance(&self) -> Hbar {
        self.data().initial_balance
    }

    /// Sets the initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    pub fn initial_balance(&mut self, balance: Hbar) -> &mut Self {
        self.data_mut().initial_balance = balance;
        self
    }

    /// Returns the auto renew period for this smart contract.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Duration {
        self.data().auto_renew_period
    }

    /// Sets the auto renew period for this smart contract.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = period;
        self
    }

    /// Returns the parameters to pass to the constructor.
    #[must_use]
    pub fn get_constructor_parameters(&self) -> &[u8] {
        self.data().constructor_parameters.as_ref()
    }

    /// Sets the parameters to pass to the constructor.
    pub fn constructor_parameters(&mut self, parameters: impl AsRef<[u8]>) -> &mut Self {
        self.data_mut().constructor_parameters = parameters.as_ref().to_vec();
        self
    }

    /// Returns the memo for the new smart contract.
    #[must_use]
    pub fn get_contract_memo(&self) -> &str {
        self.data().contract_memo.as_str()
    }

    /// Sets the memo for the new smart contract.
    pub fn contract_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().contract_memo = memo.into();
        self
    }

    /// Returns the maximum number of tokens that the contract can be automatically associated with.
    #[must_use]
    pub fn get_max_automatic_token_associations(&self) -> u32 {
        self.data().max_automatic_token_associations
    }

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    pub fn max_automatic_token_associations(&mut self, max: u32) -> &mut Self {
        self.data_mut().max_automatic_token_associations = max;
        self
    }

    /// Returns the account ot be used at the contract's expiration time to extend the
    /// life of the contract
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

    /// Returns the ID of the account to which this contract is staking.
    #[must_use]
    pub fn get_staked_account_id(&self) -> Option<AccountId> {
        self.data().staked_id.and_then(|it| it.to_account_id())
    }

    /// Sets the ID of the account to which this contract is staking.
    ///
    /// This is mutually exclusive with [`staked_node_id`](Self::staked_node_id).
    pub fn staked_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().staked_id = Some(id.into());
        self
    }

    /// Returns the ID of the node to which this contract is staking.
    #[must_use]
    pub fn get_staked_node_id(&self) -> Option<u64> {
        self.data().staked_id.and_then(|it| it.to_node_id())
    }

    /// Sets the ID of the node to which this contract is staking.
    /// This is mutually exclusive with [`staked_account_id`](Self::staked_account_id).
    pub fn staked_node_id(&mut self, id: u64) -> &mut Self {
        self.data_mut().staked_id = Some(id.into());
        self
    }

    /// Returns `true` if the contract will decline receiving staking rewards, `false` otherwise.
    #[must_use]
    pub fn get_decline_staking_reward(&self) -> bool {
        self.data().decline_staking_reward
    }

    /// If `true` the contract should declie receiving staking rewards. The default value is `false`.
    pub fn decline_staking_reward(&mut self, decline: bool) -> &mut Self {
        self.data_mut().decline_staking_reward = decline;
        self
    }
}

#[async_trait]
impl TransactionExecute for ContractCreateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        SmartContractServiceClient::new(channel).create_contract(request).await
    }
}

impl ValidateChecksums for ContractCreateTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.bytecode_file_id.validate_checksums(ledger_id)?;
        self.auto_renew_account_id.validate_checksums(ledger_id)?;
        self.staked_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for ContractCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let admin_key = self.admin_key.to_protobuf();
        let auto_renew_period = self.auto_renew_period.into();
        let auto_renew_account_id = self.auto_renew_account_id.to_protobuf();

        let initcode_source = match (&self.bytecode, &self.bytecode_file_id) {
            (_, Some(file_id)) => {
                Some(services::contract_create_transaction_body::InitcodeSource::FileId(
                    file_id.to_protobuf(),
                ))
            }

            (Some(bytecode), _) => {
                Some(services::contract_create_transaction_body::InitcodeSource::Initcode(
                    bytecode.clone(),
                ))
            }

            _ => None,
        };

        let staked_id = match self.staked_id {
            Some(StakedId::NodeId(node_id)) => Some(
                services::contract_create_transaction_body::StakedId::StakedNodeId(node_id as i64),
            ),

            Some(StakedId::AccountId(account_id)) => {
                Some(services::contract_create_transaction_body::StakedId::StakedAccountId(
                    account_id.to_protobuf(),
                ))
            }

            _ => None,
        };

        services::transaction_body::Data::ContractCreateInstance(
            #[allow(deprecated)]
            services::ContractCreateTransactionBody {
                admin_key,
                gas: self.gas as i64,
                initial_balance: self.initial_balance.to_tinybars(),
                proxy_account_id: None,
                auto_renew_period: Some(auto_renew_period),
                constructor_parameters: self.constructor_parameters.clone(),
                shard_id: None,
                realm_id: None,
                new_realm_admin_key: None,
                memo: self.contract_memo.clone(),
                max_automatic_token_associations: self.max_automatic_token_associations as i32,
                auto_renew_account_id,
                decline_reward: self.decline_staking_reward,
                initcode_source,
                staked_id,
            },
        )
    }
}

impl From<ContractCreateTransactionData> for AnyTransactionData {
    fn from(transaction: ContractCreateTransactionData) -> Self {
        Self::ContractCreate(transaction)
    }
}

impl FromProtobuf<services::ContractCreateTransactionBody> for ContractCreateTransactionData {
    fn from_protobuf(pb: services::ContractCreateTransactionBody) -> crate::Result<Self> {
        use services::contract_create_transaction_body::InitcodeSource;
        let (bytecode, bytecode_file_id) = match pb.initcode_source {
            Some(InitcodeSource::FileId(it)) => (None, Some(FileId::from_protobuf(it)?)),
            Some(InitcodeSource::Initcode(it)) => (Some(it), None),
            None => (None, None),
        };

        Ok(Self {
            bytecode,
            bytecode_file_id,
            admin_key: Option::from_protobuf(pb.admin_key)?,
            gas: pb.gas as u64,
            initial_balance: Hbar::from_tinybars(pb.initial_balance),
            auto_renew_period: pb_getf!(pb, auto_renew_period)?.into(),
            constructor_parameters: pb.constructor_parameters,
            contract_memo: pb.memo,
            max_automatic_token_associations: pb.max_automatic_token_associations as u32,
            auto_renew_account_id: Option::from_protobuf(pb.auto_renew_account_id)?,
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

        use crate::staked_id::StakedId;
        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            AccountId,
            ContractCreateTransaction,
            FileId,
            Hbar,
            Key,
            PublicKey,
        };

        // language=JSON
        const CONTRACT_CREATE_EMPTY: &str = r#"{
  "$type": "contractCreate"
}"#;

        // language=JSON
        const CONTRACT_CREATE_TRANSACTION_JSON: &str = r#"{
  "$type": "contractCreate",
  "bytecode": "SGVsbG8sIHdvcmxkIQ==",
  "bytecodeFileId": "0.0.1001",
  "adminKey": {
    "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
  },
  "gas": 1000,
  "initialBalance": 1000000,
  "autoRenewPeriod": 7776000,
  "constructorParameters": "BQoP",
  "contractMemo": "A contract memo",
  "maxAutomaticTokenAssociations": 512,
  "autoRenewAccountId": "0.0.1002",
  "stakedAccountId": "0.0.1003",
  "declineStakingReward": false
}"#;

        const ADMIN_KEY: &str =
        "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = ContractCreateTransaction::new();

            transaction
                .bytecode("Hello, world!")
                .bytecode_file_id(FileId::from(1001))
                .admin_key(PublicKey::from_str(ADMIN_KEY)?)
                .gas(1000)
                .initial_balance(Hbar::from_tinybars(1_000_000))
                .auto_renew_period(Duration::days(90))
                .constructor_parameters([5, 10, 15])
                .contract_memo("A contract memo")
                .max_automatic_token_associations(512)
                .auto_renew_account_id(AccountId::from(1002))
                .staked_account_id(AccountId::from(1003))
                .decline_staking_reward(false);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, CONTRACT_CREATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction =
                serde_json::from_str(CONTRACT_CREATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.into_body().data, AnyTransactionData::ContractCreate(transaction) => transaction);

            assert_eq!(data.bytecode_file_id.unwrap(), FileId::from(1001));
            assert_eq!(data.gas, 1000);
            assert_eq!(data.initial_balance.to_tinybars(), 1_000_000);
            assert_eq!(data.auto_renew_period, Duration::days(90));
            assert_eq!(data.constructor_parameters, [5, 10, 15]);
            assert_eq!(data.contract_memo, "A contract memo");
            assert_eq!(data.max_automatic_token_associations, 512);
            assert_eq!(data.decline_staking_reward, false);

            let bytes: Vec<u8> = "Hello, world!".into();
            assert_eq!(data.bytecode.unwrap(), bytes);

            let admin_key =
                assert_matches!(data.admin_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(admin_key, PublicKey::from_str(ADMIN_KEY)?);

            assert_eq!(data.auto_renew_account_id, Some(AccountId::from(1002)));
            assert_eq!(data.staked_id, Some(StakedId::AccountId(AccountId::from(1003))));

            Ok(())
        }

        #[test]
        fn it_should_deserialize_empty() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(CONTRACT_CREATE_EMPTY)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::ContractCreate(transaction) => transaction);

            assert_eq!(data.auto_renew_period, Duration::days(90));
            assert_eq!(data.decline_staking_reward, false);

            Ok(())
        }
    }
}
