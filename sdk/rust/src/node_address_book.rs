use hedera_proto::services;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::NodeAddress;

#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct NodeAddressBook {
    pub addresses: Vec<NodeAddress>,
}

impl NodeAddressBook {
    /// Create a new `NodeAddressBook` from protobuf-encoded `bytes`.
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

impl FromProtobuf<services::NodeAddressBook> for NodeAddressBook {
    fn from_protobuf(pb: services::NodeAddressBook) -> crate::Result<Self> {
        Ok(Self { addresses: Vec::from_protobuf(pb.node_address)? })
    }
}

impl ToProtobuf for NodeAddressBook {
    type Protobuf = services::NodeAddressBook;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::NodeAddressBook { node_address: self.addresses.to_protobuf() }
    }
}
