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
    QueryExecute,
    ToQueryProtobuf,
};
use crate::token::token_info::TokenInfo;
use crate::{
    Error,
    LedgerId,
    Query,
    ToProtobuf,
    TokenId,
    ValidateChecksums,
};

/// Gets information about Token instance.
///
pub type TokenInfoQuery = Query<TokenInfoQueryData>;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TokenInfoQueryData {
    token_id: Option<TokenId>,
}

impl From<TokenInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TokenInfoQueryData) -> Self {
        Self::TokenInfo(data)
    }
}

impl TokenInfoQuery {
    /// Returns the token ID for which information is requested.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data.token_id
    }

    /// Sets the token ID for which information is requested.
    pub fn token_id(&mut self, id: impl Into<TokenId>) -> &mut Self {
        self.data.token_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for TokenInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let token_id = self.token_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::TokenGetInfo(services::TokenGetInfoQuery {
                header: Some(header),
                token: token_id,
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for TokenInfoQueryData {
    type Response = TokenInfo;

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        TokenServiceClient::new(channel).get_token_info(request).await
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use assert_matches::assert_matches;

        use crate::query::AnyQueryData;
        use crate::{
            AnyQuery,
            TokenId,
            TokenInfoQuery,
        };

        // language=JSON
        const TOKEN_INFO: &str = r#"{
  "$type": "tokenInfo",
  "tokenId": "0.0.1001",
  "payment": {}
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut query = TokenInfoQuery::new();
            query.token_id(TokenId::from(1001));

            let s = serde_json::to_string_pretty(&query)?;
            assert_eq!(s, TOKEN_INFO);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let query: AnyQuery = serde_json::from_str(TOKEN_INFO)?;

            let data = assert_matches!(query.data, AnyQueryData::TokenInfo(query) => query);
            assert_eq!(
                data.token_id,
                Some(TokenId { shard: 0, realm: 0, num: 1001, checksum: None })
            );
            Ok(())
        }
    }
}
