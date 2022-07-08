use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use serde::{
    Deserialize,
    Serialize,
};
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

use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountAddress,
    AccountId,
    ContractId,
    Key,
    ToProtobuf,
    Transaction,
};

/// Updates the fields of a smart contract to the given values.
pub type ContractUpdateTransaction = Transaction<ContractUpdateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct ContractUpdateTransactionData {
    contract_id: Option<ContractId>,

    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    expires_at: Option<OffsetDateTime>,

    admin_key: Option<Key>,

    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    auto_renew_period: Option<Duration>,

    contract_memo: Option<String>,

    max_automatic_token_associations: Option<u32>,

    auto_renew_account_id: Option<AccountAddress>,

    staked_account_id: Option<AccountAddress>,

    staked_node_id: Option<u64>,

    decline_staking_reward: Option<bool>,
}

impl ContractUpdateTransaction {
    /// Sets the contract to be updated.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.body.data.contract_id = Some(contract_id);
        self
    }

    /// Sets the admin key.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.body.data.admin_key = Some(key.into());
        self
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    pub fn expires_at(&mut self, at: OffsetDateTime) -> &mut Self {
        self.body.data.expires_at = Some(at);
        self
    }

    /// Set the auto renew period for this smart contract.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.body.data.auto_renew_period = Some(period);
        self
    }

    /// Sets the memo for the new smart contract.
    pub fn contract_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.contract_memo = Some(memo.into());
        self
    }

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    pub fn max_automatic_token_associations(&mut self, max: u32) -> &mut Self {
        self.body.data.max_automatic_token_associations = Some(max);
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
        self.body.data.decline_staking_reward = Some(decline);
        self
    }
}

#[async_trait]
impl TransactionExecute for ContractUpdateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        SmartContractServiceClient::new(channel).update_contract(request).await
    }
}

impl ToTransactionDataProtobuf for ContractUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.as_ref().map(ContractId::to_protobuf);
        let expiration_time = self.expires_at.map(Into::into);
        let admin_key = self.admin_key.as_ref().map(Key::to_protobuf);
        let auto_renew_period = self.auto_renew_period.map(Into::into);
        let auto_renew_account_id =
            self.auto_renew_account_id.as_ref().map(AccountAddress::to_protobuf);

        let staked_id = match (&self.staked_account_id, self.staked_node_id) {
            (_, Some(node_id)) => Some(
                services::contract_update_transaction_body::StakedId::StakedNodeId(node_id as i64),
            ),

            (Some(account_id), _) => {
                Some(services::contract_update_transaction_body::StakedId::StakedAccountId(
                    account_id.to_protobuf(),
                ))
            }

            _ => None,
        };

        let memo_field = match self.contract_memo.clone() {
            Some(memo) => {
                Some(services::contract_update_transaction_body::MemoField::MemoWrapper(memo))
            }
            None => None,
        };

        services::transaction_body::Data::ContractUpdateInstance(
            #[allow(deprecated)]
            services::ContractUpdateTransactionBody {
                contract_id,
                expiration_time,
                admin_key,
                proxy_account_id: None,
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

impl From<ContractUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: ContractUpdateTransactionData) -> Self {
        Self::ContractUpdate(transaction)
    }
}
