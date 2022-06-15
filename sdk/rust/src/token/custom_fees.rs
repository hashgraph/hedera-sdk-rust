use fraction::Fraction;
use hedera_proto::services;

use crate::{AccountId, FromProtobuf, ToProtobuf, TokenId};

/// A transfer fee to assess during a CryptoTransfer that transfers units of the token to which the
/// fee is attached. A custom fee may be either fixed or fractional, and must specify a fee collector
/// account to receive the assessed fees. Only positive fees may be assessed.
#[derive(serde::Serialize, serde::Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct CustomFee {
    /// The fee to be charged.
    fee: Fee,

    /// The account to receive the custom fee
    fee_collector_account_id: AccountId,
}

impl FromProtobuf for CustomFee {
    type Protobuf = services::CustomFee;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        let fee = Fee::from_protobuf(pb_getf!(pb, fee)?)?;
        let fee_collector_account_id =
            AccountId::from_protobuf(pb_getf!(pb, fee_collector_account_id)?)?;

        Ok(Self { fee_collector_account_id, fee })
    }
}

impl ToProtobuf for CustomFee {
    type Protobuf = services::CustomFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            fee: Some(self.fee.to_protobuf()),
            fee_collector_account_id: Some(self.fee_collector_account_id.to_protobuf()),
        }
    }
}

/// Represents the possible fee types.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Fee {
    /// Fixed fee to be charged.
    FixedFee(FixedFee),

    /// Fractional fee to be charged.
    FractionalFee(FractionalFee),

    /// Royalty fee to be charged.
    RoyaltyFee(RoyaltyFee),
}

impl FromProtobuf for Fee {
    type Protobuf = services::custom_fee::Fee;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        match pb {
            Self::Protobuf::FixedFee(fixed_fee) => {
                Ok(Fee::FixedFee(FixedFee::from_protobuf(fixed_fee)?))
            }
            Self::Protobuf::FractionalFee(fractional_fee) => {
                Ok(Fee::FractionalFee(FractionalFee::from_protobuf(fractional_fee)?))
            }
            Self::Protobuf::RoyaltyFee(royalty_fee) => {
                Ok(Fee::RoyaltyFee(RoyaltyFee::from_protobuf(royalty_fee)?))
            }
        }
    }
}

impl ToProtobuf for Fee {
    type Protobuf = services::custom_fee::Fee;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            Self::FixedFee(fixed_fee) => Self::Protobuf::FixedFee(fixed_fee.to_protobuf()),
            Self::FractionalFee(fractional_fee) => {
                Self::Protobuf::FractionalFee(fractional_fee.to_protobuf())
            }
            Self::RoyaltyFee(royalty_fee) => Self::Protobuf::RoyaltyFee(royalty_fee.to_protobuf()),
        }
    }
}

impl From<FixedFee> for Fee {
    fn from(fixed_fee: FixedFee) -> Self {
        Self::FixedFee(fixed_fee)
    }
}

impl From<RoyaltyFee> for Fee {
    fn from(royalty_fee: RoyaltyFee) -> Self {
        Self::RoyaltyFee(royalty_fee)
    }
}

impl From<FractionalFee> for Fee {
    fn from(fractional_fee: FractionalFee) -> Self {
        Self::FractionalFee(fractional_fee)
    }
}

/// A fixed number of units (hbar or token) to assess as a fee during a CryptoTransfer that transfers
/// units of the token to which this fixed fee is attached.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct FixedFee {
    /// The number of units to assess as a fee
    pub amount: i64,

    /// The denomination of the fee; taken as hbar if left unset and, in a TokenCreate, taken as the id
    /// of the newly created token if set to the sentinel value of 0.0.0
    pub denominating_token_id: TokenId,
}

impl FromProtobuf for FixedFee {
    type Protobuf = services::FixedFee;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            amount: pb.amount,
            denominating_token_id: TokenId::from_protobuf(pb_getf!(pb, denominating_token_id)?)?,
        })
    }
}

impl ToProtobuf for FixedFee {
    type Protobuf = services::FixedFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            amount: self.amount,
            denominating_token_id: Some(self.denominating_token_id.to_protobuf()),
        }
    }
}

/// A fraction of the transferred units of a token to assess as a fee. The amount assessed will never
/// be less than the given minimum_amount, and never greater than the given maximum_amount.  The
/// denomination is always units of the token to which this fractional fee is attached.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct FractionalFee {
    /// The fraction of the transferred units to assess as a fee
    pub fractional_amount: Fraction,

    /// The minimum amount to assess
    pub minimum_amount: i64,

    /// The maximum amount to assess (zero implies no maximum)
    pub maximum_amount: i64,

    /// If true, assesses the fee to the sender, so the receiver gets the full amount from the token
    /// transfer list, and the sender is charged an additional fee; if false, the receiver does NOT get
    /// the full amount, but only what is left over after paying the fractional fee
    pub net_of_transfers: bool,
}

impl FromProtobuf for FractionalFee {
    type Protobuf = services::FractionalFee;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            net_of_transfers: pb.net_of_transfers,
            maximum_amount: pb.maximum_amount,
            minimum_amount: pb.minimum_amount,
            fractional_amount: pb.fractional_amount.map(Into::into).unwrap_or_default(),
        })
    }
}

impl ToProtobuf for FractionalFee {
    type Protobuf = services::FractionalFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            fractional_amount: Some(self.fractional_amount.into()),
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            net_of_transfers: self.net_of_transfers,
        }
    }
}

/// A fee to assess during a CryptoTransfer that changes ownership of an NFT. Defines the fraction of
/// the fungible value exchanged for an NFT that the ledger should collect as a royalty. ("Fungible
/// value" includes both ℏ and units of fungible HTS tokens.) When the NFT sender does not receive
/// any fungible value, the ledger will assess the fallback fee, if present, to the new NFT owner.
/// Royalty fees can only be added to tokens of type type NON_FUNGIBLE_UNIQUE.
#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct RoyaltyFee {
    /// The fraction of fungible value exchanged for an NFT to collect as royalty
    pub exchange_value_fraction: Fraction,

    /// If present, the fixed fee to assess to the NFT receiver when no fungible value is exchanged
    /// with the sender
    pub fallback_fee: FixedFee,
}

impl FromProtobuf for RoyaltyFee {
    type Protobuf = services::RoyaltyFee;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            fallback_fee: FixedFee::from_protobuf(pb_getf!(pb, fallback_fee)?)?,
            exchange_value_fraction: pb.exchange_value_fraction.map(Into::into).unwrap_or_default(),
        })
    }
}

impl ToProtobuf for RoyaltyFee {
    type Protobuf = services::RoyaltyFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            fallback_fee: Some(self.fallback_fee.to_protobuf()),
            exchange_value_fraction: Some(self.exchange_value_fraction.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use hedera_proto::services;

    use crate::fraction::Fraction;
    use crate::token::custom_fees::{CustomFee, Fee, FixedFee, FractionalFee, RoyaltyFee};
    use crate::{AccountId, FromProtobuf, ToProtobuf, TokenId};

    #[test]
    fn custom_fee_can_convert_to_protobuf() -> anyhow::Result<()> {
        let custom_fee = CustomFee {
            fee_collector_account_id: AccountId::from(1),
            fee: Fee::FixedFee(FixedFee { denominating_token_id: TokenId::from(2), amount: 1000 }),
        };

        let custom_fee_proto = custom_fee.to_protobuf();

        assert_eq!(Some(custom_fee.fee.to_protobuf()), custom_fee_proto.fee);
        assert_eq!(
            Some(custom_fee.fee_collector_account_id.to_protobuf()),
            custom_fee_proto.fee_collector_account_id
        );

        Ok(())
    }

    #[test]
    fn custom_fixed_fee_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let custom_fee_proto = services::CustomFee {
            fee: Some(services::custom_fee::Fee::FixedFee(services::FixedFee {
                denominating_token_id: Some(TokenId::from(2).to_protobuf()),
                amount: 1000,
            })),
            fee_collector_account_id: Some(AccountId::from(1).to_protobuf()),
        };

        let custom_fee = CustomFee::from_protobuf(custom_fee_proto.clone()).unwrap();

        assert_eq!(Some(custom_fee.fee.to_protobuf()), custom_fee_proto.fee);
        assert_eq!(
            Some(custom_fee.fee_collector_account_id.to_protobuf()),
            custom_fee_proto.fee_collector_account_id
        );

        Ok(())
    }

    #[test]
    fn fee_can_convert_to_protobuf() -> anyhow::Result<()> {
        let amount = 1000;
        let fee = Fee::FixedFee(FixedFee { amount, denominating_token_id: TokenId::from(1) });

        let fee_proto = fee.to_protobuf();

        let fixed_fee_proto = match fee_proto {
            services::custom_fee::Fee::FixedFee(fixed_fee) => Some(fixed_fee),
            _ => None,
        };

        assert_eq!(fixed_fee_proto.unwrap().amount, amount);

        Ok(())
    }

    #[test]
    fn fee_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let amount = 1000;
        let fee_proto = services::custom_fee::Fee::FixedFee(services::FixedFee {
            denominating_token_id: Some(TokenId::from(2).to_protobuf()),
            amount,
        });

        let custom_fee = Fee::from_protobuf(fee_proto).unwrap();

        let fixed_fee = match custom_fee {
            Fee::FixedFee(fixed_fee) => Some(fixed_fee),
            _ => None,
        };

        assert_eq!(fixed_fee.unwrap().amount, amount);

        Ok(())
    }

    #[test]
    fn fixed_fee_can_convert_to_protobuf() -> anyhow::Result<()> {
        let amount = 1000;
        let fixed_fee = FixedFee { amount, denominating_token_id: TokenId::from(2) };

        let fixed_fee_proto = fixed_fee.to_protobuf();

        assert_eq!(fixed_fee_proto.amount, amount);

        Ok(())
    }

    #[test]
    fn fixed_fee_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let amount = 1000;
        let fixed_fee_proto = services::FixedFee {
            amount,
            denominating_token_id: Some(TokenId::from(2).to_protobuf()),
        };

        let fixed_fee = FixedFee::from_protobuf(fixed_fee_proto).unwrap();

        assert_eq!(fixed_fee.amount, amount);

        Ok(())
    }

    #[test]
    fn fractional_fee_can_convert_to_protobuf() -> anyhow::Result<()> {
        let minimum_amount = 500;
        let maximum_amount = 1000;
        let net_of_transfers = true;

        let fractional_fee = FractionalFee {
            fractional_amount: Fraction { numerator: 1, denominator: 2 },
            minimum_amount,
            maximum_amount,
            net_of_transfers,
        };

        let fractional_fee_proto = fractional_fee.to_protobuf();

        assert_eq!(fractional_fee_proto.minimum_amount, minimum_amount);
        assert_eq!(fractional_fee_proto.maximum_amount, maximum_amount);
        assert_eq!(fractional_fee_proto.net_of_transfers, net_of_transfers);

        Ok(())
    }

    #[test]
    fn fractional_fee_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let minimum_amount = 500;
        let maximum_amount = 1000;
        let net_of_transfers = true;

        let fractional_fee_protobuf = services::FractionalFee {
            fractional_amount: Some(services::Fraction { numerator: 1, denominator: 2 }),
            minimum_amount,
            maximum_amount,
            net_of_transfers,
        };

        let fractional_fee = FractionalFee::from_protobuf(fractional_fee_protobuf).unwrap();

        assert_eq!(fractional_fee.minimum_amount, minimum_amount);
        assert_eq!(fractional_fee.maximum_amount, maximum_amount);
        assert_eq!(fractional_fee.net_of_transfers, net_of_transfers);

        Ok(())
    }

    #[test]
    fn royalty_fee_can_convert_to_protobuf() -> anyhow::Result<()> {
        let fallback_fee = FixedFee { denominating_token_id: TokenId::from(1), amount: 1000 };
        let exchange_value_fraction = Fraction { numerator: 1, denominator: 2 };

        let royalty_fee =
            RoyaltyFee { fallback_fee: fallback_fee.clone(), exchange_value_fraction };

        let royalty_fee_proto = royalty_fee.to_protobuf();

        assert_eq!(royalty_fee_proto.fallback_fee, Some(fallback_fee.to_protobuf()));
        assert_eq!(
            royalty_fee_proto.exchange_value_fraction,
            Some(exchange_value_fraction.to_protobuf())
        );

        Ok(())
    }

    #[test]
    fn royalty_fee_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let amount = 1000;
        let numerator = 1;
        let denominator = 2;

        let fallback_fee = services::FixedFee {
            denominating_token_id: Some(TokenId::from(1).to_protobuf()),
            amount,
        };
        let exchange_value_fraction = services::Fraction { numerator, denominator };

        let royalty_fee_proto = services::RoyaltyFee {
            fallback_fee: Some(fallback_fee),
            exchange_value_fraction: Some(exchange_value_fraction),
        };

        let royalty_fee = RoyaltyFee::from_protobuf(royalty_fee_proto).unwrap();

        assert_eq!(royalty_fee.fallback_fee.amount, amount);
        assert_eq!(royalty_fee.exchange_value_fraction.numerator, numerator);
        assert_eq!(royalty_fee.exchange_value_fraction.denominator, denominator);

        Ok(())
    }
}
