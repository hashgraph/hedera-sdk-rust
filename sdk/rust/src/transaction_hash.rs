use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};

use serde_with::SerializeDisplay;
use sha2::{
    Digest,
    Sha384,
};

#[derive(Copy, Clone, Hash, SerializeDisplay)]
pub struct TransactionHash(pub [u8; 48]);

impl TransactionHash {
    #[must_use]
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self(Sha384::digest(&bytes).into())
    }
}

impl Debug for TransactionHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for TransactionHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&hex::encode(&self.0))
    }
}
