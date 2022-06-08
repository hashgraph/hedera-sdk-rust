use std::fmt::{self, Debug, Display, Formatter};

use hedera_proto::services;

use crate::{AccountId, FromProtobuf, TokenId, ToProtobuf};

#[derive(serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CustomFee {
    FixedFee(FixedFee),
    FractionalFee(FractionalFee),
    RoyaltyFee(RoyaltyFee),
}

impl Debug for CustomFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for CustomFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ToProtobuf for CustomFee {
    type Protobuf = services::CustomFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            Self::FixedFee(fixed_fee) => fixed_fee.to_protobuf(),
            Self::FractionalFee(fractional_fee) => fractional_fee.to_protobuf(),
            Self::RoyaltyFee(royalty_fee) => royalty_fee.to_protobuf(),
        }
    }
}

impl From<FixedFee> for CustomFee {
    fn from(fixed_fee: FixedFee) -> Self {
        Self::FixedFee(fixed_fee)
    }
}

impl From<RoyaltyFee> for CustomFee {
    fn from(royalty_fee: RoyaltyFee) -> Self {
        Self::RoyaltyFee(royalty_fee)
    }
}

impl From<FractionalFee> for CustomFee {
    fn from(fractional_fee: FractionalFee) -> Self {
        Self::FractionalFee(fractional_fee)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct FixedFee {
    amount: i64,
    denominating_token_id: TokenId,
    fee_collector_account_id: AccountId,
}

impl Debug for FixedFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for FixedFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ToProtobuf for FixedFee {
    type Protobuf = services::CustomFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        let fixed_fee_proto = services::FixedFee {
            amount: self.amount,
            denominating_token_id: Some(self.denominating_token_id.to_protobuf())
        };

        Self::Protobuf {
            fee: Some(services::custom_fee::Fee::FixedFee(fixed_fee_proto)),
            fee_collector_account_id: Some(self.fee_collector_account_id.to_protobuf())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct FractionalFee {}

impl Debug for FractionalFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for FractionalFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ToProtobuf for FractionalFee {
    type Protobuf = services::CustomFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        todo!()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct RoyaltyFee {}

impl Debug for RoyaltyFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for RoyaltyFee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ToProtobuf for RoyaltyFee {
    type Protobuf = services::CustomFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use hedera_proto::services;
    use crate::{AccountId, FromProtobuf, TokenId, ToProtobuf};
    use crate::token::custom_fees::{CustomFee, FixedFee};

    #[test]
    fn it_can_convert_to_protobuf() -> anyhow::Result<()> {

        Ok(())
    }

    #[test]
    fn it_can_be_created_from_protobuf() -> anyhow::Result<()> {

        Ok(())
    }
}
