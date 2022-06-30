use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::{serde_as, skip_serializing_none, DurationSeconds};
use time::Duration;
use tonic::transport::Channel;

use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, FileId, Key, ToProtobuf, Transaction};

/// Start a new smart contract instance.
pub type ContractCreateTransaction = Transaction<ContractCreateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct ContractCreateTransactionData {
    #[serde_as(as = "Option<Base64>")]
    bytecode: Option<Vec<u8>>,

    bytecode_file_id: Option<FileId>,

    admin_key: Option<Key>,

    gas_limit: u64,

    // TODO: Hbar
    initial_balance: u64,

    #[serde_as(as = "DurationSeconds<i64>")]
    auto_renew_period: Duration,

    #[serde_as(as = "Base64")]
    constructor_parameters: Vec<u8>,

    contract_memo: String,

    max_automatic_token_associations: u32,

    auto_renew_account_id: Option<AccountAddress>,

    staked_account_id: Option<AccountAddress>,

    staked_node_id: Option<u64>,

    decline_staking_reward: bool,
}

impl Default for ContractCreateTransactionData {
    fn default() -> Self {
        Self {
            bytecode: None,
            bytecode_file_id: None,
            admin_key: None,
            gas_limit: 0,
            initial_balance: 0,
            auto_renew_period: Duration::days(90),
            constructor_parameters: Vec::new(),
            contract_memo: String::new(),
            max_automatic_token_associations: 0,
            auto_renew_account_id: None,
            staked_account_id: None,
            staked_node_id: None,
            decline_staking_reward: false,
        }
    }
}

impl ContractCreateTransaction {
    /// Sets the file to use as the bytes for the smart contract.
    pub fn bytecode_file_id(&mut self, file_id: FileId) -> &mut Self {
        self.body.data.bytecode_file_id = Some(file_id);
        self
    }

    /// Sets the bytes of the smart contract.
    pub fn bytecode(&mut self, bytecode: impl AsRef<[u8]>) -> &mut Self {
        self.body.data.bytecode = Some(bytecode.as_ref().to_vec());
        self
    }

    /// Sets the admin key.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.body.data.admin_key = Some(key.into());
        self
    }

    /// Sets the gas limit to deploy the smart contract.
    pub fn gas_limit(&mut self, gas: u64) -> &mut Self {
        self.body.data.gas_limit = gas;
        self
    }

    /// Sets the initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    pub fn initial_balance(&mut self, balance: u64) -> &mut Self {
        self.body.data.initial_balance = balance;
        self
    }

    /// Set the auto renew period for this smart contract.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.body.data.auto_renew_period = period;
        self
    }

    /// Sets the parameters to pass to the constructor.
    pub fn constructor_parameters(&mut self, parameters: impl AsRef<[u8]>) -> &mut Self {
        self.body.data.constructor_parameters = parameters.as_ref().to_vec();
        self
    }

    /// Sets the memo for the new smart contract.
    pub fn contract_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.contract_memo = memo.into();
        self
    }

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    pub fn max_automatic_token_associations(&mut self, max: u32) -> &mut Self {
        self.body.data.max_automatic_token_associations = max;
        self
    }

    /// Sets the account to be used at the contract's expiration time to extend the
    /// life of the contract.
    pub fn auto_renew_account_id(&mut self, account_id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.auto_renew_account_id = Some(account_id.into());
        self
    }

    /// Set the ID of the account to which this contract is staking.
    /// This is mutually exclusive with `staked_node_id`.
    pub fn staked_account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.staked_account_id = Some(id.into());
        self
    }

    /// Set the ID of the node to which this contract is staking.
    /// This is mutually exclusive with `staked_account_id`.
    pub fn staked_node_id(&mut self, id: u64) -> &mut Self {
        self.body.data.staked_node_id = Some(id);
        self
    }

    /// Set to true, the contract declines receiving a staking reward. The default value is false.
    pub fn decline_staking_reward(&mut self, decline: bool) -> &mut Self {
        self.body.data.decline_staking_reward = decline;
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

impl ToTransactionDataProtobuf for ContractCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let admin_key = self.admin_key.as_ref().map(Key::to_protobuf);
        let auto_renew_period = self.auto_renew_period.into();
        let auto_renew_account_id =
            self.auto_renew_account_id.as_ref().map(AccountAddress::to_protobuf);

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

        let staked_id = match (&self.staked_account_id, self.staked_node_id) {
            (_, Some(node_id)) => Some(
                services::contract_create_transaction_body::StakedId::StakedNodeId(node_id as i64),
            ),

            (Some(account_id), _) => {
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
                gas: self.gas_limit as i64,
                initial_balance: self.initial_balance as i64,
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use assert_matches::assert_matches;
    use time::Duration;
    use crate::{AccountAddress, AccountId, ContractCreateTransaction, FileId, Key, PublicKey};
    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // language=JSON
    const CONTRACT_CREATE_TRANSACTION_JSON: &str = r#"{
  "$type": "contractCreate",
  "bytecode": "SGVsbG8sIHdvcmxkIQ==",
  "bytecodeFileId": "0.0.1001",
  "adminKey": {
    "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
  },
  "gasLimit": 1000,
  "initialBalance": 1000000,
  "autoRenewPeriod": 7776000,
  "constructorParameters": "BQoP",
  "contractMemo": "A contract memo",
  "maxAutomaticTokenAssociations": 512,
  "autoRenewAccountId": "0.0.1002",
  "stakedAccountId": "0.0.1003",
  "stakedNodeId": 7,
  "declineStakingReward": false
}"#;

    const ADMIN_KEY: &str = "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = ContractCreateTransaction::new();

        transaction
            .bytecode("Hello, world!")
            .bytecode_file_id(FileId::from(1001))
            .admin_key(PublicKey::from_str(ADMIN_KEY)?)
            .gas_limit(1000)
            .initial_balance(1_000_000)
            .auto_renew_period(Duration::days(90))
            .constructor_parameters([ 5, 10, 15 ])
            .contract_memo("A contract memo")
            .max_automatic_token_associations(512)
            .auto_renew_account_id(AccountId::from(1002))
            .staked_account_id(AccountId::from(1003))
            .staked_node_id(7)
            .decline_staking_reward(false);

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, CONTRACT_CREATE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(CONTRACT_CREATE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::ContractCreate(transaction) => transaction);

        let bytes: Vec<u8> = "Hello, world!".into();
        assert_eq!(data.bytecode.unwrap(), bytes);

        assert_eq!(data.bytecode_file_id.unwrap(), FileId::from(1001));
        assert_eq!(data.gas_limit, 1000);
        assert_eq!(data.initial_balance, 1_000_000);
        assert_eq!(data.auto_renew_period, Duration::days(90));
        assert_eq!(data.constructor_parameters, [5, 10, 15]);
        assert_eq!(data.contract_memo, "A contract memo");
        assert_eq!(data.max_automatic_token_associations, 512);
        assert_eq!(data.staked_node_id.unwrap(), 7);
        assert_eq!(data.decline_staking_reward, false);

        let admin_key = assert_matches!(data.admin_key.unwrap(), Key::Single(public_key) => public_key);
        assert_eq!(admin_key, PublicKey::from_str(ADMIN_KEY)?);
        let auto_renew_account_id = assert_matches!(data.auto_renew_account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(auto_renew_account_id, AccountId::from(1002));
        let staked_account_id = assert_matches!(data.staked_account_id.unwrap(), AccountAddress::AccountId(account_id) => account_id);
        assert_eq!(staked_account_id, AccountId::from(1003));

        Ok(())
    }
}
