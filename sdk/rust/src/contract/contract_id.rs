use hedera_proto::services;

use crate::{FromProtobuf, ToProtobuf};

/// The unique identifier for a smart contract on Hedera.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct ContractId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
}

impl FromProtobuf for ContractId {
    type Protobuf = services::ContractId;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        let account = pb_getf!(pb, contract)?;
        let num = pb_getv!(account, ContractNum, services::contract_id::Contract);

        Ok(Self { num: num as u64, shard: pb.shard_num as u64, realm: pb.realm_num as u64 })
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

/// Either [`ContractId`] or [`ContractEvmAddress`].
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ContractIdOrEvmAddress {
    ContractId(ContractId),
    ContractEvmAddress(ContractEvmAddress),
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

impl ToProtobuf for ContractIdOrEvmAddress {
    type Protobuf = services::ContractId;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            Self::ContractId(id) => id.to_protobuf(),
            Self::ContractEvmAddress(address) => address.to_protobuf(),
        }
    }
}
