use hedera_proto::services;

use crate::protobuf::FromProtobuf;
use crate::ContractId;

/// The log information for an event returned by a smart contract function call.
/// One function call may return several such events.
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct ContractLogInfo {
    /// Address of the contract that emitted the event.
    pub contract_id: ContractId,

    /// Bloom filter for this log.
    pub bloom: Vec<u8>,

    /// A list of topics this log is relevent to.
    pub topics: Vec<Vec<u8>>,

    /// The log's data payload.
    pub data: Vec<u8>,
}

impl FromProtobuf<services::ContractLoginfo> for ContractLogInfo {
    fn from_protobuf(pb: services::ContractLoginfo) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            contract_id: ContractId::from_protobuf(pb_getf!(pb, contract_id)?)?,
            bloom: pb.bloom,
            topics: pb.topic,
            data: pb.data,
        })
    }
}
