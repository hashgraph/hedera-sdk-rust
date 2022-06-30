use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use tonic::transport::Channel;

use crate::query::{AnyQueryData, QueryExecute, ToQueryProtobuf};
use crate::{TokenId, Query, ToProtobuf};
use crate::token::token_info::TokenInfo;

/// Get all the information about an token, including the balance.
///
/// This does not get the list of token records.
///
pub type TokenInfoQuery = Query<TokenInfoQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
    /// Sets the token ID for which information is requested.
    pub fn token_id(&mut self, id: impl Into<TokenId>) -> &mut Self {
        self.data.token_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for TokenInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let token_id = self.token_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::TokenGetInfo(services::TokenGetInfoQuery {
                header: Some(header),
                token_id,
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for TokenInfoQueryData {
    type Response = TokenInfo;

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
    use assert_matches::assert_matches;

    use crate::query::AnyQueryData;
    use crate::{TokenId, TokenId, TokenInfoQuery, AnyQuery};
    use crate::token::token_info_query::TokenInfoQuery;

    // language=JSON
    const TOKEN_INFO: &str = r#"{
  "$type": "tokenInfo",
  "tokenId": "0.0.1001"
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut query = TokenInfoQuery::new();
        query
            .token_id(TokenId::from(1001))
            .token_id(TokenId::from(6189));

        let s = serde_json::to_string_pretty(&query)?;
        assert_eq!(s, TOKEN_INFO);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let query: AnyQuery = serde_json::from_str(TOKEN_INFO)?;

        let data = assert_matches!(query.data, AnyQueryData::TokenInfo(query) => query);
        let token_id =
            assert_matches!(data.token_id, Some(TokenId::TokenId(id)) => id);

        assert_eq!(token_id.num, 1001);
        Ok(())
    }
}
