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
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;
use std::sync::Arc;
use std::{
    fmt,
    iter,
};

use ed25519_dalek::{
    Keypair,
    Signer,
};
use hmac::Hmac;
use k256::pkcs8::der::Encode;
use pkcs8::der::Decode;
use pkcs8::{
    AssociatedOid,
    ObjectIdentifier,
};
use rand::thread_rng;
use sha2::Sha512;

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

        // PrivateKey is an `OctetString`, and the private keys we all support are `OctetStrings`.
        // So, we, awkwardly, have an `OctetString` containing an `OctetString` containing our key material.
        let inner = pkcs8::der::asn1::OctetStringRef::from_der(info.private_key)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        let inner = inner.as_bytes();

        if info.algorithm.oid == k256::Secp256k1::OID {
            return Self::from_bytes_ecdsa_secp256k1(inner);
        }

        if info.algorithm.oid == ED25519_OID {
            return Self::from_bytes_ed25519(inner);
        }

        Err(Error::key_parse(format!("unsupported key algorithm: {}", info.algorithm.oid)))
    }

    /// Return this private key, serialized as bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut inner = Vec::with_capacity(34);

        pkcs8::der::asn1::OctetStringRef::new(&self.to_bytes_raw())
            .unwrap()
            .encode_to_vec(&mut inner)
            .unwrap();

        let info = pkcs8::PrivateKeyInfo {
            algorithm: self.algorithm(),
            private_key: &inner,
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
                PrivateKeyData::EcdsaSecp256k1(_) => k256::Secp256k1::OID,
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

    // todo: what do we do about i32?
    // It's basically just a cast to support them, but, unlike Java, operator overloading doesn't exist.
    /// Derive a private key based on the `index`.
    // ⚠️ unaudited cryptography ⚠️
    pub fn legacy_derive(&self, index: i64) -> crate::Result<Self> {
        match &*self.0 {
            PrivateKeyData::Ed25519(key) => {
                let entropy = key.secret.as_bytes();
                let mut seed = Vec::with_capacity(entropy.len() + 8);

                seed.extend_from_slice(&*entropy);

                let i1: i32 = match index {
                    // fixme: this exact case is untested.
                    0xffffffffff => 0xff,
                    0.. => 0,
                    _ => -1,
                };

                let i2 = index as u8;

                seed.extend_from_slice(&i1.to_be_bytes());
                // any better way to do this?
                seed.extend(iter::repeat(i2).take(4));

                let salt: Vec<u8> = vec![0xff];

                let mut mat = [0; 32];

                pbkdf2::pbkdf2::<Hmac<Sha512>>(&seed, &salt, 2048, &mut mat);

                // note: this shouldn't fail, but there isn't an infaliable conversion.
                Self::from_bytes_ed25519(&mat)
            }

            // need to add an error variant, key derivation doesn't exist for Ecdsa keys in Java impl.
            PrivateKeyData::EcdsaSecp256k1(_) => todo!(),
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
// TODO: legacy_derive (!) - k256
// TODO: sign_message
// TODO: sign_transaction

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pkcs8::AssociatedOid;

    use super::{
        PrivateKey,
        ED25519_OID,
    };

    #[test]
    fn ed25519_from_str() {
        const S: &str = "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312";
        let pk = PrivateKey::from_str(S).unwrap();

        assert_eq!(pk.algorithm().oid, ED25519_OID);

        assert_eq!(pk.to_string(), S);
    }

    #[test]
    fn ecdsa_secp_256_k1_from_str() {
        const S: &str = "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048";
        let pk = PrivateKey::from_str(S).unwrap();

        assert_eq!(pk.algorithm().oid, k256::Secp256k1::OID);

        assert_eq!(pk.to_string(), S);
    }

    #[test]
    fn ed25519_legacy_derive() {
        // private key was lifted from a Mnemonic test.
        let private_key = PrivateKey::from_str(
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312",
        )
        .unwrap();

        let private_key_0 = private_key.legacy_derive(0).unwrap();

        assert_eq!(private_key_0.to_string(), "302e020100300506032b6570042204202b7345f302a10c2a6d55bf8b7af40f125ec41d780957826006d30776f0c441fb");

        let private_key_neg_1 = private_key.legacy_derive(-1).unwrap();

        assert_eq!(private_key_neg_1.to_string(), "302e020100300506032b657004220420caffc03fdb9853e6a91a5b3c57a5c0031d164ce1c464dea88f3114786b5199e5");
    }
}
