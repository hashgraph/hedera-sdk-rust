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

use hedera_proto::services;

use crate::{
    ContractId,
    Error,
    FromProtobuf,
    KeyList,
    PublicKey,
    ToProtobuf,
};

/// Any method that can be used to authorize an operation on Hedera.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub enum Key {
    // todo(sr): not happy with any of these (fix before merge)
    /// A single public key.
    Single(PublicKey),

    /// A contract ID.
    ContractId(ContractId),

    /// A delegatable contract ID.
    DelegatableContractId(ContractId),

    /// A key list.
    KeyList(KeyList),
}

impl Key {
    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
}

impl ToProtobuf for Key {
    type Protobuf = services::Key;

    fn to_protobuf(&self) -> Self::Protobuf {
        use services::key::Key::*;

        services::Key {
            key: Some(match self {
                Self::Single(key) => {
                    let bytes = key.to_bytes_raw();

                    match key.kind() {
                        crate::key::KeyKind::Ed25519 => Ed25519(bytes),
                        crate::key::KeyKind::Ecdsa => EcdsaSecp256k1(bytes),
                    }
                }

                Self::ContractId(id) => ContractId(id.to_protobuf()),
                Self::DelegatableContractId(id) => DelegatableContractId(id.to_protobuf()),
                // `KeyList`s are special and can be both a key list and a threshold key.
                Self::KeyList(key) => key.to_protobuf_key(),
            }),
        }
    }
}

impl From<PublicKey> for Key {
    fn from(key: PublicKey) -> Self {
        Self::Single(key)
    }
}

impl From<ContractId> for Key {
    fn from(id: ContractId) -> Self {
        Self::ContractId(id)
    }
}

impl From<KeyList> for Key {
    fn from(value: KeyList) -> Self {
        Self::KeyList(value)
    }
}

impl FromProtobuf<services::Key> for Key {
    fn from_protobuf(pb: services::Key) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::key::Key::*;

        match pb.key {
            Some(Ed25519(bytes)) => Ok(Self::Single(PublicKey::from_bytes_ed25519(&bytes)?)),
            Some(ContractId(id)) => Ok(Self::ContractId(crate::ContractId::from_protobuf(id)?)),
            Some(DelegatableContractId(id)) => {
                Ok(Self::DelegatableContractId(crate::ContractId::from_protobuf(id)?))
            }
            Some(Rsa3072(_)) => {
                Err(Error::from_protobuf("unexpected unsupported RSA-3072 key in Key"))
            }
            Some(Ecdsa384(_)) => {
                Err(Error::from_protobuf("unexpected unsupported ECDSA-384 key in Key"))
            }
            Some(ThresholdKey(it)) => Ok(Self::KeyList(crate::KeyList::from_protobuf(it)?)),
            Some(KeyList(it)) => Ok(Self::KeyList(crate::KeyList::from_protobuf(it)?)),
            Some(EcdsaSecp256k1(bytes)) => Ok(Self::Single(PublicKey::from_bytes_ecdsa(&bytes)?)),
            None => Err(Error::from_protobuf("unexpected empty key in Key")),
        }
    }
}
