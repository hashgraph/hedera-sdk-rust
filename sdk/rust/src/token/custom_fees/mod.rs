/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use fraction::Fraction;
use hedera_proto::services;

use crate::{
    AccountId,
    FromProtobuf,
    Hbar,
    ToProtobuf,
    TokenId,
};

#[cfg(test)]
mod tests;

/// A transfer fee to assess during a `CryptoTransfer` that transfers units of the token to which the
/// fee is attached. A custom fee may be either fixed or fractional, and must specify a fee collector
/// account to receive the assessed fees. Only positive fees may be assessed.
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct CustomFee {
    /// The fee to be charged.
    pub fee: Fee,

    /// The account to receive the custom fee
    pub fee_collector_account_id: AccountId,
}

impl FromProtobuf<services::CustomFee> for CustomFee {
    fn from_protobuf(pb: services::CustomFee) -> crate::Result<Self> {
        let fee = Fee::from_protobuf(pb_getf!(pb, fee)?)?;
        let fee_collector_account_id =
            AccountId::from_protobuf(pb_getf!(pb, fee_collector_account_id)?)?;

        Ok(Self { fee, fee_collector_account_id })
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
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum Fee {
    /// Fixed fee to be charged.
    FixedFee(FixedFee),

    /// Fractional fee to be charged.
    FractionalFee(FractionalFee),

    /// Royalty fee to be charged.
    RoyaltyFee(RoyaltyFee),
}

impl FromProtobuf<services::custom_fee::Fee> for Fee {
    fn from_protobuf(pb: services::custom_fee::Fee) -> crate::Result<Self> {
        use services::custom_fee::Fee as ProtoFee;

        match pb {
            ProtoFee::FixedFee(fixed_fee) => Ok(Fee::FixedFee(FixedFee::from_protobuf(fixed_fee)?)),
            ProtoFee::FractionalFee(fractional_fee) => {
                Ok(Fee::FractionalFee(FractionalFee::from_protobuf(fractional_fee)?))
            }
            ProtoFee::RoyaltyFee(royalty_fee) => {
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

/// A fixed number of units (hbar or token) to assess as a fee during a `CryptoTransfer` that transfers
/// units of the token to which this fixed fee is attached.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct FixedFee {
    /// The number of units to assess as a fee
    pub amount: i64,

    /// The denomination of the fee; taken as hbar if left unset and, in a TokenCreate, taken as the id
    /// of the newly created token if set to the sentinel value of 0.0.0
    pub denominating_token_id: TokenId,
}

impl FixedFee {
    #[must_use]
    pub fn from_hbar(amount: Hbar) -> Self {
        Self {
            amount: amount.to_tinybars(),
            denominating_token_id: TokenId { shard: 0, realm: 0, num: 0, checksum: None },
        }
    }

    #[must_use]
    pub fn get_hbar(&self) -> Option<Hbar> {
        (self.denominating_token_id == TokenId { shard: 0, realm: 0, num: 0, checksum: None })
            .then(|| Hbar::from_tinybars(self.amount))
    }
}

impl FromProtobuf<services::FixedFee> for FixedFee {
    fn from_protobuf(pb: services::FixedFee) -> crate::Result<Self> {
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
/// be less than the given `minimum_amount`, and never greater than the given `maximum_amount`.  The
/// denomination is always units of the token to which this fractional fee is attached.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
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

impl FromProtobuf<services::FractionalFee> for FractionalFee {
    fn from_protobuf(pb: services::FractionalFee) -> crate::Result<Self> {
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

/// A fee to assess during a `CryptoTransfer` that changes ownership of an NFT. Defines the fraction of
/// the fungible value exchanged for an NFT that the ledger should collect as a royalty. ("Fungible
/// value" includes both ℏ and units of fungible HTS tokens.) When the NFT sender does not receive
/// any fungible value, the ledger will assess the fallback fee, if present, to the new NFT owner.
/// Royalty fees can only be added to tokens of type type `NON_FUNGIBLE_UNIQUE`.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct RoyaltyFee {
    /// The fraction of fungible value exchanged for an NFT to collect as royalty
    pub exchange_value_fraction: Fraction,

    /// If present, the fixed fee to assess to the NFT receiver when no fungible value is exchanged
    /// with the sender
    pub fallback_fee: FixedFee,
}

impl FromProtobuf<services::RoyaltyFee> for RoyaltyFee {
    fn from_protobuf(pb: services::RoyaltyFee) -> crate::Result<Self> {
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
