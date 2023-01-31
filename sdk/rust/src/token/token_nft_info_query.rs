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

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    Query,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    Error,
    LedgerId,
    NftId,
    ToProtobuf,
    TokenNftInfo,
    ValidateChecksums,
};

/// Gets info on an NFT for a given `TokenID` and serial number.
pub type TokenNftInfoQuery = Query<TokenNftInfoQueryData>;

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TokenNftInfoQueryData {
    /// The ID of the NFT
    nft_id: Option<NftId>,
}

impl From<TokenNftInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TokenNftInfoQueryData) -> Self {
        Self::TokenNftInfo(data)
    }
}

impl TokenNftInfoQuery {
    /// Returns the ID of the NFT for which information is requested.
    #[must_use]
    pub fn get_nft_id(&self) -> Option<NftId> {
        self.data.nft_id
    }

    /// Sets the ID of the NFT for which information is requested.
    pub fn nft_id(&mut self, nft_id: impl Into<NftId>) -> &mut Self {
        self.data.nft_id = Some(nft_id.into());
        self
    }
}

impl ToQueryProtobuf for TokenNftInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let nft_id = self.nft_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::TokenGetNftInfo(services::TokenGetNftInfoQuery {
                header: Some(header),
                nft_id,
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for TokenNftInfoQueryData {
    type Response = TokenNftInfo;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        TokenServiceClient::new(channel).get_token_nft_info(request).await
    }
}

impl ValidateChecksums for TokenNftInfoQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.nft_id.validate_checksums(ledger_id)
    }
}
