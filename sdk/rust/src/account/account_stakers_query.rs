use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    AccountAddress,
    AllProxyStakers,
    Query,
    ToProtobuf,
};

/// Get all the accounts that are proxy staking to this account.
/// For each of them, give the amount currently staked.
pub type AccountStakersQuery = Query<AccountStakersQueryData>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStakersQueryData {
    account_id: Option<AccountAddress>,
}

impl From<AccountStakersQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountStakersQueryData) -> Self {
        Self::AccountStakers(data)
    }
}

impl AccountStakersQuery {
    /// Sets the account ID for which the records should be retrieved.
    pub fn account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.data.account_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for AccountStakersQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let account_id = self.account_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::CryptoGetProxyStakers(
                services::CryptoGetStakersQuery { account_id, header: Some(header) },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for AccountStakersQueryData {
    type Response = AllProxyStakers;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_stakers_by_account_id(request).await
    }
}
