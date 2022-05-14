use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;
use itertools::Itertools;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{Error, FromProtobuf, PublicKey, ToProtobuf};

/// The unique identifier for a cryptocurrency account on Hedera.
#[derive(SerializeDisplay, DeserializeFromStr, Copy, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct AccountId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl Debug for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
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
        let account = pb_getf!(pb, account)?;
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

/// The identifier for a cryptocurrency account represented with an alias instead of an
/// account number.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone)]
pub struct AccountAlias {
    pub shard: u64,
    pub realm: u64,
    pub alias: PublicKey,
}

impl Debug for AccountAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for AccountAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.shard, self.realm, self.alias)
    }
}

impl ToProtobuf for AccountAlias {
    type Protobuf = services::AccountId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountId {
            account: Some(services::account_id::Account::Alias(self.alias.as_bytes_raw().to_vec())),
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

impl From<PublicKey> for AccountAlias {
    fn from(alias: PublicKey) -> Self {
        Self { shard: 0, realm: 0, alias }
    }
}

impl FromStr for AccountAlias {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(3, '.').collect();

        if parts.len() == 1 {
            Ok(Self::from(PublicKey::from_str(parts[0])?))
        } else if parts.len() == 3 {
            let shard = parts[0].parse().map_err(Error::basic_parse)?;
            let realm = parts[1].parse().map_err(Error::basic_parse)?;
            let alias = parts[2].parse().map_err(Error::basic_parse)?;

            Ok(Self { shard, realm, alias })
        } else {
            Err(Error::basic_parse("expecting <shard>.<realm>.<alias> (ex. `0.0.0a410c8fe4912e3652b61dd222b1b4d7773261537d7ebad59df6cd33622a693e`)"))
        }
    }
}

/// Either [`AccountId`] or [`AccountAlias`]. Some transactions and queries
/// accept `AccountIdOrAlias` as an input. All transactions and queries return only `AccountId`
/// as an output however.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone)]
pub enum AccountIdOrAlias {
    AccountId(AccountId),
    AccountAlias(AccountAlias),
}

impl Debug for AccountIdOrAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for AccountIdOrAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountId(id) => Display::fmt(id, f),
            Self::AccountAlias(id) => Display::fmt(id, f),
        }
    }
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AccountId::from_str(s)
            .map(Self::AccountId)
            .or_else(|_| AccountAlias::from_str(s).map(Self::AccountAlias))
            .map_err(|_| Error::basic_parse("expecting <shard>.<realm>.<num> (ex. `0.0.1001`) or <shard>.<realm>.<alias> (ex. `0.0.0a410c8fe4912e3652b61dd222b1b4d7773261537d7ebad59df6cd33622a693e`)"))
    }
}
