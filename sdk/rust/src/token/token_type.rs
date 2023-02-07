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

use hedera_proto::services;

use crate::{
    FromProtobuf,
    ToProtobuf,
};

/// Possible token types.
///
/// Apart from fungible and non-fungible, tokens can have either a common or
/// unique representation.
///
/// Only `FungibleCommon` and `NonFungibleUnique` are supported right now. More
/// may be added in the future.
///
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    /// Interchangeable value with one another, where any quantity of them has the same value as
    /// another equal quantity if they are in the same class.  Share a single set of properties, not
    /// distinct from one another. Simply represented as a balance or quantity to a given Hedera
    /// account.
    FungibleCommon,

    /// Unique, not interchangeable with other tokens of the same type as they typically have
    /// different values.  Individually traced and can carry unique properties (e.g. serial number).
    NonFungibleUnique,
}

impl FromProtobuf<services::TokenType> for TokenType {
    fn from_protobuf(pb: services::TokenType) -> crate::Result<Self> {
        Ok(match pb {
            services::TokenType::FungibleCommon => Self::FungibleCommon,
            services::TokenType::NonFungibleUnique => Self::NonFungibleUnique,
        })
    }
}

impl ToProtobuf for TokenType {
    type Protobuf = services::TokenType;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            Self::FungibleCommon => Self::Protobuf::FungibleCommon,
            Self::NonFungibleUnique => Self::Protobuf::NonFungibleUnique,
        }
    }
}

#[cfg(test)]
mod tests {
    use hedera_proto::services;

    use crate::token::token_type::TokenType;
    use crate::{
        FromProtobuf,
        ToProtobuf,
    };

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

        let nft_token_type = TokenType::from_protobuf(nft_protobuf)?;
        let fungible_token_type = TokenType::from_protobuf(fungible_protobuf)?;

        assert_eq!(nft_token_type, TokenType::NonFungibleUnique);
        assert_eq!(fungible_token_type, TokenType::FungibleCommon);

        Ok(())
    }
}
