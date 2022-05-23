use hedera_proto::services;

use crate::FromProtobuf;

/// The unique identifier for a file on Hedera.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct FileId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl FromProtobuf for FileId {
    type Protobuf = services::FileId;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self { num: pb.file_num as u64, shard: pb.shard_num as u64, realm: pb.realm_num as u64 })
    }
}
