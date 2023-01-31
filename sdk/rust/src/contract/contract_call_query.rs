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
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    AccountId,
    ContractFunctionResult,
    ContractId,
    Error,
    LedgerId,
    Query,
    ToProtobuf,
    ValidateChecksums,
};

/// Call a function of the given smart contract instance.
/// It will consume the entire given amount of gas.
///
/// This is performed locally on the particular node that the client is communicating with.
/// It cannot change the state of the contract instance (and so, cannot spend
/// anything from the instance's cryptocurrency account).
///
pub type ContractCallQuery = Query<ContractCallQueryData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct ContractCallQueryData {
    /// The contract instance to call.
    contract_id: Option<ContractId>,

    /// The amount of gas to use for the call.
    gas: u64,

    /// The function parameters as their raw bytes.
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    function_parameters: Vec<u8>,

    /// The sender for this transaction.
    sender_account_id: Option<AccountId>,
}

impl ContractCallQuery {
    /// Gets the contract instance to call.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data.contract_id
    }

    /// Sets the contract to make a static call against.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data.contract_id = Some(contract_id);
        self
    }

    /// Gets the amount of gas to use for the call.
    #[must_use]
    pub fn get_gas(&self) -> u64 {
        self.data.gas
    }

    /// Sets the amount of gas to use for the call.
    pub fn gas(&mut self, gas: u64) -> &mut Self {
        self.data.gas = gas;
        self
    }

    /// Gets the function parameters as their raw bytes.
    #[must_use]
    pub fn get_contract_parameters(&self) -> &[u8] {
        self.data.function_parameters.as_ref()
    }

    /// Sets the function parameters as their raw bytes.
    pub fn function_parameters(&mut self, data: Vec<u8>) -> &mut Self {
        self.data.function_parameters = data;
        self
    }

    /// Gets the sender for this transaction.
    #[must_use]
    pub fn get_sender_account_id(&self) -> Option<AccountId> {
        self.data.sender_account_id
    }

    /// Sets the sender for this transaction.
    pub fn sender_account_id(&mut self, sender_account_id: AccountId) -> &mut Self {
        self.data.sender_account_id = Some(sender_account_id);
        self
    }
}

impl From<ContractCallQueryData> for AnyQueryData {
    #[inline]
    fn from(data: ContractCallQueryData) -> Self {
        Self::ContractCall(data)
    }
}

impl ToQueryProtobuf for ContractCallQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let contract_id = self.contract_id.to_protobuf();
        let sender_id = self.sender_account_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::ContractCallLocal(
                #[allow(deprecated)]
                services::ContractCallLocalQuery {
                    contract_id,
                    gas: self.gas as i64,
                    function_parameters: self.function_parameters.clone(),
                    max_result_size: 0,
                    header: Some(header),
                    sender_id,
                },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for ContractCallQueryData {
    type Response = ContractFunctionResult;

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksums_for_ledger_id(ledger_id)?;
        self.sender_account_id.validate_checksums_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        SmartContractServiceClient::new(channel).contract_call_local_method(request).await
    }
}
