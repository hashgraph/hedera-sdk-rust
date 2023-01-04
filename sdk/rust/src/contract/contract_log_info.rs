use hedera_proto::services;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
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
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    pub bloom: Vec<u8>,

    /// A list of topics this log is relevent to.
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<Vec<serde_with::base64::Base64>>"))]
    pub topics: Vec<Vec<u8>>,

    /// The log's data payload.
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    pub data: Vec<u8>,
}

impl ContractLogInfo {
    /// Create a new `ContractLogInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
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

impl ToProtobuf for ContractLogInfo {
    type Protobuf = services::ContractLoginfo;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            contract_id: Some(self.contract_id.to_protobuf()),
            bloom: self.bloom.clone(),
            topic: self.topics.clone(),
            data: self.data.clone(),
        }
    }
}
