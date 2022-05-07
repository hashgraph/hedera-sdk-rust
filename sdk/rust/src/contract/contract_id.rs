/// The unique identifier for a smart contract on Hedera.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct ContractId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

/// The identifier for a smart contract represented with an EVM address instead of a
/// contract number.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ContractEvmAddress {
    pub shard: u64,
    pub realm: u64,
    pub address: [u8; 20],
}

/// Either [`ContractId`] or [`ContractEvmAddress`].
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ContractIdOrEvmAddress {
    ContractId(ContractId),
    ContractEvmAddress(ContractEvmAddress),
}
