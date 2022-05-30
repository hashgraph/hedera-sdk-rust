use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;
use itertools::Itertools;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{Error, FromProtobuf, ToProtobuf};

/// The unique identifier for a file on Hedera.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct FileId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl Debug for FileId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for FileId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
    }
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

impl From<u64> for FileId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0 }
    }
}

impl FromStr for FileId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: share with other entity IDs

        let parts: Vec<u64> =
            s.splitn(3, '.').map(u64::from_str).try_collect().map_err(Error::basic_parse)?;

        if parts.len() == 1 {
            Ok(Self::from(parts[0]))
        } else if parts.len() == 3 {
            Ok(Self { shard: parts[0], realm: parts[1], num: parts[2] })
        } else {
            Err(Error::basic_parse("expecting <shard>.<realm>.<num> (ex. `0.0.1001`)"))
        }
    }
}
