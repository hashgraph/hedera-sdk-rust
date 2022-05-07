use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;

use crate::{Error, FromProtobuf, ToProtobuf};

/// The unique identifier for a cryptocurrency account on Hedera.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct AccountId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl Display for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
    }
}

impl ToProtobuf for AccountId {
    type Protobuf = services::AccountId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountId {
            account: Some(services::account_id::Account::AccountNum(self.num as i64)),
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

impl FromProtobuf for AccountId {
    type Protobuf = services::AccountId;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        let account = pb_getf!(pb, account, "AccountId")?;
        let num = pb_getv!(account, AccountNum, services::account_id::Account);

        Ok(Self { num: num as u64, shard: pb.shard_num as u64, realm: pb.realm_num as u64 })
    }
}

impl From<u64> for AccountId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0 }
    }
}

impl FromStr for AccountId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// The identifier for a cryptocurrency account represented with an alias instead of an
/// account number.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct AccountAlias {
    pub shard: u64,
    pub realm: u64,

    // TODO: Use hedera::PublicKey instead of Vec<u8>
    pub alias: Vec<u8>,
}

impl ToProtobuf for AccountAlias {
    type Protobuf = services::AccountId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountId {
            account: Some(services::account_id::Account::Alias(self.alias.clone())),
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

// TODO: From<PublicKey> for AccountAlias

impl FromStr for AccountAlias {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Either [`AccountId`] or [`AccountAlias`]. Some transactions and queries
/// accept `AccountIdOrAlias` as an input. All transactions and queries return only `AccountId`
/// as an output however.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum AccountIdOrAlias {
    AccountId(AccountId),
    AccountAlias(AccountAlias),
}

impl ToProtobuf for AccountIdOrAlias {
    type Protobuf = services::AccountId;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            Self::AccountId(id) => id.to_protobuf(),
            Self::AccountAlias(alias) => alias.to_protobuf(),
        }
    }
}

impl From<AccountId> for AccountIdOrAlias {
    fn from(id: AccountId) -> Self {
        Self::AccountId(id)
    }
}

impl From<AccountAlias> for AccountIdOrAlias {
    fn from(alias: AccountAlias) -> Self {
        Self::AccountAlias(alias)
    }
}

impl FromStr for AccountIdOrAlias {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
