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
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
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
    Hbar,
    LedgerId,
    ToProtobuf,
    Transaction,
    ValidateChecksums,
};

/// Call a function of the given smart contract instance, giving it
/// parameters as its inputs.
///
/// It can use the given amount of gas, and any unspent gas will
/// be refunded to the paying account.
///
/// If this function stores information, it is charged gas to store it.
/// There is a fee in hbars to maintain that storage until the expiration time,
/// and that fee is added as part of the transaction fee.
///
pub type ContractExecuteTransaction = Transaction<ContractExecuteTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct ContractExecuteTransactionData {
    /// The contract instance to call.
    contract_id: Option<ContractId>,

    /// The maximum amount of gas to use for the call.
    gas: u64,

    /// The number of hbars sent with this function call.
    payable_amount: Hbar,

    /// The function parameters as their raw bytes.
    function_parameters: Vec<u8>,
}

impl ContractExecuteTransaction {
    /// Returns the contract instance to call.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data().contract_id
    }

    /// Sets the contract instance to call.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data_mut().contract_id = Some(contract_id);
        self
    }

    /// Returns the maximum amount of gas to use for the call.
    #[must_use]
    pub fn get_gas(&self) -> u64 {
        self.data().gas
    }

    /// Sets the maximum amount of gas to use for the call.
    pub fn gas(&mut self, gas: u64) -> &mut Self {
        self.data_mut().gas = gas;
        self
    }

    /// Returns the number of hbars to be sent with this function call.
    #[must_use]
    pub fn get_payable_amount(&self) -> Hbar {
        self.data().payable_amount
    }

    /// Sets the number of hbars to be sent with this function call.
    pub fn payable_amount(&mut self, amount: Hbar) -> &mut Self {
        self.data_mut().payable_amount = amount;
        self
    }

    /// Returns the function parameters as their raw bytes.
    #[must_use]
    pub fn get_function_parameters(&self) -> &[u8] {
        &self.data().function_parameters
    }

    /// Sets the function parameters as their raw bytes.
    pub fn function_parameters(&mut self, data: Vec<u8>) -> &mut Self {
        self.data_mut().function_parameters = data;
        self
    }
}

impl TransactionData for ContractExecuteTransactionData {}

impl TransactionExecute for ContractExecuteTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async {
            SmartContractServiceClient::new(channel).contract_call_method(request).await
        })
    }
}

impl ValidateChecksums for ContractExecuteTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksums(ledger_id)?;
        Ok(())
    }
}

impl ToTransactionDataProtobuf for ContractExecuteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.to_protobuf();

        services::transaction_body::Data::ContractCall(
            #[allow(deprecated)]
            services::ContractCallTransactionBody {
                gas: self.gas as i64,
                amount: self.payable_amount.to_tinybars(),
                contract_id,
                function_parameters: self.function_parameters.clone(),
            },
        )
    }
}

impl From<ContractExecuteTransactionData> for AnyTransactionData {
    fn from(transaction: ContractExecuteTransactionData) -> Self {
        Self::ContractExecute(transaction)
    }
}

impl FromProtobuf<services::ContractCallTransactionBody> for ContractExecuteTransactionData {
    fn from_protobuf(pb: services::ContractCallTransactionBody) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            contract_id: Option::from_protobuf(pb.contract_id)?,
            gas: pb.gas as u64,
            payable_amount: Hbar::from_tinybars(pb.amount),
            function_parameters: pb.function_parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use assert_matches::assert_matches;

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            ContractExecuteTransaction,
            ContractId,
            Hbar,
        };

        // language=JSON
        const CONTRACT_EXECUTE_TRANSACTION_JSON: &str = r#"{
  "$type": "contractExecute",
  "contractId": "0.0.1001",
  "gas": 1000,
  "payableAmount": 10,
  "functionParameters": [
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
                .gas(1000)
                .payable_amount(Hbar::from_tinybars(10))
                .function_parameters("Hello, world!".into());

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, CONTRACT_EXECUTE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction =
                serde_json::from_str(CONTRACT_EXECUTE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::ContractExecute(transaction) => transaction);

            assert_eq!(data.contract_id.unwrap(), ContractId::from(1001));
            assert_eq!(data.gas, 1000);
            assert_eq!(data.payable_amount.to_tinybars(), 10);

            let bytes: Vec<u8> = "Hello, world!".into();
            assert_eq!(data.function_parameters, bytes);

            Ok(())
        }
    }
}
