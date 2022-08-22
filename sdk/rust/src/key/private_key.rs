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

use std::fmt;
use std::fmt::{
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;
use std::sync::Arc;

use ed25519_dalek::{
    Keypair,
    Signer,
};
use k256::pkcs8::der::Encode;
use pkcs8::der::Decode;
use pkcs8::ObjectIdentifier;
use rand::thread_rng;

use crate::{
    Error,
    PublicKey,
    SignaturePair,
};

pub(super) const ED25519_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.101.112");

/// A private key on the Hedera network.
#[derive(Clone)]
pub struct PrivateKey(Arc<PrivateKeyData>);

enum PrivateKeyData {
    Ed25519(ed25519_dalek::Keypair),
    EcdsaSecp256k1(k256::ecdsa::SigningKey),
}

impl PrivateKey {
    /// Generates a new Ed25519 private key.
    #[must_use]
    pub fn generate_ed25519() -> Self {
        let data = ed25519_dalek::Keypair::generate(&mut thread_rng());
        let data = PrivateKeyData::Ed25519(data);

        Self(Arc::new(data))
    }

    /// Generates a new ECDSA(secp256k1) private key.
    #[must_use]
    pub fn generate_ecdsa_secp256k1() -> Self {
        let data = k256::ecdsa::SigningKey::random(&mut thread_rng());
        let data = PrivateKeyData::EcdsaSecp256k1(data);

        Self(Arc::new(data))
    }

    /// Gets the public key which corresponds to this private key.
    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        match &*self.0 {
            PrivateKeyData::Ed25519(key) => PublicKey::ed25519(key.public),
            PrivateKeyData::EcdsaSecp256k1(key) => PublicKey::ecdsa_secp256k1(key.verifying_key()),
        }
    }

    /// Parse a `PrivateKey` from a sequence of bytes.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        if bytes.len() == 32 || bytes.len() == 64 {
            return Self::from_bytes_ed25519(bytes);
        }

        Self::from_bytes_pkcs8_der(bytes)
    }

    /// Parse a Ed25519 `PrivateKey` from a sequence of bytes.
    pub fn from_bytes_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 || bytes.len() == 64 {
            ed25519_dalek::SecretKey::from_bytes(&bytes[..32]).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_pkcs8_der(bytes);
        };

        let data = Keypair { public: (&data).into(), secret: data };

        Ok(Self(Arc::new(PrivateKeyData::Ed25519(data))))
    }

    /// Parse a ECDSA(secp256k1) `PrivateKey` from a sequence of bytes.
    pub fn from_bytes_ecdsa_secp256k1(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 {
            // not DER encoded, raw bytes for key
            k256::ecdsa::SigningKey::from_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_pkcs8_der(bytes);
        };

        Ok(Self(Arc::new(PrivateKeyData::EcdsaSecp256k1(data))))
    }

    fn from_bytes_pkcs8_der(bytes: &[u8]) -> crate::Result<Self> {
        let info = pkcs8::PrivateKeyInfo::from_der(bytes)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        if info.algorithm.oid == k256::elliptic_curve::ALGORITHM_OID {
            return Self::from_bytes_ecdsa_secp256k1(info.private_key);
        }

        if info.algorithm.oid == ED25519_OID {
            return Self::from_bytes_ed25519(info.private_key);
        }

        Err(Error::key_parse(format!("unsupported key algorithm: {}", info.algorithm.oid)))
    }

    /// Return this private key, serialized as bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let info = pkcs8::PrivateKeyInfo {
            algorithm: self.algorithm(),
            private_key: &self.to_bytes_raw(),
            public_key: None,
        };

        let mut buf = Vec::with_capacity(64);
        info.encode_to_vec(&mut buf).unwrap();

        buf
    }

    fn to_bytes_raw(&self) -> [u8; 32] {
        match &*self.0 {
            PrivateKeyData::Ed25519(key) => key.secret.to_bytes(),
            PrivateKeyData::EcdsaSecp256k1(key) => key.to_bytes().into(),
        }
    }

    fn algorithm(&self) -> pkcs8::AlgorithmIdentifier<'_> {
        pkcs8::AlgorithmIdentifier {
            parameters: None,
            oid: match &*self.0 {
                PrivateKeyData::Ed25519(_) => ED25519_OID,
                PrivateKeyData::EcdsaSecp256k1(_) => k256::elliptic_curve::ALGORITHM_OID,
            },
        }
    }

    pub(crate) fn sign(&self, message: &[u8]) -> SignaturePair {
        let public = self.public_key();

        match &*self.0 {
            PrivateKeyData::Ed25519(key) => SignaturePair::ed25519(key.sign(message), public),
            PrivateKeyData::EcdsaSecp256k1(key) => todo!(),
        }
    }
}

impl Debug for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&hex::encode(self.to_bytes()))
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(&hex::decode(s).map_err(Error::key_parse)?)
    }
}

// TODO: from_mnemonic
// TODO: derive (!)
// TODO: legacy_derive (!)
// TODO: sign_message
// TODO: sign_transaction
