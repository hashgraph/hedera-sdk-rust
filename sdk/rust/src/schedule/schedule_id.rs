use hedera_proto::services;

use crate::FromProtobuf;

/// The unique identifier for a schedule on Hedera.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct ScheduleId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl FromProtobuf for ScheduleId {
    type Protobuf = services::ScheduleId;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            num: pb.schedule_num as u64,
            shard: pb.shard_num as u64,
            realm: pb.realm_num as u64,
        })
    }
}
