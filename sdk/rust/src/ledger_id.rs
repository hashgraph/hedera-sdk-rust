use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use crate::Error;

// todo: use `Box<[u8]>` (16 bytes on cpus with 64 bit pointers)
// or use an enum of `Mainnet`, `Testnet`, `Previewnet`, `Other`, and `Static`, don't expose the enum though.
// `Other` would be `Box<[u8]>`, but nothing else would have an alloc, the whole struct would be 24 bytes on x86-64,
// wouldn't allocate 99.99% of the time, and could be const constructable in 99.999% of cases.
/// The ID of a Hedera Ledger.
#[derive(Eq, PartialEq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
pub struct LedgerId(Vec<u8>);

impl LedgerId {
    /// ID for the `mainnet` ledger.
    #[must_use]
    pub fn mainnet() -> Self {
        Self(vec![0])
    }

    /// ID for the `testnet` ledger.
    #[must_use]
    pub fn testnet() -> Self {
        Self(vec![1])
    }

    /// ID for the `previewnet` ledger.
    #[must_use]
    pub fn previewnet() -> Self {
        Self(vec![2])
    }

    /// Create a ledger ID from the given bytes.
    #[must_use]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Returns `true` if `self` is `mainnet`.
    #[must_use]
    pub fn is_mainnet(&self) -> bool {
        self == &Self::mainnet()
    }

    /// Returns `true` if `self` is `testnet`.
    #[must_use]
    pub fn is_testnet(&self) -> bool {
        self == &Self::testnet()
    }

    /// Returns `true` if `self` is `previewnet`.
    #[must_use]
    pub fn is_previewnet(&self) -> bool {
        self == &Self::previewnet()
    }

    /// Returns `true` if `self` is `mainnet`, `testnet`, or `previewnet`.
    #[must_use]
    pub fn is_known_network(&self) -> bool {
        self.is_mainnet() || self.is_previewnet() || self.is_testnet()
    }

    // todo: remove so that we can have `LedgerId` be an enum internally?
    // that would make `mainnet`, `testnet`, and `previewnet`, all const constructable.
    // then we could use a
    #[must_use]
    fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Provides a byte representation of `self`.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl Debug for LedgerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for LedgerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_mainnet() {
            f.write_str("mainnet")
        } else if self.is_testnet() {
            f.write_str("testnet")
        } else if self.is_previewnet() {
            f.write_str("previewnet")
        } else {
            f.write_str(&hex::encode(self.as_bytes()))
        }
    }
}

impl FromStr for LedgerId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainnet" => Ok(Self::mainnet()),
            "testnet" => Ok(Self::testnet()),
            "previewnet" => Ok(Self::previewnet()),
            _ => hex::decode(s).map(Self::from_bytes).map_err(Error::basic_parse),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::LedgerId;

    #[test]
    fn it_can_to_string() {
        assert_eq!(LedgerId::mainnet().to_string(), "mainnet");
        assert_eq!(LedgerId::testnet().to_string(), "testnet");
        assert_eq!(LedgerId::previewnet().to_string(), "previewnet");
        assert_eq!(
            LedgerId::from_bytes(vec![0x00, 0xFF, 0x00, 0xFF]).to_string().to_uppercase(),
            "00FF00FF"
        );
    }

    #[test]
    fn it_can_from_string() {
        assert_eq!(LedgerId::from_str("mainnet").unwrap(), LedgerId::mainnet());
        assert_eq!(LedgerId::from_str("testnet").unwrap(), LedgerId::testnet());
        assert_eq!(LedgerId::from_str("previewnet").unwrap(), LedgerId::previewnet());
        assert_eq!(
            LedgerId::from_str("00ff00ff").unwrap(),
            LedgerId::from_bytes(vec![0x00, 0xFF, 0x00, 0xFF])
        );
        assert_eq!(
            LedgerId::from_str("00FF00FF").unwrap(),
            LedgerId::from_bytes(vec![0x00, 0xFF, 0x00, 0xFF])
        );
    }

    #[test]
    fn it_can_to_bytes() {
        let bytes = vec![0x00, 0xFF, 0x00, 0xFF];
        assert_eq!(LedgerId::from_bytes(bytes.clone()).as_bytes(), &bytes);
    }
}
