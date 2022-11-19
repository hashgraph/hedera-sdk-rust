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

use crate::{
    EntityId,
    Error,
    FromProtobuf,
    PublicKey,
    ToProtobuf,
};

// TODO: checksum
/// A unique identifier for a cryptocurrency account on Hedera.
#[derive(Copy, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
pub struct AccountId {
    /// A non-negative number identifying the shard containing this account.
    pub shard: u64,

    /// A non-negative number identifying the realm within the shard containing this account.
    pub realm: u64,

    /// A non-negative number identifying the entity within the realm containing this account.
    pub num: u64,

    /// An alias for `num` if the account was created from a public key directly.
    pub alias: Option<PublicKey>,
}

impl AccountId {
    /// Create a new `AccountId` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`] if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`] if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
}

impl Debug for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for AccountId {
    // allowed because `alias` would go before `shard` and `realm` and create a confusing reading experience:
    // `write!(f, "{}.{}.{alias}", self.shard, self.realm);`
    #[allow(clippy::uninlined_format_args)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(alias) = &self.alias {
            write!(f, "{}.{}.{}", self.shard, self.realm, alias)
        } else {
            write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
        }
    }
}

impl ToProtobuf for AccountId {
    type Protobuf = services::AccountId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountId {
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
            account: Some(match &self.alias {
                None => services::account_id::Account::AccountNum(self.num as i64),
                Some(alias) => services::account_id::Account::Alias(alias.to_bytes_raw()),
            }),
        }
    }
}

impl FromProtobuf<services::AccountId> for AccountId {
    fn from_protobuf(pb: services::AccountId) -> crate::Result<Self> {
        let account = pb_getf!(pb, account)?;

        let (num, alias) = match account {
            services::account_id::Account::AccountNum(num) => (num, None),
            services::account_id::Account::Alias(alias) => {
                (0, Some(PublicKey::from_bytes(&alias)?))
            }
        };

        Ok(Self { num: num as u64, alias, shard: pb.shard_num as u64, realm: pb.realm_num as u64 })
    }
}

impl From<u64> for AccountId {
    fn from(num: u64) -> Self {
        Self { num, alias: None, shard: 0, realm: 0 }
    }
}

impl From<PublicKey> for AccountId {
    fn from(alias: PublicKey) -> Self {
        Self { num: 0, shard: 0, realm: 0, alias: Some(alias) }
    }
}

impl FromStr for AccountId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        account_num_from_str(s)
            .or_else(|| account_alias_from_str(s))
            .ok_or_else(|| Error::basic_parse("expecting <shard>.<realm>.<num> (ex. `0.0.1001`) or <shard>.<realm>.<alias> (ex. `0.0.0a410c8fe4912e3652b61dd222b1b4d7773261537d7ebad59df6cd33622a693e`)"))
    }
}

fn account_num_from_str(s: &str) -> Option<AccountId> {
    s.parse()
        .map(|EntityId { shard, realm, num }| AccountId { shard, realm, num, alias: None })
        .ok()
}

fn account_alias_from_str(s: &str) -> Option<AccountId> {
    let parts: Vec<&str> = s.splitn(3, '.').collect();

    if parts.len() == 1 {
        Some(AccountId::from(PublicKey::from_str(parts[0]).ok()?))
    } else if parts.len() == 3 {
        let shard = parts[0].parse().map_err(Error::basic_parse).ok()?;
        let realm = parts[1].parse().map_err(Error::basic_parse).ok()?;
        let alias = parts[2].parse().map_err(Error::basic_parse).ok()?;

        Some(AccountId { shard, realm, alias: Some(alias), num: 0 })
    } else {
        None
    }
}
