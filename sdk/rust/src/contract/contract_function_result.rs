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
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::base64::Base64;

use crate::{
    AccountId,
    ContractId,
    FromProtobuf,
};

// TODO: log info
/// The result returned by a call to a smart contract function.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractFunctionResult {
    /// The smart contract instance whose function was called.
    pub contract_id: ContractId,

    /// The new contract's 20-byte EVM address.
    pub evm_address: Option<ContractId>,

    /// The raw bytes returned by the function.
    #[serde(with = "serde_with::As::<Base64>")]
    pub bytes: Vec<u8>,

    /// Message if there was an error during smart contract execution.
    pub error_message: Option<String>,

    /// Bloom filter for record.
    #[serde(with = "serde_with::As::<Base64>")]
    pub bloom: Vec<u8>,

    /// Units of gas used to execute contract.
    pub gas_used: u64,

    /// The amount of gas available for the call.
    pub gas: u64,

    /// Number of HBAR sent (the function must be payable if this is nonzero).
    pub hbar_amount: u64,

    /// The parameters passed into the contract call.
    #[serde(with = "serde_with::As::<Base64>")]
    pub contract_function_parameters_bytes: Vec<u8>,

    /// The account that is the "sender." If not present it is the accountId from the transactionId.
    pub sender_account_id: Option<AccountId>,
}

impl FromProtobuf<services::ContractFunctionResult> for ContractFunctionResult {
    fn from_protobuf(pb: services::ContractFunctionResult) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let contract_id = pb_getf!(pb, contract_id)?;
        let contract_id = ContractId::from_protobuf(contract_id)?;

        let sender_account_id = pb.sender_id.map(AccountId::from_protobuf).transpose()?;

        let evm_address = pb
            .evm_address
            .and_then(|address| <[u8; 20]>::try_from(address).ok())
            .map(ContractId::from);

        Ok(Self {
            contract_id,
            bytes: pb.contract_call_result,
            error_message: if pb.error_message.is_empty() { None } else { Some(pb.error_message) },
            bloom: pb.bloom,
            gas_used: pb.gas_used as u64,
            gas: pb.gas as u64,
            hbar_amount: pb.amount as u64,
            contract_function_parameters_bytes: pb.function_parameters,
            sender_account_id,
            evm_address,
        })
    }
}

impl FromProtobuf<services::response::Response> for ContractFunctionResult {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, ContractCallLocal, services::response::Response);

        let result = pb_getf!(pb, function_result)?;
        let result = ContractFunctionResult::from_protobuf(result)?;

        Ok(result)
    }
}
