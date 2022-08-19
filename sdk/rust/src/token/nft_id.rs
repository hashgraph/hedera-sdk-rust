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

use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use hedera_proto::services;
use serde_with::{
    DeserializeFromStr,
    SerializeDisplay,
};

use crate::{
    Error,
    FromProtobuf,
    ToProtobuf,
    TokenId,
};

/// The unique identifier for a token on Hedera.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct NftId {
    /// The (non-fungible) token of which this NFT is an instance.
    pub token_id: TokenId,

    /// The unique identifier for this instance.
    pub serial_number: u64,
}

impl Debug for NftId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for NftId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.token_id, self.serial_number)
    }
}

impl FromProtobuf<services::NftId> for NftId {
    fn from_protobuf(pb: services::NftId) -> crate::Result<Self> {
        Ok(Self {
            token_id: TokenId::from_protobuf(pb_getf!(pb, token_id)?)?,
            serial_number: pb.serial_number as u64,
        })
    }
}

impl ToProtobuf for NftId {
    type Protobuf = services::NftId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::NftId {
            token_id: Some(self.token_id.to_protobuf()),
            serial_number: self.serial_number as i64,
        }
    }
}

impl FromStr for NftId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str>;
        if s.contains("/") {
            parts = s.split("/").collect();
        } else if s.contains("@") {
            parts = s.split("@").collect();
        } else {
            return Err(Error::basic_parse("unexpected NftId format - expected [token_id]/[serial_number] or [token_id]@[serial_number]"));
        }

        let serial_number = match parts[1].parse::<u64>() {
            Ok(serial_number) => serial_number,
            Err(_) => {
                return Err(Error::basic_parse("unexpected error - could not parse serial number"))
            }
        };

        Ok(Self { token_id: TokenId::from_str(parts[0])?, serial_number })
    }
}

impl From<(TokenId, u64)> for NftId {
    fn from(tuple: (TokenId, u64)) -> Self {
        Self { token_id: tuple.0, serial_number: tuple.1 }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use hedera_proto::services;

    use crate::token::nft_id::NftId;
    use crate::{
        FromProtobuf,
        ToProtobuf,
        TokenId,
    };

    #[test]
    fn it_can_convert_to_protobuf() -> anyhow::Result<()> {
        let nft_id = NftId { token_id: TokenId::from(1), serial_number: 1 };

        let nft_id_proto = nft_id.to_protobuf();

        assert_eq!(nft_id.serial_number, nft_id_proto.serial_number as u64);
        assert_eq!(nft_id.token_id.to_protobuf(), nft_id_proto.token_id.unwrap());

        Ok(())
    }

    #[test]
    fn it_can_create_from_protobuf() -> anyhow::Result<()> {
        let nft_id_proto =
            services::NftId { token_id: Some(TokenId::from(1).to_protobuf()), serial_number: 1 };

        let nft_id = NftId::from_protobuf(nft_id_proto.clone())?;

        assert_eq!(nft_id.serial_number, nft_id_proto.serial_number as u64);
        assert_eq!(nft_id.token_id, TokenId::from_protobuf(nft_id_proto.token_id.unwrap())?);

        Ok(())
    }

    #[test]
    fn it_can_parse_from_str() -> anyhow::Result<()> {
        // Test '/' format parsing
        let nft_id_slash_str = "0.0.123/456";

        let nft_id_from_slash_str = NftId::from_str(nft_id_slash_str)?;

        assert_eq!(nft_id_from_slash_str.serial_number, 456);
        assert_eq!(nft_id_from_slash_str.token_id.num, 123);

        // Test '@' format parsing
        let nft_id_at_str = "0.0.123@456";

        let nft_id_from_at_str = NftId::from_str(nft_id_at_str)?;

        assert_eq!(nft_id_from_at_str.serial_number, 456);
        assert_eq!(nft_id_from_at_str.token_id.num, 123);

        Ok(())
    }

    #[test]
    fn it_can_create_from_a_tuple() -> anyhow::Result<()> {
        let tuple = (TokenId::from(1), 123);

        let nft_id_from_tuple = NftId::from(tuple);

        assert_eq!(tuple.0, nft_id_from_tuple.token_id);
        assert_eq!(tuple.1, nft_id_from_tuple.serial_number);

        Ok(())
    }

    #[test]
    fn it_can_create_by_using_into_on_tuple() -> anyhow::Result<()> {
        let tuple = (TokenId::from(1), 123);

        let nft_id_from_tuple: NftId = tuple.into();

        assert_eq!(tuple.0, nft_id_from_tuple.token_id);
        assert_eq!(tuple.1, nft_id_from_tuple.serial_number);

        Ok(())
    }
}
