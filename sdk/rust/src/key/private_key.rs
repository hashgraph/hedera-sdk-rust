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
use rand::{
    thread_rng,
    Rng,
};
use sha2::Sha512;
use sha3::Digest;

use crate::{
    Error,
    PublicKey,
    SignaturePair,
};

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

// for usage in tests (provides a way to snapshot test)
impl Debug for PrivateKeyDataWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        enum Algorithm {
            Ed25519,
            EcdsaSecp256k1,
        }

        let (algorithm, key) = match &self.data {
            PrivateKeyData::Ed25519(key) => {
                (Algorithm::Ed25519, hex::encode(key.secret.as_bytes()))
            }

            PrivateKeyData::EcdsaSecp256k1(key) => {
                (Algorithm::EcdsaSecp256k1, hex::encode(&key.to_bytes()))
            }
        };

        f.debug_struct("PrivateKeyData")
            .field("algorithm", &algorithm)
            .field("key", &key)
            .field("chain_code", &self.chain_code.as_ref().map(hex::encode))
            .finish()
    }
}

enum PrivateKeyData {
    Ed25519(ed25519_dalek::Keypair),
    EcdsaSecp256k1(k256::ecdsa::SigningKey),
}

impl PrivateKey {
    #[cfg(test)]
    pub(crate) fn debug_pretty(&self) -> &impl Debug {
        &*self.0
    }

    /// Generates a new Ed25519 private key.
    #[must_use]
    pub fn generate_ed25519() -> Self {
        let mut csprng = thread_rng();
        let data = ed25519_dalek::Keypair::generate(&mut csprng);
        let data = PrivateKeyData::Ed25519(data);

        let mut chain_code = [0u8; 32];
        csprng.fill(&mut chain_code);

        Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, chain_code)))
    }

    /// Generates a new ECDSA(secp256k1) private key.
    #[must_use]
    pub fn generate_ecdsa_secp256k1() -> Self {
        let data = k256::ecdsa::SigningKey::random(&mut thread_rng());
        let data = PrivateKeyData::EcdsaSecp256k1(data);

        Self(Arc::new(PrivateKeyDataWrapper::new(data)))
    }

    /// Gets the public key which corresponds to this private key.
    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        match &self.0.data {
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

        Ok(Self(Arc::new(PrivateKeyDataWrapper::new(PrivateKeyData::Ed25519(data)))))
    }

    /// Parse a ECDSA(secp256k1) `PrivateKey` from a sequence of bytes.
    pub fn from_bytes_ecdsa_secp256k1(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 {
            // not DER encoded, raw bytes for key
            k256::ecdsa::SigningKey::from_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_pkcs8_der(bytes);
        };

        Ok(Self(Arc::new(PrivateKeyDataWrapper::new(PrivateKeyData::EcdsaSecp256k1(data)))))
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
    // panic should be impossible (`unreachable`)
    #[allow(clippy::missing_panics_doc)]
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
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => key.secret.to_bytes(),
            PrivateKeyData::EcdsaSecp256k1(key) => key.to_bytes().into(),
        }
    }

    fn algorithm(&self) -> pkcs8::AlgorithmIdentifier<'_> {
        pkcs8::AlgorithmIdentifier {
            parameters: None,
            oid: match &self.0.data {
                PrivateKeyData::Ed25519(_) => ED25519_OID,
                PrivateKeyData::EcdsaSecp256k1(_) => k256::Secp256k1::OID,
            },
        }
    }

    pub(crate) fn sign(&self, message: &[u8]) -> SignaturePair {
        let public = self.public_key();

        match &self.0.data {
            PrivateKeyData::Ed25519(key) => SignaturePair::ed25519(key.sign(message), public),
            PrivateKeyData::EcdsaSecp256k1(key) => SignaturePair::ecdsa_secp256k1(
                key.sign_digest(sha3::Keccak256::new_with_prefix(message)),
                public,
            ),
        }
    }

    /// Derives a child key based on `index`.
    pub fn derive(&self, index: i32) -> crate::Result<Self> {
        const HARDEND_MASK: u32 = 1 << 31;
        let index = index as u32;

        let chain_code = match &self.0.chain_code {
            Some(chain_code) => chain_code,
            // Key is not derivable
            None => todo!(),
        };

        match &self.0.data {
            PrivateKeyData::Ed25519(key) => {
                // force hardened.
                let index = index | HARDEND_MASK;

                let output: [u8; 64] = Hmac::<Sha512>::new_from_slice(chain_code)
                    .expect("HMAC can take keys of any size")
                    .chain_update([0])
                    .chain_update(key.secret.as_bytes())
                    .chain_update(index.to_be_bytes())
                    .finalize()
                    .into_bytes()
                    .into();

                // todo: use `split_array_ref` when that's stable.
                let (left, right) = output.split_at(32);

                // this is exactly 32 bytes
                let chain_code: [u8; 32] = right.try_into().unwrap();

                let data = ed25519_dalek::SecretKey::from_bytes(left).unwrap();
                let data = Keypair { public: (&data).into(), secret: data };
                let data = PrivateKeyData::Ed25519(data);

                Ok(Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, chain_code))))
            }
            PrivateKeyData::EcdsaSecp256k1(_) => todo!(),
        }
    }

    // todo: what do we do about i32?
    // It's basically just a cast to support them, but, unlike Java, operator overloading doesn't exist.
    /// Derive a private key based on the `index`.
    // ⚠️ unaudited cryptography ⚠️
    pub fn legacy_derive(&self, index: i64) -> crate::Result<Self> {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => {
                let entropy = key.secret.as_bytes();
                let mut seed = Vec::with_capacity(entropy.len() + 8);

                seed.extend_from_slice(entropy);

                let i1: i32 = match index {
                    0x00ff_ffff_ffff => 0xff,
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

    /// Recover a private key from a generated mnemonic phrase and a passphrase.
    // this is specifically for the two `try_into`s which depend on `split_array_ref`.
    // panic should be impossible (`unreachable`)
    #[allow(clippy::missing_panics_doc)]
    pub fn from_mnemonic(
        mnemonic: &crate::Mnemonic,
        passphrase: &str,
    ) -> Result<PrivateKey, Error> {
        let seed = mnemonic.to_seed(passphrase);

        let output: [u8; 64] = Hmac::<Sha512>::new_from_slice(b"ed25519 seed")
            .expect("hmac can take a seed of any size")
            .chain_update(&seed)
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

        let data = ed25519_dalek::SecretKey::from_bytes(&left).unwrap();
        let data = ed25519_dalek::Keypair { public: (&data).into(), secret: data };
        let data = PrivateKeyData::Ed25519(data);

        let mut key = Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, right)));

        for index in [44, 3030, 0, 0] {
            key = key.derive(index)?;
        }

        Ok(key)
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

// TODO: derive (!) - secp256k1
// TODO: legacy_derive (!) - secp256k1
// TODO: sign_transaction

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;

    use expect_test::expect;
    use hex_literal::hex;
    use pkcs8::AssociatedOid;

    use super::{
        PrivateKey,
        PrivateKeyDataWrapper,
    };

    #[test]
    fn ed25519_from_str() {
        const S: &str = "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312";
        let pk = PrivateKey::from_str(S).unwrap();
        // ensure round-tripping works.
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
    fn ed25519_sign() {
        let private_key = PrivateKey::from_str(
            "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10",
        )
        .unwrap();

        let signature = private_key.sign(b"hello, world").signature;
        expect![[r#"
            Signature::Ed25519(9d04bfed7baa97c80d29a6ae48c0d896ce8463a7ea0c16197d55a563c73996ef062b2adf507f416c108422c0310fc6fb21886e11ce3de3e951d7a56049743f07)
        "#]]
        .assert_debug_eq(&signature);
    }

    #[test]
    fn ecdsa_secp_256_k1_sign() {
        let private_key = PrivateKey::from_str(
            "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048"
        )
        .unwrap();

        // notice that this doesn't match other impls
        // this is to avoid signature malleability.
        // see: https://github.com/bitcoin/bips/blob/43da5dec5eaf0d8194baa66ba3dd976f923f9d07/bip-0032.mediawiki
        let signature = private_key.sign(b"hello world").signature;
        expect![[r#"
            Signature::EcdsaSecp256k1(f3a13a555f1f8cd6532716b8f388bd4e9d8ed0b252743e923114c0c6cbfe414c086e3717a6502c3edff6130d34df252fb94b6f662d0cd27e2110903320563851)
        "#]]
        .assert_debug_eq(&signature);
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

    #[test]
    fn ed25519_legacy_derive_2() {
        let private_key = PrivateKey::from_str(
            "302e020100300506032b65700422042000c2f59212cb3417f0ee0d38e7bd876810d04f2dd2cb5c2d8f26ff406573f2bd",
        )
        .unwrap();

        let private_key_mhw = private_key.legacy_derive(0xffffffffff).unwrap();

        assert_eq!(private_key_mhw.to_string(), "302e020100300506032b6570042204206890dc311754ce9d3fc36bdf83301aa1c8f2556e035a6d0d13c2cccdbbab1242")
    }

    /// This is for testing purposes only.
    ///
    /// # Panics
    /// If `data` and `chain_code` don't make a valid [`PrivateKey`].
    fn key_with_chain(data: &str, chain_code: [u8; 32]) -> PrivateKey {
        let key_without_chain = PrivateKey::from_str(data).unwrap();

        let data = match Arc::try_unwrap(key_without_chain.0) {
            Ok(it) => it.data,
            Err(_) => unreachable!(),
        };

        PrivateKey(Arc::new(PrivateKeyDataWrapper::new_derivable(data, chain_code)))
    }

    // "iosKey"
    #[test]
    fn ed25519_derive_1() {
        // have to create a private key with a chain code, which means... Hacks!
        // luckily, we're a unit test, so, we can access private fields.
        let key = key_with_chain(
            "302e020100300506032b657004220420a6b9548d7e123ad4c8bc6fee58301e9b96360000df9d03785c07b620569e7728",
            hex!("cde7f535264f1db4e2ded409396f8c72f8075cc43757bd5a205c97699ea40271"),
        );

        let child_key = key.derive(0).unwrap();

        expect![[r#"
            PrivateKeyData {
                algorithm: Ed25519,
                key: "5f66a51931e8c99089472e0d70516b6272b94dd772b967f8221e1077f966dbda",
                chain_code: Some(
                    "0e5c869c1cf9daecd03edb2d49cf2621412578a352578a4bb7ef4eef2942b7c9",
                ),
            }
        "#]]
        .assert_debug_eq(&*child_key.0);
    }

    // "androidKey"
    #[test]
    fn ed25519_derive_2() {
        let key = key_with_chain(
            "302e020100300506032b65700422042097dbce1988ef8caf5cf0fd13a5374969e2be5f50650abd19314db6b32f96f18e",
            hex!("b7b406314eb2224f172c1907fe39f807e306655e81f2b3bc4766486f42ef1433")
        );
        let child_key = key.derive(0).unwrap();

        expect![[r#"
            PrivateKeyData {
                algorithm: Ed25519,
                key: "c284c25b3a1458b59423bc289e83703b125c8eefec4d5aa1b393c2beb9f2bae6",
                chain_code: Some(
                    "a7a1c2d115a988e51efc12c23692188a4796b312a4a700d6c703e4de4cf1a7f6",
                ),
            }
        "#]]
        .assert_debug_eq(&child_key.0);
    }
}
