/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

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
    /// A non-negative number identifying the shard containing this token.
    pub shard: u64,

    /// A non-negative number identifying the realm within the shard containing this token.
    pub realm: u64,

    /// A non-negative number identifying the entity within the realm containing this token.
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

impl FromProtobuf<services::TokenId> for TokenId {
    fn from_protobuf(pb: services::TokenId) -> crate::Result<Self> {
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
