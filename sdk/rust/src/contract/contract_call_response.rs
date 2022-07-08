use hedera_proto::services;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    ContractFunctionResult,
    FromProtobuf,
};

/// Response from [`ContractCallQuery`][crate::ContractCallQuery].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCallResponse {
    /// The value returned by the function.
    pub result: ContractFunctionResult,
}

impl FromProtobuf for ContractCallResponse {
    type Protobuf = services::response::Response;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, ContractCallLocal, services::response::Response);

        let result = pb_getf!(pb, function_result)?;
        let result = ContractFunctionResult::from_protobuf(result)?;

        Ok(Self { result })
    }
}
