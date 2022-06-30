use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use tonic::transport::Channel;

use crate::topic::TopicInfo;
use crate::query::{AnyQueryData, QueryExecute, ToQueryProtobuf};
use crate::{TopicAddress, Query, ToProtobuf};

/// Get all the information about a topic, including the balance.
///
/// This does not get the list of topic records.
///
pub type TopicInfoQuery = Query<TopicInfoQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopicInfoQueryData {
    topic_id: Option<TopicAddress>,
}

impl From<TopicInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TopicInfoQueryData) -> Self {
        Self::TopicInfo(data)
    }
}

impl TopicInfoQuery {
    /// Sets the topic ID for which information is requested.
    pub fn topic_id(&mut self, id: impl Into<TopicAddress>) -> &mut Self {
        self.data.topic_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for TopicInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let topic_id = self.topic_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::CryptoGetInfo(services::CryptoGetInfoQuery {
                topic_id,
                header: Some(header),
            })),
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

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::query::AnyQueryData;
    use crate::{TopicAddress, TopicId, TopicInfoQuery, AnyQuery};

    // language=JSON
    const TOPIC_INFO: &str = r#"{
  "$type": "topicInfo",
  "topicId": "0.0.1001",
  "payment": {
    "amount": 50,
    "transactionMemo": "query payment",
    "payerTopicId": "0.0.6189"
  }
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut query = TopicInfoQuery::new();
        query
            .topic_id(TopicId::from(1001))
            .payer_topic_id(TopicId::from(6189))
            .payment_amount(50)
            .payment_transaction_memo("query payment");

        let s = serde_json::to_string_pretty(&query)?;
        assert_eq!(s, TOPIC_INFO);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let query: AnyQuery = serde_json::from_str(TOPIC_INFO)?;

        let data = assert_matches!(query.data, AnyQueryData::TopicInfo(query) => query);
        let topic_id =
            assert_matches!(data.topic_id, Some(TopicAddress::TopicId(id)) => id);

        assert_eq!(topic_id.num, 1001);
        assert_eq!(query.payment.body.data.amount, Some(50));
        assert_eq!(query.payment.body.transaction_memo, "query payment");
        assert_eq!(query.payment.body.payer_topic_id, Some(TopicId::from(6189)));

        Ok(())
    }
}
