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
use std::hash::{
    Hash,
    Hasher,
};
use std::str::FromStr;

use ed25519_dalek::ed25519::signature::DigestVerifier;
use ed25519_dalek::{
    Digest,
    Verifier,
};
use hedera_proto::services;
use pkcs8::der::{
    Decode,
    Encode,
};
use prost::Message;
use serde_with::{
    DeserializeFromStr,
    SerializeDisplay,
};

use crate::key::private_key::ED25519_OID;
use crate::{
    AccountId,
    Error,
    FromProtobuf,
};

/// A public key on the Hedera network.
#[derive(Clone, Eq, Copy, Hash, PartialEq, SerializeDisplay, DeserializeFromStr)]
pub struct PublicKey(PublicKeyData);

#[derive(Clone, Copy)]
enum PublicKeyData {
    Ed25519(ed25519_dalek::PublicKey),
    EcdsaSecp256k1(k256::ecdsa::VerifyingKey),
}

impl Hash for PublicKeyData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match &self {
            PublicKeyData::Ed25519(key) => key.to_bytes().hash(state),
            PublicKeyData::EcdsaSecp256k1(key) => key.to_bytes().hash(state),
        }
    }
}

impl PartialEq for PublicKeyData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ed25519(l0), Self::Ed25519(r0)) => l0 == r0,
            (Self::EcdsaSecp256k1(l0), Self::EcdsaSecp256k1(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for PublicKeyData {}

impl PublicKey {
    pub(super) fn ed25519(key: ed25519_dalek::PublicKey) -> Self {
        Self(PublicKeyData::Ed25519(key))
    }

    pub(super) fn ecdsa_secp256k1(key: k256::ecdsa::VerifyingKey) -> Self {
        Self(PublicKeyData::EcdsaSecp256k1(key))
    }

    #[must_use]
    pub fn is_ed25519(&self) -> bool {
        matches!(&self.0, PublicKeyData::Ed25519(_))
    }

    #[must_use]
    pub fn is_ecdsa_secp256k1(&self) -> bool {
        matches!(&self.0, PublicKeyData::EcdsaSecp256k1(_))
    }

    pub(crate) fn from_alias_bytes(bytes: &[u8]) -> crate::Result<Option<Self>> {
        if bytes.is_empty() {
            return Ok(None);
        }
        Ok(Some(PublicKey::from_protobuf(
            services::Key::decode(bytes).map_err(Error::from_protobuf)?,
        )?))
    }

    /// Parse a `PublicKey` from a sequence of bytes.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        if bytes.len() == 32 {
            return Self::from_bytes_ed25519(bytes);
        }

        if bytes.len() == 33 {
            return Self::from_bytes_ecdsa_secp256k1(bytes);
        }

        Self::from_bytes_der(bytes)
    }

    /// Parse a Ed25519 `PublicKey` from a sequence of bytes.
    pub fn from_bytes_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 {
            ed25519_dalek::PublicKey::from_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_der(bytes);
        };

        Ok(Self::ed25519(data))
    }

    /// Parse a ECDSA(secp256k1) `PublicKey` from a sequence of bytes.
    pub fn from_bytes_ecdsa_secp256k1(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 33 {
            k256::ecdsa::VerifyingKey::from_sec1_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_der(bytes);
        };

        Ok(Self::ecdsa_secp256k1(data))
    }

    pub fn from_bytes_der(bytes: &[u8]) -> crate::Result<Self> {
        let info = pkcs8::SubjectPublicKeyInfo::from_der(bytes)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        if info.algorithm.oid == k256::elliptic_curve::ALGORITHM_OID {
            return Self::from_bytes_ecdsa_secp256k1(info.subject_public_key);
        }

        if info.algorithm.oid == ED25519_OID {
            return Self::from_bytes_ed25519(info.subject_public_key);
        }

        Err(Error::key_parse(format!("unsupported key algorithm: {}", info.algorithm.oid)))
    }

    pub fn from_str_der(s: &str) -> crate::Result<Self> {
        Self::from_bytes_der(
            &hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?,
        )
    }

    pub fn from_str_ed25519(s: &str) -> crate::Result<Self> {
        Self::from_bytes_ed25519(
            &hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?,
        )
    }

    pub fn from_str_ecdsa_secp256k1(s: &str) -> crate::Result<Self> {
        Self::from_bytes_ecdsa_secp256k1(
            &hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?,
        )
    }

    /// Return this `PublicKey`, serialized as bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.0 {
            PublicKeyData::Ed25519(_) => self.to_bytes_raw(),
            PublicKeyData::EcdsaSecp256k1(_) => self.to_bytes_der(),
        }
    }

    /// Return this `PublicKey`, serialized as bytes.
    // panic should be impossible (`unreachable`)
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn to_bytes_der(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(64);

        match &self.0 {
            PublicKeyData::Ed25519(key) => {
                let key = key.to_bytes();
                let info = pkcs8::SubjectPublicKeyInfo {
                    algorithm: self.algorithm(),
                    subject_public_key: &key,
                };

                info.encode_to_vec(&mut buf).unwrap();
            }

            PublicKeyData::EcdsaSecp256k1(key) => {
                let key = key.to_bytes();
                let info = pkcs8::SubjectPublicKeyInfo {
                    algorithm: self.algorithm(),
                    subject_public_key: key.as_slice(),
                };

                info.encode_to_vec(&mut buf).unwrap();
            }
        }

        buf
    }

    fn algorithm(&self) -> pkcs8::AlgorithmIdentifier<'_> {
        pkcs8::AlgorithmIdentifier {
            parameters: None,
            oid: match self.0 {
                PublicKeyData::Ed25519(_) => ED25519_OID,
                PublicKeyData::EcdsaSecp256k1(_) => k256::elliptic_curve::ALGORITHM_OID,
            },
        }
    }

    pub(crate) fn to_bytes_raw(self) -> Vec<u8> {
        match &self.0 {
            PublicKeyData::Ed25519(key) => key.to_bytes().as_slice().to_vec(),
            PublicKeyData::EcdsaSecp256k1(key) => key.to_bytes().as_slice().to_vec(),
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn to_string_der(&self) -> String {
        self.to_string()
    }

    #[must_use]
    pub fn to_string_raw(&self) -> String {
        hex::encode(self.to_bytes_raw())
    }

    #[must_use]
    pub fn to_account_id(&self, shard: u64, realm: u64) -> AccountId {
        AccountId { shard, realm, alias: Some(*self), num: 0 }
    }

    pub fn verify(&self, msg: &[u8], signature: &crate::Signature) -> crate::Result<()> {
        match &self.0 {
            PublicKeyData::Ed25519(key) => {
                // todo: figure out what the signature actually was if it wasn't ed25519
                // technically it'll always be ecdsa-secp256k1 but that is annoyingly not future proof.
                let signature = signature
                    .as_ed25519()
                    .ok_or_else(|| "Expected Ed25519 signature".to_owned())
                    .map_err(crate::Error::signature_verify)?;

                key.verify(msg, signature).map_err(crate::Error::signature_verify)
            }
            PublicKeyData::EcdsaSecp256k1(key) => {
                // todo: see above comment on ed25519 signatures
                let signature = signature
                    .as_ecdsa_secp256k1()
                    .ok_or_else(|| "Expected Ecdsa-Secp256k1 signature".to_owned())
                    .map_err(crate::Error::signature_verify)?;

                key.verify_digest(sha3::Keccak256::new_with_prefix(msg), signature)
                    .map_err(crate::Error::signature_verify)
            }
        }
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&hex::encode(self.to_bytes_der()))
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(&hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?)
    }
}

impl FromProtobuf<services::Key> for PublicKey {
    fn from_protobuf(pb: services::Key) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::key::Key::*;

        match pb.key {
            Some(Ed25519(bytes)) => PublicKey::from_bytes_ed25519(&bytes),
            Some(ContractId(_)) => {
                Err(Error::from_protobuf("unexpected unsupported Contract ID key in single key"))
            }
            Some(DelegatableContractId(_)) => Err(Error::from_protobuf(
                "unexpected unsupported Delegatable Contract ID key in single key",
            )),
            Some(Rsa3072(_)) => {
                Err(Error::from_protobuf("unexpected unsupported RSA-3072 key in single key"))
            }
            Some(Ecdsa384(_)) => {
                Err(Error::from_protobuf("unexpected unsupported ECDSA-384 key in single key"))
            }
            Some(ThresholdKey(_)) => {
                Err(Error::from_protobuf("unexpected threshold key as single key"))
            }
            Some(KeyList(_)) => Err(Error::from_protobuf("unexpected key list as single key")),
            Some(EcdsaSecp256k1(bytes)) => PublicKey::from_bytes_ecdsa_secp256k1(&bytes),
            None => Err(Error::from_protobuf("unexpected empty key in single key")),
        }
    }
}

// TODO: to_protobuf
// TODO: verify_transaction
