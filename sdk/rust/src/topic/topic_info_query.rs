use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    Query,
    ToProtobuf,
    TopicId,
    TopicInfo,
};

/// Retrieve the latest state of a topic.
///
pub type TopicInfoQuery = Query<TopicInfoQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopicInfoQueryData {
    topic_id: Option<TopicId>,
}

impl From<TopicInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TopicInfoQueryData) -> Self {
        Self::TopicInfo(data)
    }
}

impl TopicInfoQuery {
    /// Set the topic to retrieve info about.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data.topic_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for TopicInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let topic_id = self.topic_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::ConsensusGetTopicInfo(
                services::ConsensusGetTopicInfoQuery { topic_id, header: Some(header) },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for TopicInfoQueryData {
    type Response = TopicInfo;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        ConsensusServiceClient::new(channel).get_topic_info(request).await
    }
}
