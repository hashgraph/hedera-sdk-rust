/// The unique identifier for a cryptocurrency account on Hedera.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct AccountId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
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

/// Either [`AccountId`] or [`AccountAlias`]. Some transactions and queries
/// accept `AccountIdOrAlias` as an input. All transactions and queries return only `AccountId`
/// as an output however.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum AccountIdOrAlias {
    AccountId(AccountId),
    AccountAlias(AccountAlias),
}
