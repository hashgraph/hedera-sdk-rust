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
use prost::Message;
use time::OffsetDateTime;

use crate::protobuf::ToProtobuf;
use crate::{
    AccountId,
    FromProtobuf,
    LedgerId,
    NftId,
};

// TODO pub ledger_id: LedgerId, --- also shows as todo in account_info.rs
/// Response from [`TokenNftInfoQuery`][crate::TokenNftInfoQuery].

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TokenNftInfo {
    /// The ID of the NFT.
    pub nft_id: NftId,

    /// The current owner of the NFT.
    pub account_id: AccountId,

    /// Effective consensus timestamp at which the NFT was minted.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<serde_with::TimestampNanoSeconds>")
    )]
    pub creation_time: OffsetDateTime,

    /// The unique metadata of the NFT.
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    pub metadata: Vec<u8>,

    /// If an allowance is granted for the NFT, its corresponding spender account.
    pub spender_id: Option<AccountId>,

    /// The ledger ID the response was returned from.
    pub ledger_id: LedgerId,
}

impl TokenNftInfo {
    /// Create a new `TokenInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::TokenNftInfo>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        services::TokenNftInfo {
            nft_id: Some(self.nft_id.to_protobuf()),
            account_id: Some(self.account_id.to_protobuf()),
            creation_time: Some(self.creation_time.to_protobuf()),
            metadata: self.metadata.clone(),
            ledger_id: self.ledger_id.to_bytes(),
            spender_id: self.spender_id.to_protobuf(),
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::response::Response> for TokenNftInfo {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, TokenGetNftInfo, services::response::Response);
        let nft = pb_getf!(pb, nft)?;
        Self::from_protobuf(nft)
    }
}

impl FromProtobuf<services::TokenNftInfo> for TokenNftInfo {
    fn from_protobuf(pb: services::TokenNftInfo) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let nft_id = pb_getf!(pb, nft_id)?;
        let account_id = pb_getf!(pb, account_id)?;
        let creation_time = pb.creation_time.unwrap();
        let metadata = pb.metadata;
        let spender_account_id = Option::from_protobuf(pb.spender_id)?;

        Ok(Self {
            nft_id: NftId::from_protobuf(nft_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
            creation_time: OffsetDateTime::from(creation_time),
            metadata,
            spender_id: spender_account_id,
            ledger_id: LedgerId::from_bytes(pb.ledger_id),
        })
    }
}
