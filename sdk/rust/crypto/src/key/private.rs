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

use std::sync::Arc;

use ed25519_dalek::Signer;
use hmac::{
    Hmac,
    Mac,
};
use k256::ecdsa::signature::DigestSigner;
use k256::pkcs8::der::Encode;
use pkcs8::der::Decode;
use pkcs8::{
    AssociatedOid,
    ObjectIdentifier,
};
use sha2::Sha512;
use sha3::Digest;

use crate::{
    Error,
    PublicKey,
};

// replace with `array::split_array_ref` when that's stable.
fn split_key_array(arr: &[u8; 64]) -> (&[u8; 32], &[u8; 32]) {
    let (lhs, rhs) = arr.split_at(32);

    // SAFETY: lhs points to [T; N]? Yes it's [T] of length 64/2 (guaranteed by split_at)
    let lhs = unsafe { &*(lhs.as_ptr().cast::<[u8; 32]>()) };
    // SAFETY: rhs points to [T; N]? Yes it's [T] of length 64/2 (rhs.len() = 64 - lhs.len(), lhs.len() has been proven to be 32 above...)
    let rhs = unsafe { &*(rhs.as_ptr().cast::<[u8; 32]>()) };

    (lhs, rhs)
}

pub(super) const ED25519_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.101.112");

/// A private key on the Hedera network.
#[derive(Clone)]
pub struct PrivateKey(Arc<PrivateKeyDataWrapper>);

// find a better name
struct PrivateKeyDataWrapper {
    data: PrivateKeyData,
    chain_code: Option<[u8; 32]>,
}

impl PrivateKeyDataWrapper {
    fn new(inner: PrivateKeyData) -> Self {
        Self { data: inner, chain_code: None }
    }

    fn new_derivable(inner: PrivateKeyData, chain_code: [u8; 32]) -> Self {
        Self { data: inner, chain_code: Some(chain_code) }
    }
}

enum PrivateKeyData {
    Ed25519(ed25519_dalek::SigningKey),
    Ecdsa(k256::ecdsa::SigningKey),
}

impl PrivateKey {
    #[must_use]
    pub fn generate_ed25519() -> Self {
        use rand::Rng as _;

        let mut csprng = rand::thread_rng();

        let data = ed25519_dalek::SigningKey::generate(&mut csprng);
        let data = PrivateKeyData::Ed25519(data);

        let mut chain_code = [0u8; 32];
        csprng.fill(&mut chain_code);

        Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, chain_code)))
    }

    #[must_use]
    pub fn generate_ecdsa() -> Self {
        let data = k256::ecdsa::SigningKey::random(&mut rand::thread_rng());
        let data = PrivateKeyData::Ecdsa(data);

        Self(Arc::new(PrivateKeyDataWrapper::new(data)))
    }

    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => PublicKey::ed25519(key.verifying_key()),
            PrivateKeyData::Ecdsa(key) => PublicKey::ecdsa(*key.verifying_key()),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        if bytes.len() == 32 || bytes.len() == 64 {
            return Self::from_bytes_ed25519(bytes);
        }

        Self::from_bytes_der(bytes)
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn from_bytes_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 || bytes.len() == 64 {
            ed25519_dalek::SigningKey::from_bytes(&bytes[..32].try_into().unwrap())
        } else {
            return Self::from_bytes_der(bytes);
        };

        Ok(Self(Arc::new(PrivateKeyDataWrapper::new(PrivateKeyData::Ed25519(data)))))
    }

    pub fn from_bytes_ecdsa(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 {
            // not DER encoded, raw bytes for key
            k256::ecdsa::SigningKey::from_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_der(bytes);
        };

        Ok(Self(Arc::new(PrivateKeyDataWrapper::new(PrivateKeyData::Ecdsa(data)))))
    }

    pub fn from_bytes_der(bytes: &[u8]) -> crate::Result<Self> {
        let info = pkcs8::PrivateKeyInfo::from_der(bytes)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        // PrivateKey is an `OctetString`, and the `PrivateKey`s we all support are `OctetStrings`.
        // So, we, awkwardly, have an `OctetString` containing an `OctetString` containing our key material.
        let inner = pkcs8::der::asn1::OctetStringRef::from_der(info.private_key)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        let inner = inner.as_bytes();

        if info.algorithm.oid == k256::Secp256k1::OID {
            return Self::from_bytes_ecdsa(inner);
        }

        if info.algorithm.oid == ED25519_OID {
            return Self::from_bytes_ed25519(inner);
        }

        Err(Error::key_parse(format!("unsupported key algorithm: {}", info.algorithm.oid)))
    }

    pub(crate) fn from_encrypted_info(der: &[u8], password: &[u8]) -> crate::Result<Self> {
        let info = pkcs8::EncryptedPrivateKeyInfo::from_der(der)
            .map_err(|e| Error::key_parse(e.to_string()))?;

        let decrypted = info.decrypt(password).map_err(|e| Error::key_parse(e.to_string()))?;

        PrivateKey::from_bytes_der(decrypted.as_bytes())
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn to_bytes_der(&self) -> Vec<u8> {
        let mut inner = Vec::with_capacity(34);

        pkcs8::der::asn1::OctetStringRef::new(&self.to_bytes_raw_internal())
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

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.0.data {
            PrivateKeyData::Ed25519(_) => self.to_bytes_raw(),
            PrivateKeyData::Ecdsa(_) => self.to_bytes_der(),
        }
    }

    /// Return this `PrivateKey`, serialized as bytes.
    #[must_use]
    pub fn to_bytes_raw(&self) -> Vec<u8> {
        self.to_bytes_raw_internal().as_slice().to_vec()
    }

    #[must_use]
    fn to_bytes_raw_internal(&self) -> [u8; 32] {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => key.to_bytes(),
            PrivateKeyData::Ecdsa(key) => key.to_bytes().into(),
        }
    }

    fn algorithm(&self) -> pkcs8::AlgorithmIdentifier<'_> {
        pkcs8::AlgorithmIdentifier {
            parameters: None,
            oid: match &self.0.data {
                PrivateKeyData::Ed25519(_) => ED25519_OID,
                PrivateKeyData::Ecdsa(_) => k256::Secp256k1::OID,
            },
        }
    }

    #[must_use]
    pub fn is_ed25519(&self) -> bool {
        matches!(self.0.data, PrivateKeyData::Ed25519(_))
    }

    #[must_use]
    pub fn is_ecdsa(&self) -> bool {
        matches!(self.0.data, PrivateKeyData::Ecdsa(_))
    }

    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => key.sign(message).to_bytes().as_slice().to_vec(),
            PrivateKeyData::Ecdsa(key) => {
                let signature: k256::ecdsa::Signature =
                    key.sign_digest(sha3::Keccak256::new_with_prefix(message));

                signature.to_vec()
            }
        }
    }

    #[must_use]
    pub fn is_derivable(&self) -> bool {
        self.is_ed25519() && self.0.chain_code.is_some()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn derive(&self, index: i32) -> crate::Result<Self> {
        const HARDEND_MASK: u32 = 1 << 31;
        let index = index as u32;

        let chain_code =
            self.0.chain_code.as_ref().ok_or_else(|| Error::key_derive("key is underivable"))?;

        match &self.0.data {
            PrivateKeyData::Ed25519(key) => {
                // force hardened.
                let index = index | HARDEND_MASK;

                let output: [u8; 64] = Hmac::<Sha512>::new_from_slice(chain_code)
                    .expect("HMAC can take keys of any size")
                    .chain_update([0])
                    .chain_update(key.to_bytes())
                    .chain_update(index.to_be_bytes())
                    .finalize()
                    .into_bytes()
                    .into();

                // todo: use `split_array_ref` when that's stable.
                let (data, chain_code) = split_key_array(&output);

                let data = ed25519_dalek::SigningKey::from_bytes(data);
                let data = PrivateKeyData::Ed25519(data);

                Ok(Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, *chain_code))))
            }
            PrivateKeyData::Ecdsa(_) => {
                Err(Error::key_derive("Ecdsa private keys don't currently support derivation"))
            }
        }
    }

    pub fn legacy_derive(&self, index: i64) -> crate::Result<Self> {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => {
                let entropy = key.to_bytes();
                let mut seed = Vec::with_capacity(entropy.len() + 8);

                seed.extend_from_slice(&entropy);

                let i1: i32 = match index {
                    0x00ff_ffff_ffff => 0xff,
                    0.. => 0,
                    _ => -1,
                };

                let i2 = index as u8;

                seed.extend_from_slice(&i1.to_be_bytes());
                // any better way to do this?
                seed.extend_from_slice(&[i2, i2, i2, i2]);

                let salt: Vec<u8> = vec![0xff];

                let mut mat = [0; 32];

                pbkdf2::pbkdf2::<Hmac<Sha512>>(&seed, &salt, 2048, &mut mat);

                // note: this shouldn't fail, but there isn't an infaliable conversion.
                Self::from_bytes_ed25519(&mat)
            }

            PrivateKeyData::Ecdsa(_) => {
                Err(Error::key_derive("Ecdsa private keys don't currently support derivation"))
            }
        }
    }

    pub(crate) fn from_mnemonic_seed(seed: &[u8]) -> Self {
        let output: [u8; 64] = Hmac::<Sha512>::new_from_slice(b"ed25519 seed")
            .expect("hmac can take a seed of any size")
            .chain_update(seed)
            .finalize()
            .into_bytes()
            .into();

        // todo: use `split_array_ref` when that's stable.
        let (left, right) = {
            let (left, right) = output.split_at(32);
            let left: [u8; 32] = left.try_into().unwrap();
            let right: [u8; 32] = right.try_into().unwrap();
            (left, right)
        };

        let data = ed25519_dalek::SigningKey::from_bytes(&left);
        let data = PrivateKeyData::Ed25519(data);

        let mut key = Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, right)));

        for index in [44, 3030, 0, 0] {
            key = key.derive(index).expect("BUG: we set the chain code earlier in this function");
        }

        key
    }
}
