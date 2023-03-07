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

use crate::entity_id::{
    format,
    ValidateChecksums,
};
use crate::{
    Checksum,
    Client,
    EntityId,
    Error,
    FromProtobuf,
    LedgerId,
    NftId,
    ToProtobuf,
};

/// The unique identifier for a token on Hedera.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
#[repr(C)]
pub struct TokenId {
    /// A non-negative number identifying the shard containing this token.
    pub shard: u64,

    /// A non-negative number identifying the realm within the shard containing this token.
    pub realm: u64,

    /// A non-negative number identifying the entity within the realm containing this token.
    pub num: u64,

    /// A checksum if the token ID was read from a user inputted string which inclueded a checksum
    pub checksum: Option<Checksum>,
}

impl TokenId {
    /// Create a `TokenId` from the given `shard`, `realm`, and `num`.
    #[must_use]
    pub fn new(shard: u64, realm: u64, num: u64) -> Self {
        Self { shard, realm, num, checksum: None }
    }

    /// Create a new `TokenId` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Create a `TokenId` from a solidity address.
    ///
    /// # Errors
    /// - [`Error::BasicParse`] if `address` cannot be parsed as a solidity address.
    pub fn from_solidity_address(address: &str) -> crate::Result<Self> {
        let EntityId { shard, realm, num, checksum } = EntityId::from_solidity_address(address)?;

        Ok(Self { shard, realm, num, checksum })
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }

    /// Convert `self` into a solidity `address`.
    ///
    /// # Errors
    /// - [`Error::BasicParse`] if `self.shard` is larger than `u32::MAX`.
    pub fn to_solidity_address(&self) -> crate::Result<String> {
        EntityId { shard: self.shard, realm: self.realm, num: self.num, checksum: None }
            .to_solidity_address()
    }

    /// Convert `self` to a string with a valid checksum.
    ///
    /// # Errors
    /// - [`Error::CannotPerformTaskWithoutLedgerId`] if the client has no ledger ID. This may become a panic in a future (breaking) release.
    pub fn to_string_with_checksum(&self, client: &Client) -> Result<String, Error> {
        EntityId::new(self.shard, self.realm, self.num).to_string_with_checksum(client)
    }

    /// Validates `self.checksum` (if it exists) for `client`.
    ///
    /// # Errors
    /// - [`Error::CannotPerformTaskWithoutLedgerId`] if the client has no `ledger_id`.
    /// - [`Error::BadEntityId`] if there is a checksum, and the checksum is not valid for the client's `ledger_id`.
    pub fn validate_checksum(&self, client: &Client) -> Result<(), Error> {
        EntityId::validate_checksum(self.shard, self.realm, self.num, self.checksum, client)
    }

    /// Create an NFT ID
    #[must_use]
    pub fn nft(&self, serial: u64) -> NftId {
        NftId { token_id: *self, serial }
    }
}

impl ValidateChecksums for TokenId {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        EntityId::validate_checksum_for_ledger_id(
            self.shard,
            self.realm,
            self.num,
            self.checksum,
            ledger_id,
        )
    }
}

impl Debug for TokenId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for TokenId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format::display(f, self.shard, self.realm, self.num)
    }
}

impl FromProtobuf<services::TokenId> for TokenId {
    fn from_protobuf(pb: services::TokenId) -> crate::Result<Self> {
        Ok(Self {
            num: pb.token_num as u64,
            shard: pb.shard_num as u64,
            realm: pb.realm_num as u64,
            checksum: None,
        })
    }
}

impl ToProtobuf for TokenId {
    type Protobuf = services::TokenId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenId {
            token_num: self.num as i64,
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
        }
    }
}

impl From<u64> for TokenId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0, checksum: None }
    }
}

impl FromStr for TokenId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EntityId::from_str(s).map(Self::from)
    }
}

impl From<EntityId> for TokenId {
    fn from(value: EntityId) -> Self {
        let EntityId { shard, realm, num, checksum } = value;

        Self { shard, realm, num, checksum }
    }
}
