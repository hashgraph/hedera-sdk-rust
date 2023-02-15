use std::borrow::Borrow;
use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use crate::Error;

/// DST that's semantically a [`LedgerId`].
///
/// `&Self` saves a pointer indirection vs `&LedgerId` because `LedgerId` is a `Box<[u8]>`, so `&LedgerId` is a `&Box<[u8]>`.
///
/// It also allows for const constructable `LedgerId`s.
///
/// todo: actually use this everywhere
/// (it'd be a big refactor for the middle of the current PR)
///
/// Internal API, don't publically expose.
#[repr(transparent)]
#[derive(Eq, PartialEq)]
pub(crate) struct RefLedgerId([u8]);

impl RefLedgerId {
    pub(crate) const MAINNET: &Self = Self::new(&[0]);
    pub(crate) const TESTNET: &Self = Self::new(&[1]);
    pub(crate) const PREVIEWNET: &Self = Self::new(&[2]);

    pub const fn new(data: &[u8]) -> &Self {
        // safety: blessed by the standard library: see `path.rs`
        // https://github.com/rust-lang/rust/blob/3020239de947ec52677e9b4e853a6a9fc073d1f9/library/std/src/path.rs#L2037-L2039
        unsafe { &*(data as *const [u8] as *const RefLedgerId) }
    }

    pub fn new_boxed(data: Box<[u8]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(data) as *mut RefLedgerId) }
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for &'a RefLedgerId {
    fn from(value: &'a [u8]) -> Self {
        RefLedgerId::new(value)
    }
}

impl ToOwned for RefLedgerId {
    type Owned = LedgerId;

    fn to_owned(&self) -> Self::Owned {
        Self::Owned::from_bytes(self.0.to_vec())
    }
}

// todo: use `Box<[u8]>` (16 bytes on cpus with 64 bit pointers)
// or use an enum of `Mainnet`, `Testnet`, `Previewnet`, `Other`, and `Static`, don't expose the enum though.
// `Other` would be `Box<[u8]>`, but nothing else would have an alloc, the whole struct would be 24 bytes on x86-64,
// wouldn't allocate 99.99% of the time, and could be const constructable in 99.999% of cases.
/// The ID of a Hedera Ledger.
#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
pub struct LedgerId(Box<RefLedgerId>);

impl LedgerId {
    /// ID for the `mainnet` ledger.
    #[must_use]
    pub fn mainnet() -> Self {
        RefLedgerId::MAINNET.to_owned()
    }

    /// ID for the `testnet` ledger.
    #[must_use]
    pub fn testnet() -> Self {
        RefLedgerId::TESTNET.to_owned()
    }

    /// ID for the `previewnet` ledger.
    #[must_use]
    pub fn previewnet() -> Self {
        RefLedgerId::PREVIEWNET.to_owned()
    }

    /// Create a ledger ID from the given bytes.
    #[must_use]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(RefLedgerId::new_boxed(bytes.into_boxed_slice()))
    }

    /// Returns `true` if `self` is `mainnet`.
    #[must_use]
    pub fn is_mainnet(&self) -> bool {
        self.as_ref() == RefLedgerId::MAINNET
    }

    /// Returns `true` if `self` is `testnet`.
    #[must_use]
    pub fn is_testnet(&self) -> bool {
        self.as_ref() == RefLedgerId::TESTNET
    }

    /// Returns `true` if `self` is `previewnet`.
    #[must_use]
    pub fn is_previewnet(&self) -> bool {
        self.as_ref() == RefLedgerId::PREVIEWNET
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
        self.0.as_bytes()
    }

    /// Provides a byte representation of `self`.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl Clone for LedgerId {
    fn clone(&self) -> Self {
        Self::from_bytes(self.0.as_bytes().to_vec())
    }
}

impl AsRef<RefLedgerId> for LedgerId {
    fn as_ref(&self) -> &RefLedgerId {
        &self.0
    }
}

impl Borrow<RefLedgerId> for LedgerId {
    fn borrow(&self) -> &RefLedgerId {
        self.as_ref()
    }
}

impl Debug for LedgerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
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
