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
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    Error,
    LedgerId,
    Query,
    ToProtobuf,
    TopicId,
    TopicInfo,
    ValidateChecksums,
};

/// Retrieve the latest state of a topic.
pub type TopicInfoQuery = Query<TopicInfoQueryData>;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
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
    /// Returns the topic to retrieve info about.
    #[must_use]
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.data.topic_id
    }

    /// Sets the topic to retrieve info about.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data.topic_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for TopicInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let topic_id = self.topic_id.to_protobuf();

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

impl ValidateChecksums for TopicInfoQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.topic_id.validate_checksums(ledger_id)
    }
}
