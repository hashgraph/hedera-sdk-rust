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

use crate::evm_address::EvmAddress;
use crate::{
    EntityId,
    Error,
    FromProtobuf,
    ToProtobuf,
};

// TODO: checksum
/// A unique identifier for a smart contract on Hedera.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
pub struct ContractId {
    /// A non-negative number identifying the shard containing this contract instance.
    pub shard: u64,

    /// A non-negative number identifying the realm within the shard containing this contract instance.
    pub realm: u64,

    /// A non-negative number identifying the entity within the realm containing this contract instance.
    ///
    /// Note: Exactly one of `evm_address` and `num` must exist.
    pub num: u64,

    /// EVM address identifying the entity within the realm containing this contract instance.
    ///
    /// Note: Exactly one of `evm_address` and `num` must exist.
    pub evm_address: Option<[u8; 20]>,
}

impl ContractId {
    /// Create a `ContractId` from the given shard/realm/num
    #[must_use]
    pub fn new(shard: u64, realm: u64, num: u64) -> Self {
        Self { shard, realm, num, evm_address: None }
    }

    /// Create a `ContractId` from a `shard.realm.evm_address` set.
    #[must_use]
    pub fn from_evm_address_bytes(shard: u64, realm: u64, evm_address: [u8; 20]) -> Self {
        Self { shard, realm, num: 0, evm_address: Some(evm_address) }
    }

    /// Create a `ContractId` from a `shard.realm.evm_address` set.
    ///
    /// # Errors
    /// [`Error::BasicParse`] if `address` is invalid hex, or the wrong length.
    pub fn from_evm_address(shard: u64, realm: u64, address: &str) -> crate::Result<Self> {
        Ok(Self { shard, realm, num: 0, evm_address: Some(EvmAddress::from_str(address)?.0) })
    }

    /// create a `ContractId` from a solidity address.
    pub fn from_solidity_address(address: &str) -> crate::Result<Self> {
        let EntityId { shard, realm, num } = EntityId::from_solidity_address(address)?;

        Ok(Self { shard, realm, num, evm_address: None })
    }

    /// Create a new `ContractId` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }

    /// Convert `self` into a solidity `address`
    pub fn to_solidity_address(&self) -> crate::Result<String> {
        if let Some(address) = self.evm_address {
            return Ok(hex::encode(&address));
        }

        EntityId { shard: self.shard, realm: self.realm, num: self.num }.to_solidity_address()
    }
}

impl Debug for ContractId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for ContractId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(address) = &self.evm_address {
            write!(f, "{}.{}.{}", self.shard, self.realm, EvmAddress::from_ref(address))
        } else {
            write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
        }
    }
}

impl FromProtobuf<services::ContractId> for ContractId {
    fn from_protobuf(pb: services::ContractId) -> crate::Result<Self> {
        let contract = pb_getf!(pb, contract)?;

        let (num, evm_address) = match contract {
            services::contract_id::Contract::ContractNum(it) => (it as u64, None),
            services::contract_id::Contract::EvmAddress(it) => {
                (0, Some(EvmAddress::try_from(it)?.0))
            }
        };

        Ok(Self { evm_address, num, shard: pb.shard_num as u64, realm: pb.realm_num as u64 })
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

impl From<[u8; 20]> for ContractId {
    fn from(address: [u8; 20]) -> Self {
        Self { shard: 0, realm: 0, num: 0, evm_address: Some(address) }
    }
}

impl From<u64> for ContractId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0, evm_address: None }
    }
}

impl FromStr for ContractId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expecting = || {
            Error::basic_parse(format!(
                "expecting <shard>.<realm>.<num> or <shard>.<realm>.<evm_address>, got `{s}`"
            ))
        };

        let Some((shard, s)) = s.split_once('.') else {
            let num: u64 = s.parse().map_err(Error::basic_parse)?;
            return Ok(num.into())
        };

        let shard = shard.parse().map_err(Error::basic_parse)?;

        let (realm, s) = s.split_once('.').ok_or_else(expecting)?;

        let realm = realm.parse().map_err(Error::basic_parse)?;

        if let Ok(num) = u64::from_str(s) {
            Ok(Self { shard, realm, num, evm_address: None })
        } else if let Ok(evm_address) = hex::decode(s.strip_prefix("0x").unwrap_or(s)) {
            let evm_address = EvmAddress::try_from(evm_address)?.0;

            Ok(Self { shard, realm, num: 0, evm_address: Some(evm_address) })
        } else {
            return Err(expecting());
        }
    }
}
