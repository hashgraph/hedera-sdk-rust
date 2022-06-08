use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{Error, FromProtobuf, ToProtobuf};

/// Possible Token Supply Types (IWA Compatibility).
/// Indicates how many tokens can have during its lifetime.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum TokenSupplyType {
    /// Indicates that tokens of that type have an upper bound of Long.MAX_VALUE.
    Infinite = 0,

    /// Indicates that tokens of that type have an upper bound of maxSupply,
    /// provided on token creation.
    Finite = 1,
}

impl Debug for TokenSupplyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for TokenSupplyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromProtobuf for TokenSupplyType {
    type Protobuf = services::TokenSupplyType;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        match pb {
            Self::Protobuf::Infinite => Ok(TokenSupplyType::Infinite),
            Self::Protobuf::Finite => Ok(TokenSupplyType::Finite),
        }
    }
}

impl ToProtobuf for TokenSupplyType {
    type Protobuf = services::TokenSupplyType;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            TokenSupplyType::Infinite => Self::Protobuf::Infinite,
            TokenSupplyType::Finite => Self::Protobuf::Finite,
        }
    }
}

impl FromStr for TokenSupplyType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Infinite" => Ok(TokenSupplyType::Infinite),
            "Finite" => Ok(TokenSupplyType::Finite),
            _ => Err(Error::basic_parse("failed to parse TokenSupplyType - expected 'Infinite' or 'Finite'."))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use hedera_proto::services;
    use crate::token::token_supply_type::TokenSupplyType;
    use crate::{FromProtobuf, ToProtobuf};

    #[test]
    fn it_can_convert_to_protobuf() -> anyhow::Result<()> {
        let infinite_supply_type = TokenSupplyType::Infinite;
        let finite_supply_type = TokenSupplyType::Finite;

        let infinite_protobuf = infinite_supply_type.to_protobuf();
        let finite_protobuf = finite_supply_type.to_protobuf();

        assert_eq!(infinite_protobuf, services::TokenSupplyType::Infinite);
        assert_eq!(finite_protobuf, services::TokenSupplyType::Finite);

        Ok(())
    }

    #[test]
    fn it_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let infinite_protobuf = services::TokenSupplyType::Infinite;
        let finite_protobuf = services::TokenSupplyType::Finite;

        let infinite_supply_type = TokenSupplyType::from_protobuf(infinite_protobuf).unwrap();
        let finite_supply_type = TokenSupplyType::from_protobuf(finite_protobuf).unwrap();

        assert_eq!(infinite_supply_type, TokenSupplyType::Infinite);
        assert_eq!(finite_supply_type, TokenSupplyType::Finite);

        Ok(())
    }

    #[test]
    fn it_can_parse_from_string() -> anyhow::Result<()> {
        let infinite_string = "Infinite";
        let finite_string = "Finite";

        let infinite_supply_type = TokenSupplyType::from_str(infinite_string).unwrap();
        let finite_supply_type = TokenSupplyType::from_str(finite_string).unwrap();

        assert_eq!(infinite_supply_type, TokenSupplyType::Infinite);
        assert_eq!(finite_supply_type, TokenSupplyType::Finite);

        Ok(())
    }
}
