use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use hedera_proto::services;
use serde_with::{
    DeserializeFromStr,
    SerializeDisplay,
};

use crate::{
    EntityId,
    Error,
    FromProtobuf,
    ToProtobuf,
};

/// The unique identifier for a token on Hedera.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct TokenId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl Debug for TokenId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for TokenId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
    }
}

impl FromProtobuf for TokenId {
    type Protobuf = services::TokenId;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            num: pb.token_num as u64,
            shard: pb.shard_num as u64,
            realm: pb.realm_num as u64,
        })
    }
}

impl ToProtobuf for TokenId {
    type Protobuf = services::TokenId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenId {
            token_num: self.num as i64,
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

impl From<u64> for TokenId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0 }
    }
}

impl FromStr for TokenId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|EntityId { shard, realm, num }| Self { shard, realm, num })
    }
}
