use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::network_service_client::NetworkServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    NetworkVersionInfo,
    Query,
};

/// Get information about the versions of protobuf and hedera.
///
pub type NetworkVersionInfoQuery = Query<NetworkVersionInfoQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVersionInfoQueryData {}

impl From<NetworkVersionInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: NetworkVersionInfoQueryData) -> Self {
        Self::NetworkVersionInfo(data)
    }
}

impl ToQueryProtobuf for NetworkVersionInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        services::Query {
            query: Some(services::query::Query::NetworkGetVersionInfo(
                services::NetworkGetVersionInfoQuery { header: Some(header) },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for NetworkVersionInfoQueryData {
    type Response = NetworkVersionInfo;

    fn is_payment_required(&self) -> bool {
        false
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        NetworkServiceClient::new(channel).get_version_info(request).await
    }
}
