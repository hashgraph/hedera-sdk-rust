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

// TODO: checksum
/// The unique identifier for a smart contract on Hedera.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ContractId {
    pub shard: u64,
    pub realm: u64,
    pub num: u64,
    pub evm_address: Option<[u8; 20]>,
}

impl Debug for ContractId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for ContractId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(address) = &self.evm_address {
            write!(f, "{}.{}.{}", self.shard, self.realm, hex::encode(address))
        } else {
            write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
        }
    }
}

impl FromProtobuf<services::ContractId> for ContractId {
    fn from_protobuf(pb: services::ContractId) -> crate::Result<Self> {
        let contract = pb_getf!(pb, contract)?;
        let num = pb_getv!(contract, ContractNum, services::contract_id::Contract);

        // TODO: handle incoming EVM address !!

        Ok(Self {
            evm_address: None,
            num: num as u64,
            shard: pb.shard_num as u64,
            realm: pb.realm_num as u64,
        })
    }
}

impl ToProtobuf for ContractId {
    type Protobuf = services::ContractId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::ContractId {
            contract: Some(match &self.evm_address {
                Some(address) => services::contract_id::Contract::EvmAddress(address.to_vec()),
                None => services::contract_id::Contract::ContractNum(self.num as i64),
            }),
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

impl From<u64> for ContractId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0, evm_address: None }
    }
}

impl FromStr for ContractId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Handle EVM Address !!

        s.parse().map(|EntityId { shard, realm, num }| Self {
            shard,
            realm,
            num,
            evm_address: None,
        })
    }
}
