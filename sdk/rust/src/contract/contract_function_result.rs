use hedera_proto::services;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::base64::Base64;
use serde_with::serde_as;

use crate::{
    AccountId,
    ContractId,
    FromProtobuf,
};

// TODO: log info
// TODO: state_changes
// TODO: evm_address
/// The result returned by a call to a smart contract function.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFunctionResult {
    /// The smart contract instance whose function was called.
    pub contract_id: ContractId,

    /// The result returned by the function.
    #[serde_as(as = "Base64")]
    pub result: Vec<u8>,

    /// Message if there was an error during smart contract execution.
    pub error_message: String,

    /// Bloom filter for record.
    pub bloom: Vec<u8>,

    /// Units of gas used to execute contract.
    pub gas_used: u64,

    /// The amount of gas available for the call.
    pub gas_limit: u64,

    /// Number of tinybars sent (the function must be payable if this is nonzero).
    pub value: u64,

    /// The parameters passed into the contract call.
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,

    /// The account that is the "sender." If not present it is the accountId from the transactionId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<AccountId>,
}

impl FromProtobuf<services::ContractFunctionResult> for ContractFunctionResult {
    fn from_protobuf(pb: services::ContractFunctionResult) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let contract_id = pb_getf!(pb, contract_id)?;
        let contract_id = ContractId::from_protobuf(contract_id)?;

        let sender_id =
            pb.sender_id.map(|sender_id| AccountId::from_protobuf(sender_id)).transpose()?;

        Ok(Self {
            contract_id,
            result: pb.contract_call_result,
            error_message: pb.error_message,
            bloom: pb.bloom,
            gas_used: pb.gas_used as u64,
            gas_limit: pb.gas as u64,
            value: pb.amount as u64,
            data: pb.function_parameters,
            sender_id,
        })
    }
}
