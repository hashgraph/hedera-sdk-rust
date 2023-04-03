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

use std::hash::{
    Hash,
    Hasher,
};

use ed25519_dalek::Verifier as _;
use hmac::digest::generic_array::sequence::Split;
use hmac::digest::generic_array::GenericArray;
use k256::ecdsa;
use k256::ecdsa::signature::DigestVerifier as _;
use pkcs8::der::{
    Decode,
    Encode,
};
use sha2::Digest;

use crate::key::private::ED25519_OID;
use crate::Error;

/// A public key on the Hedera network.
#[derive(Clone, Eq, Copy, Hash, PartialEq)]
pub struct PublicKey(PublicKeyData);

#[derive(Clone, Copy)]
enum PublicKeyData {
    Ed25519(ed25519_dalek::VerifyingKey),
    Ecdsa(k256::ecdsa::VerifyingKey),
}

impl Hash for PublicKeyData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match &self {
            PublicKeyData::Ed25519(key) => key.to_bytes().hash(state),
            PublicKeyData::Ecdsa(key) => key.to_encoded_point(true).as_bytes().hash(state),
        }
    }
}

impl PartialEq for PublicKeyData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ed25519(l0), Self::Ed25519(r0)) => l0 == r0,
            (Self::Ecdsa(l0), Self::Ecdsa(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for PublicKeyData {}

impl PublicKey {
    pub(super) fn ed25519(key: ed25519_dalek::VerifyingKey) -> Self {
        Self(PublicKeyData::Ed25519(key))
    }

    pub(super) fn ecdsa(key: k256::ecdsa::VerifyingKey) -> Self {
        Self(PublicKeyData::Ecdsa(key))
    }

    /// Returns `true` if the public key is `Ed25519`.
    #[must_use]
    pub fn is_ed25519(&self) -> bool {
        matches!(&self.0, PublicKeyData::Ed25519(_))
    }

    /// Returns `true` if the public key data is `Ecdsa`.
    #[must_use]
    pub fn is_ecdsa(&self) -> bool {
        matches!(&self.0, PublicKeyData::Ecdsa(_))
    }

    /// Parse a `PublicKey` from a sequence of bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PublicKey`.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        if bytes.len() == 32 {
            return Self::from_bytes_ed25519(bytes);
        }

        if bytes.len() == 33 {
            return Self::from_bytes_ecdsa(bytes);
        }

        Self::from_bytes_der(bytes)
    }

    /// Parse a Ed25519 `PublicKey` from a sequence of bytes.   
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ed25519 `PublicKey`.
    pub fn from_bytes_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        let data = if let Ok(bytes) = bytes.try_into() {
            ed25519_dalek::VerifyingKey::from_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_der(bytes);
        };

        Ok(Self::ed25519(data))
    }

    /// Parse a ECDSA(secp256k1) `PublicKey` from a sequence of bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ECDSA(secp256k1) `PublicKey`.
    pub fn from_bytes_ecdsa(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 33 {
            k256::ecdsa::VerifyingKey::from_sec1_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_der(bytes);
        };

        Ok(Self::ecdsa(data))
    }

    /// Parse a `PublicKey` from a sequence of der encoded bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PublicKey`.
    pub fn from_bytes_der(bytes: &[u8]) -> crate::Result<Self> {
        let info = pkcs8::SubjectPublicKeyInfo::from_der(bytes)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        if info.algorithm.oid == k256::elliptic_curve::ALGORITHM_OID {
            return Self::from_bytes_ecdsa(info.subject_public_key);
        }

        if info.algorithm.oid == ED25519_OID {
            return Self::from_bytes_ed25519(info.subject_public_key);
        }

        Err(Error::key_parse(format!("unsupported key algorithm: {}", info.algorithm.oid)))
    }

    /// Return this `PublicKey`, serialized as bytes.
    ///
    /// If this is an ed25519 public key, this is equivalent to [`to_bytes_raw`](Self::to_bytes_raw)
    /// If this is an ecdsa public key, this is equivalent to [`to_bytes_der`](Self::to_bytes_der)
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.0 {
            PublicKeyData::Ed25519(_) => self.to_bytes_raw(),
            PublicKeyData::Ecdsa(_) => self.to_bytes_der(),
        }
    }

    /// Return this `PublicKey`, serialized as der-encoded bytes.
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

            PublicKeyData::Ecdsa(key) => {
                let key = key.to_encoded_point(true);
                let info = pkcs8::SubjectPublicKeyInfo {
                    algorithm: self.algorithm(),
                    subject_public_key: key.as_bytes(),
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
                PublicKeyData::Ecdsa(_) => k256::elliptic_curve::ALGORITHM_OID,
            },
        }
    }

    /// Return this `PublicKey`, serialized as bytes.
    #[must_use]
    pub fn to_bytes_raw(&self) -> Vec<u8> {
        match &self.0 {
            PublicKeyData::Ed25519(key) => key.to_bytes().as_slice().to_vec(),
            PublicKeyData::Ecdsa(key) => key.to_encoded_point(true).to_bytes().into_vec(),
        }
    }

    /// Convert this public key into an evm address.
    /// The EVM address is This is the rightmost 20 bytes of the 32 byte Keccak-256 hash of the ECDSA public key.
    ///
    /// Returns `Some(evm_address)` if `self.is_ecdsa`, otherwise `None`.
    #[must_use]
    pub fn to_evm_address(&self) -> Option<Vec<u8>> {
        if let PublicKeyData::Ecdsa(ecdsa_key) = &self.0 {
            // we specifically want the uncompressed form ...
            let encoded_point = ecdsa_key.to_encoded_point(false);
            let bytes = encoded_point.as_bytes();
            // ... and without the tag (04):
            let bytes = &bytes[1..];
            let hash = sha3::Keccak256::digest(bytes);

            let (_, sliced): (GenericArray<u8, hmac::digest::typenum::U12>, _) = hash.split();

            let sliced: [u8; 20] = sliced.into();
            Some(sliced.as_slice().to_vec())
        } else {
            None
        }
    }

    /// Verify a `signature` on a `msg` with this public key.
    ///
    /// # Errors
    /// - [`Error::SignatureVerify`] if the signature algorithm doesn't match this `PublicKey`.
    /// - [`Error::SignatureVerify`] if the signature is invalid for this `PublicKey`.
    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> crate::Result<()> {
        match &self.0 {
            PublicKeyData::Ed25519(key) => {
                let signature = ed25519_dalek::Signature::try_from(signature)
                    .map_err(Error::signature_verify)?;

                key.verify(msg, &signature).map_err(Error::signature_verify)
            }
            PublicKeyData::Ecdsa(key) => {
                // todo: see above comment on ed25519 signatures

                let signature =
                    ecdsa::Signature::try_from(signature).map_err(Error::signature_verify)?;

                key.verify_digest(sha3::Keccak256::new_with_prefix(msg), &signature)
                    .map_err(Error::signature_verify)
            }
        }
    }
}
