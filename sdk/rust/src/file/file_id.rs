use hedera_proto::services;

use crate::{FromProtobuf, ToProtobuf};

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

impl ToProtobuf for FileId {
    type Protobuf = services::FileId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::FileId {
            file_num: self.num as i64,
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}
