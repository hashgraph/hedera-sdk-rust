use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};

use serde_with::SerializeDisplay;

#[derive(Eq, PartialEq, Clone, SerializeDisplay)]
pub struct LedgerId(pub(crate) Vec<u8>);

impl LedgerId {
    #[must_use]
    pub fn mainnet() -> Self {
        Self(vec![0])
    }

    #[must_use]
    pub fn testnet() -> Self {
        Self(vec![1])
    }

    #[must_use]
    pub fn previewnet() -> Self {
        Self(vec![2])
    }

    #[must_use]
    pub fn is_mainnet(&self) -> bool {
        self == &Self::mainnet()
    }

    #[must_use]
    pub fn is_testnet(&self) -> bool {
        self == &Self::testnet()
    }

    #[must_use]
    pub fn is_previewnet(&self) -> bool {
        self == &Self::previewnet()
    }

    #[must_use]
    pub fn is_known_network(&self) -> bool {
        self.is_mainnet() || self.is_previewnet() || self.is_testnet()
    }
}

impl Debug for LedgerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for LedgerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&base64::encode(&self.0))
    }
}
