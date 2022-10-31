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

impl From<[u8; 20]> for ContractId {
    fn from(address: [u8; 20]) -> Self {
        Self { num: 0, shard: 0, realm: 0, evm_address: Some(address) }
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
