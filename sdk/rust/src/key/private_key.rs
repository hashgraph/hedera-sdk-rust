use std::str::FromStr;

use ed25519_dalek::Signer;
use rand::thread_rng;

use crate::key::public_key::PublicKeyData;
use crate::{Error, PublicKey, SignaturePair};

/// A private key on the Hedera network.
pub struct PrivateKey(PrivateKeyData);

enum PrivateKeyData {
    Ed25519(ed25519_dalek::Keypair),
    // TODO: Ecdsa(_)
}

impl Clone for PrivateKey {
    fn clone(&self) -> Self {
        Self(match &self.0 {
            PrivateKeyData::Ed25519(key) => PrivateKeyData::Ed25519(
                ed25519_dalek::Keypair::from_bytes(&key.to_bytes()).unwrap(),
            ),
        })
    }
}

impl PrivateKey {
    /// Generates a new Ed25519 private key.
    pub fn generate_ed25519() -> Self {
        Self(PrivateKeyData::Ed25519(ed25519_dalek::Keypair::generate(&mut thread_rng())))
    }

    /// Return the public key, derived from this private key.
    pub fn public_key(&self) -> PublicKey {
        match &self.0 {
            PrivateKeyData::Ed25519(key) => PublicKey(PublicKeyData::Ed25519(key.public)),
        }
    }

    pub(crate) fn from_bytes_raw_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        let secret = ed25519_dalek::SecretKey::from_bytes(bytes).map_err(Error::key_parse)?;
        let public = ed25519_dalek::PublicKey::from(&secret);
        let key = ed25519_dalek::Keypair { public, secret };

        Ok(Self(PrivateKeyData::Ed25519(key)))
    }

    pub(crate) fn sign(&self, message: &[u8]) -> SignaturePair {
        let public = self.public_key();

        match &self.0 {
            PrivateKeyData::Ed25519(key) => SignaturePair::ed25519(key.sign(message), public),
        }
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: handle DER-prefixed
        // TODO: handle private+public
        // TODO: handle ecdsa

        Self::from_bytes_raw_ed25519(&hex::decode(s).map_err(Error::key_parse)?)
    }
}

// TODO: generate_ecdsa()
// TODO: from_mnemonic
// TODO: from_str
// TODO: from_bytes
// TODO: derive (!)
// TODO: legacy_derive (!)
// TODO: sign_message
// TODO: sign_transaction
