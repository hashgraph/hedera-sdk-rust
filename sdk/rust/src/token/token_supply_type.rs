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

/// Possible token supply types.
/// Can be used to restrict supply to a set maximum.
/// Defaults to [`Infinite`](Self::Infinite).
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum TokenSupplyType {
    /// Indicates the token has a maximum supply of [`u64::MAX`].
    Infinite = 0,

    /// Indicates the token has a configurable maximum supply, provided on token creation.
    Finite = 1,
}

impl FromProtobuf<services::TokenSupplyType> for TokenSupplyType {
    fn from_protobuf(pb: services::TokenSupplyType) -> crate::Result<Self> {
        Ok(match pb {
            services::TokenSupplyType::Infinite => Self::Infinite,
            services::TokenSupplyType::Finite => Self::Finite,
        })
    }
}

impl ToProtobuf for TokenSupplyType {
    type Protobuf = services::TokenSupplyType;

    fn to_protobuf(&self) -> Self::Protobuf {
        match self {
            Self::Infinite => Self::Protobuf::Infinite,
            Self::Finite => Self::Protobuf::Finite,
        }
    }
}

#[cfg(test)]
mod tests {
    use hedera_proto::services;

    use crate::token::token_supply_type::TokenSupplyType;
    use crate::{
        FromProtobuf,
        ToProtobuf,
    };

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

        let infinite_supply_type = TokenSupplyType::from_protobuf(infinite_protobuf)?;
        let finite_supply_type = TokenSupplyType::from_protobuf(finite_protobuf)?;

        assert_eq!(infinite_supply_type, TokenSupplyType::Infinite);
        assert_eq!(finite_supply_type, TokenSupplyType::Finite);

        Ok(())
    }
}
