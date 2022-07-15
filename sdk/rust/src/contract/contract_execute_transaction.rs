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
};
use tonic::transport::Channel;

use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    ContractId,
    ToProtobuf,
    Transaction,
};

/// Call a function of the given smart contract instance.
pub type ContractExecuteTransaction = Transaction<ContractExecuteTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractExecuteTransactionData {
    contract_id: Option<ContractId>,
    gas_limit: u64,
    value: u64,
    data: Vec<u8>,
}

impl ContractExecuteTransaction {
    /// Sets the contract to call.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.body.data.contract_id = Some(contract_id);
        self
    }

    /// Sets the gas limit for this transaction.
    pub fn gas_limit(&mut self, gas: u64) -> &mut Self {
        self.body.data.gas_limit = gas;
        self
    }

    /// Sets the value (in HBAR) for this transaction.
    pub fn value(&mut self, value: u64) -> &mut Self {
        self.body.data.value = value;
        self
    }

    /// Sets the data for this transaction.
    pub fn data(&mut self, data: Vec<u8>) -> &mut Self {
        self.body.data.data = data;
        self
    }
}

#[async_trait]
impl TransactionExecute for ContractExecuteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        SmartContractServiceClient::new(channel).contract_call_method(request).await
    }
}

impl ToTransactionDataProtobuf for ContractExecuteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.as_ref().map(ContractId::to_protobuf);

        services::transaction_body::Data::ContractCall(
            #[allow(deprecated)]
            services::ContractCallTransactionBody {
                gas: self.gas_limit as i64,
                amount: self.value as i64,
                contract_id,
                function_parameters: self.data.clone(),
            },
        )
    }
}

impl From<ContractExecuteTransactionData> for AnyTransactionData {
    fn from(transaction: ContractExecuteTransactionData) -> Self {
        Self::ContractExecute(transaction)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use crate::{ContractExecuteTransaction, ContractId};
    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // language=JSON
    const CONTRACT_EXECUTE_TRANSACTION_JSON: &str = r#"{
  "$type": "contractExecute",
  "contractId": "0.0.1001",
  "gasLimit": 1000,
  "value": 10,
  "data": [
    72,
    101,
    108,
    108,
    111,
    44,
    32,
    119,
    111,
    114,
    108,
    100,
    33
  ]
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = ContractExecuteTransaction::new();

        transaction
            .contract_id(ContractId::from(1001))
            .gas_limit(1000)
            .value(10)
            .data("Hello, world!".into());

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, CONTRACT_EXECUTE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(CONTRACT_EXECUTE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::ContractExecute(transaction) => transaction);

        assert_eq!(data.contract_id.unwrap(), ContractId::from(1001));
        assert_eq!(data.gas_limit, 1000);
        assert_eq!(data.value, 10);
        
        let bytes: Vec<u8> = "Hello, world!".into();
        assert_eq!(data.data, bytes);

        Ok(())
    }
}
