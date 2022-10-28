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
use time::OffsetDateTime;

use crate::{
    AccountId,
    FromProtobuf,
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
    pub creation_time: OffsetDateTime,

    /// The unique metadata of the NFT.
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    pub metadata: Vec<u8>,

    /// If an allowance is granted for the NFT, its corresponding spender account.
    pub spender_account_id: Option<AccountId>,
}

impl FromProtobuf<services::response::Response> for TokenNftInfo {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, TokenGetNftInfo, services::response::Response);
        let nft = pb_getf!(pb, nft)?;

        let nft_id = pb_getf!(nft, nft_id)?;
        let account_id = pb_getf!(nft, account_id)?;
        let creation_time = nft.creation_time.unwrap();
        let metadata = nft.metadata;
        let spender_account_id = nft.spender_id.map(AccountId::from_protobuf).transpose()?;

        Ok(Self {
            nft_id: NftId::from_protobuf(nft_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
            creation_time: OffsetDateTime::from(creation_time),
            metadata,
            spender_account_id,
        })
    }
}
