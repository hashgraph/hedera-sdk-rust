use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;

use crate::{entity_id, FromProtobuf, ToProtobuf};

/// The unique identifier for a schedule on Hedera.
#[derive(serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct ScheduleId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl Debug for ScheduleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for ScheduleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
    }
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

impl ToProtobuf for ScheduleId {
    type Protobuf = services::ScheduleId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::ScheduleId {
            schedule_num: self.num as i64,
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

impl From<u64> for ScheduleId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0 }
    }
}

impl FromStr for ScheduleId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        entity_id::parse(s).map(|(shard, realm, num)| Self { shard, realm, num })
    }
}
