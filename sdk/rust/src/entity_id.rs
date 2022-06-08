use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use itertools::Itertools;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::Error;

#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
pub struct EntityId {
    /// The shard number (non-negative).
    pub shard: u64,

    /// The realm number (non-negative).
    pub realm: u64,

    pub num: u64,
}

impl Debug for EntityId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
    }
}

impl From<u64> for EntityId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0 }
    }
}

impl FromStr for EntityId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
