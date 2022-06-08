use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{Error, FromProtobuf, ToProtobuf};

/// Possible Token Types (IWA Compatibility).
/// Apart from fungible and non-fungible, Tokens can have either a common or unique representation.
/// This distinction might seem subtle, but it is important when considering how tokens can be traced
/// and if they can have isolated and unique properties.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum TokenType {
    /// Interchangeable value with one another, where any quantity of them has the same value as
    /// another equal quantity if they are in the same class.  Share a single set of properties, not
    /// distinct from one another. Simply represented as a balance or quantity to a given Hedera
    /// account.
    FungibleCommon = 0,

    /// Unique, not interchangeable with other tokens of the same type as they typically have
    /// different values.  Individually traced and can carry unique properties (e.g. serial number).
    NonFungibleUnique = 1,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromProtobuf for TokenType {
    type Protobuf = services::TokenType;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        match pb {
            Self::Protobuf::FungibleCommon => Ok(TokenType::FungibleCommon),
            Self::Protobuf::NonFungibleUnique => Ok(TokenType::NonFungibleUnique),
        }
    }
}

impl ToProtobuf for TokenType {
    type Protobuf = services::TokenType;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            TokenType::FungibleCommon => Self::Protobuf::FungibleCommon,
            TokenType::NonFungibleUnique => Self::Protobuf::NonFungibleUnique,
        }
    }
}

impl FromStr for TokenType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FungibleCommon" => Ok(TokenType::FungibleCommon),
            "NonFungibleUnique" => Ok(TokenType::NonFungibleUnique),
            _ => Err(Error::basic_parse("failed to parse TokenType - expected 'FungibleCommon' or 'NonFungibleUnique'."))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use hedera_proto::services;
    use crate::token::token_type::TokenType;
    use crate::{FromProtobuf, ToProtobuf};

    #[test]
    fn it_can_convert_to_protobuf() -> anyhow::Result<()> {
        let nft_token_type = TokenType::NonFungibleUnique;
        let fungible_token_type = TokenType::FungibleCommon;

        let nft_protobuf = nft_token_type.to_protobuf();
        let fungible_protobuf = fungible_token_type.to_protobuf();

        assert_eq!(nft_protobuf, services::TokenType::NonFungibleUnique);
        assert_eq!(fungible_protobuf, services::TokenType::FungibleCommon);

        Ok(())
    }

    #[test]
    fn it_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let nft_protobuf = services::TokenType::NonFungibleUnique;
        let fungible_protobuf = services::TokenType::FungibleCommon;

        let nft_token_type = TokenType::from_protobuf(nft_protobuf).unwrap();
        let fungible_token_type = TokenType::from_protobuf(fungible_protobuf).unwrap();

        assert_eq!(nft_token_type, TokenType::NonFungibleUnique);
        assert_eq!(fungible_token_type, TokenType::FungibleCommon);

        Ok(())
    }

    #[test]
    fn it_can_parse_from_string() -> anyhow::Result<()> {
        let nft_string = "NonFungibleUnique";
        let fungible_string = "FungibleCommon";

        let nft_token_type = TokenType::from_str(nft_string).unwrap();
        let fungible_token_type = TokenType::from_str(fungible_string).unwrap();

        assert_eq!(nft_token_type, TokenType::NonFungibleUnique);
        assert_eq!(fungible_token_type, TokenType::FungibleCommon);

        Ok(())
    }
}
