use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use serde_with::{
    DeserializeFromStr,
    SerializeDisplay,
};

use crate::Error;

#[derive(Eq, PartialEq, Clone, SerializeDisplay, DeserializeFromStr)]
pub struct LedgerId(Vec<u8>);

impl LedgerId {
    pub fn mainnet() -> Self {
        Self(vec![0])
    }

    pub fn testnet() -> Self {
        Self(vec![1])
    }

    pub fn previewnet() -> Self {
        Self(vec![2])
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn is_mainnet(&self) -> bool {
        self == &Self::mainnet()
    }

    pub fn is_testnet(&self) -> bool {
        self == &Self::testnet()
    }

    pub fn is_previewnet(&self) -> bool {
        self == &Self::previewnet()
    }

    pub fn is_known_network(&self) -> bool {
        self.is_mainnet() || self.is_previewnet() || self.is_testnet()
    }

    pub const fn to_bytes(&self) -> &Vec<u8> {
        &self.0
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
            f.write_str(&hex::encode(self.to_bytes()))
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
        assert_eq!(LedgerId::from_bytes(bytes.clone()).to_bytes(), &bytes);
    }
}
