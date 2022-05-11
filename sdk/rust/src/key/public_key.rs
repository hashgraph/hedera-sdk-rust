use std::hash::{Hash, Hasher};
use std::str::FromStr;

use crate::Error;

/// A public key on the Hedera network.
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PublicKey(pub(crate) PublicKeyData);

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) enum PublicKeyData {
    Ed25519(ed25519_dalek::PublicKey),
    // TODO: Ecdsa(_)
}

impl PublicKey {
    pub(crate) fn from_bytes_raw_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        Ok(Self(PublicKeyData::Ed25519(
            ed25519_dalek::PublicKey::from_bytes(bytes).map_err(Error::key_parse)?,
        )))
    }

    pub(crate) fn as_bytes_raw(&self) -> &[u8] {
        match &self.0 {
            PublicKeyData::Ed25519(key) => key.as_bytes(),
            // TODO: ecdsa
        }
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes_raw().hash(state);
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: handle DER-prefixed
        // TODO: handle ecdsa

        Self::from_bytes_raw_ed25519(&hex::decode(s).map_err(Error::key_parse)?)
    }
}

// impl ToProtobuf for PublicKey {
//     type Protobuf = services::Key;
//
//     fn to_protobuf(&self) -> Self::Protobuf {
//     }
// }

// TODO: from_protobuf
// TODO: to_protobuf
// TODO: from_bytes_ecdsa
// TODO: from_bytes
// TODO: from_str
// TODO: verify_message
// TODO: verify_transaction
// TODO: to_bytes
// TODO: is_ed25519
// TODO: is_ecsda
