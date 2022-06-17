use hedera_proto::services;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

use crate::FromProtobuf;

/// Response from [`ContractBytecodeQuery`][crate::ContractBytecodeQuery].
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractBytecodeResponse {
    /// The runtime bytecode of the contract.
    #[serde_as(as = "Base64")]
    pub bytecode: Vec<u8>,
}

impl FromProtobuf for ContractBytecodeResponse {
    type Protobuf = services::response::Response;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, ContractGetBytecodeResponse, services::response::Response);

        Ok(Self { bytecode: pb.bytecode })
    }
}
