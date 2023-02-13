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
use hedera_proto::services::account_id::Account;

use crate::entity_id::{
    Checksum,
    PartialEntityId,
    ValidateChecksums,
};
use crate::evm_address::{
    EvmAddress,
    IdEvmAddress,
};
use crate::{
    Client,
    EntityId,
    Error,
    FromProtobuf,
    LedgerId,
    PublicKey,
    ToProtobuf,
};

/// A unique identifier for a cryptocurrency account on Hedera.
#[derive(Copy, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
pub struct AccountId {
    /// A non-negative number identifying the shard containing this account.
    pub shard: u64,

    /// A non-negative number identifying the realm within the shard containing this account.
    pub realm: u64,

    /// A non-negative number identifying the entity within the realm containing this account.
    pub num: u64,

    /// An alias for `num` if the account was created from a public key directly.
    pub alias: Option<PublicKey>,

    /// The last 20 bytes of the keccak-256 hash of a ECDSA_SECP256K1 primitive key.
    pub evm_address: Option<EvmAddress>,

    /// A checksum if the account ID was read from a user inputted string which inclueded a checksum
    pub checksum: Option<Checksum>,
}

impl AccountId {
    /// Create a new `AccountId` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`] if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`] if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Create an `AccountId` from a solidity address.
    pub fn from_solidity_address(address: &str) -> crate::Result<Self> {
        let EntityId { shard, realm, num, checksum } = EntityId::from_solidity_address(address)?;

        Ok(Self { shard, realm, num, alias: None, evm_address: None, checksum })
    }

    /// Create an `AccountId` from an evm address.
    ///
    /// Accepts "0x___" Ethereum public address.
    #[must_use]
    pub fn from_evm_address(address: &EvmAddress) -> Self {
        Self {
            shard: 0,
            realm: 0,
            num: 0,
            alias: None,
            evm_address: Some(*address),
            checksum: None,
        }
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }

    /// Convert `self` into a solidity `address`
    pub fn to_solidity_address(&self) -> crate::Result<String> {
        EntityId { shard: self.shard, realm: self.realm, num: self.num, checksum: None }
            .to_solidity_address()
    }

    // todo: note: the specifics of this function are undecided (should it even exist?)
    // followup discussion required as per https://github.com/hashgraph/hedera-sdk-reference/issues/70.
    /// Returns "0x_____" string Ethereum public address.
    pub(crate) fn _to_evm_address(&self) -> crate::Result<String> {
        if let Some(evm_address) = &self.evm_address {
            Ok(evm_address.to_string())
        } else {
            Err(Error::NoEvmAddressPresent { task: "convert account ID to evm address string" })
        }
    }

    /// Convert `self` to a string with a valid checksum.
    pub async fn to_string_with_checksum(&self, client: &Client) -> Result<String, Error> {
        if self.alias.is_some() || self.evm_address.is_some() {
            Err(Error::CannotToStringWithChecksum)
        } else {
            EntityId::to_string_with_checksum(self.to_string(), client).await
        }
    }

    /// If this account ID was constructed from a user input string, it might include a checksum.
    ///
    /// This function will validate that the checksum is correct, returning an `Err()` result containing an
    /// [`Error::BadEntityId`](crate::Error::BadEntityId) if it's invalid, and a `Some(())` result if it is valid.
    ///
    /// If no checksum is present, validation will silently pass (the function will return `Some(())`)
    pub async fn validate_checksum(&self, client: &Client) -> Result<(), Error> {
        if self.alias.is_some() || self.evm_address.is_some() {
            Ok(())
        } else {
            EntityId::validate_checksum(self.shard, self.realm, self.num, &self.checksum, client)
                .await
        }
    }
}

impl ValidateChecksums for AccountId {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        if self.alias.is_some() || self.evm_address.is_some() {
            Ok(())
        } else {
            EntityId::validate_checksum_for_ledger_id(
                self.shard,
                self.realm,
                self.num,
                &self.checksum,
                ledger_id,
            )
        }
    }
}

impl Debug for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for AccountId {
    // allowed because `alias` would go before `shard` and `realm` and create a confusing reading experience:
    // `write!(f, "{}.{}.{alias}", self.shard, self.realm);`
    #[allow(clippy::uninlined_format_args)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(alias) = &self.alias {
            write!(f, "{}.{}.{}", self.shard, self.realm, alias)
        } else if let Some(evm_address) = &self.evm_address {
            write!(f, "{evm_address}")
        } else {
            write!(f, "{}.{}.{}", self.shard, self.realm, self.num)
        }
    }
}

impl ToProtobuf for AccountId {
    type Protobuf = services::AccountId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountId {
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
            account: Some(match &self.alias {
                None => services::account_id::Account::AccountNum(self.num as i64),
                Some(alias) => services::account_id::Account::Alias(ToProtobuf::to_bytes(alias)),
            }),
        }
    }
}

impl FromProtobuf<services::AccountId> for AccountId {
    fn from_protobuf(pb: services::AccountId) -> crate::Result<Self> {
        let account = pb_getf!(pb, account)?;

        let (num, alias, evm_address) = match account {
            services::account_id::Account::AccountNum(num) => (num, None, None),
            services::account_id::Account::Alias(alias) => {
                (0, Some(FromProtobuf::from_bytes(&alias)?), None)
            }
            Account::EvmAddress(evm_address) => {
                (0, None, Some(IdEvmAddress::try_from(evm_address)?.0))
            }
        };

        Ok(Self {
            num: num as u64,
            alias,
            evm_address,
            shard: pb.shard_num as u64,
            realm: pb.realm_num as u64,
            checksum: None,
        })
    }
}

impl From<u64> for AccountId {
    fn from(num: u64) -> Self {
        Self { num, alias: None, evm_address: None, checksum: None, shard: 0, realm: 0 }
    }
}

impl From<PublicKey> for AccountId {
    fn from(alias: PublicKey) -> Self {
        Self { num: 0, shard: 0, realm: 0, evm_address: None, alias: Some(alias), checksum: None }
    }
}

impl FromStr for AccountId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // override the error message for better context.
        let partial = PartialEntityId::from_str(s)
        .map_err(|_| Error::basic_parse("expecting <shard>.<realm>.<num> (ex. `0.0.1001`) or <shard>.<realm>.<alias> (ex. `0.0.0a410c8fe4912e3652b61dd222b1b4d7773261537d7ebad59df6cd33622a693e"))?;

        match partial {
            PartialEntityId::ShortNum(it) => Ok(it.into()),
            PartialEntityId::LongNum(it) => Ok(it.into()),

            // 0x<evm_address>
            PartialEntityId::ShortOther(evm_address) => {
                Ok(Self::from_evm_address(&evm_address.parse()?))
            }

            // <shard>.<realm>.<alias>
            PartialEntityId::LongOther { shard, realm, last } => Ok(Self {
                shard,
                realm,
                num: 0,
                alias: Some(last.parse()?),
                evm_address: None,
                checksum: None,
            }),
        }
    }
}

impl From<EntityId> for AccountId {
    fn from(value: EntityId) -> Self {
        let EntityId { shard, realm, num, checksum } = value;
        Self { shard, realm, num, checksum, alias: None, evm_address: None }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use hex_literal::hex;

    use crate::evm_address::EvmAddress;
    use crate::{
        AccountId,
        Client,
    };

    #[test]
    fn parse() {
        let account_id: AccountId = "0.0.1001".parse().unwrap();

        assert_eq!(
            account_id,
            AccountId {
                shard: 0,
                realm: 0,
                num: 1001,
                alias: None,
                evm_address: None,
                checksum: None
            }
        );
    }

    #[test]
    fn to_from_bytes_roundtrip() {
        let account_id = AccountId {
            shard: 0,
            realm: 0,
            num: 1001,
            alias: None,
            evm_address: None,
            checksum: None,
        };

        assert_eq!(account_id, AccountId::from_bytes(&account_id.to_bytes()).unwrap());
    }

    #[test]
    fn from_evm_address_string() {
        let evm_address = hex!("302a300506032b6570032100114e6abc371b82da");
        assert_eq!(
            AccountId::from_str("0x302a300506032b6570032100114e6abc371b82da").unwrap(),
            AccountId {
                shard: 0,
                realm: 0,
                num: 0,
                alias: None,
                evm_address: Some(EvmAddress(evm_address)),
                checksum: None
            }
        )
    }

    #[test]
    fn to_evm_address_string() {
        assert_eq!(
            &AccountId {
                shard: 0,
                realm: 0,
                num: 0,
                alias: None,
                evm_address: Some(EvmAddress(hex!("302a300506032b6570032100114e6abc371b82da"))),
                checksum: None
            }
            .to_string(),
            "0x302a300506032b6570032100114e6abc371b82da"
        )
    }

    #[tokio::test]
    async fn good_checksum_on_mainnet() {
        AccountId::from_str("0.0.123-vfmkw")
            .unwrap()
            .validate_checksum(&Client::for_mainnet())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn good_checksum_on_testnet() {
        AccountId::from_str("0.0.123-esxsf")
            .unwrap()
            .validate_checksum(&Client::for_testnet())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn good_checksum_on_previewnet() {
        AccountId::from_str("0.0.123-ogizo")
            .unwrap()
            .validate_checksum(&Client::for_previewnet())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn to_string_with_checksum() {
        assert_eq!(
            AccountId::from_str("0.0.123")
                .unwrap()
                .to_string_with_checksum(&Client::for_testnet())
                .await
                .unwrap(),
            "0.0.123-esxsf"
        );
    }
}
