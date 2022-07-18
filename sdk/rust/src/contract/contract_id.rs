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
    FromProtobuf,
    ToProtobuf,
};

/// The unique identifier for a smart contract on Hedera.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct ContractId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl Debug for ContractId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for ContractId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
    }
}

impl FromProtobuf<services::ContractId> for ContractId {
    fn from_protobuf(pb: services::ContractId) -> crate::Result<Self> {
        let contract = pb_getf!(pb, contract)?;
        let num = pb_getv!(contract, ContractNum, services::contract_id::Contract);

        Ok(Self { num: num as u64, shard: pb.shard_num as u64, realm: pb.realm_num as u64 })
    }
}

impl ToProtobuf for ContractId {
    type Protobuf = services::ContractId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::ContractId {
            contract: Some(services::contract_id::Contract::ContractNum(self.num as i64)),
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

impl From<u64> for ContractId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0 }
    }
}

impl FromStr for ContractId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|EntityId { shard, realm, num }| Self { shard, realm, num })
    }
}

/// The identifier for a smart contract represented with an EVM address instead of a
/// contract number.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ContractEvmAddress {
    pub shard: u64,
    pub realm: u64,
    pub address: [u8; 20],
}

impl ToProtobuf for ContractEvmAddress {
    type Protobuf = services::ContractId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::ContractId {
            contract: Some(services::contract_id::Contract::EvmAddress(self.address.to_vec())),
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

/// Either [`ContractId`] or [`ContractEvmAddress`].
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ContractAddress {
    ContractId(ContractId),
    ContractEvmAddress(ContractEvmAddress),
}

impl ToProtobuf for ContractAddress {
    type Protobuf = services::ContractId;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            Self::ContractId(id) => id.to_protobuf(),
            Self::ContractEvmAddress(address) => address.to_protobuf(),
        }
    }
}
