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
use hedera_proto::mirror;
use hedera_proto::mirror::consensus_service_client::ConsensusServiceClient;
use hedera_proto::mirror::ConsensusTopicQuery;
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::mirror_query::{
    AnyMirrorQueryData,
    MirrorQuerySubscribe,
};
use crate::{
    MirrorQuery,
    ToProtobuf,
    TopicId,
    TopicMessage,
};

// TODO: test, test, and test
// TODO: investigate failure scenarios

/// Query a stream of Hedera Consensus Service (HCS)
/// messages for an HCS Topic via a specific (possibly open-ended) time range.
pub type TopicMessageQuery = MirrorQuery<TopicMessageQueryData>;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TopicMessageQueryData {
    /// The topic ID to retrieve messages for.
    topic_id: Option<TopicId>,

    /// Include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    start_time: Option<OffsetDateTime>,

    /// Include messages which reached consensus before this time.
    end_time: Option<OffsetDateTime>,

    /// The maximum number of messages to receive before stopping.
    limit: u64,
}

impl TopicMessageQuery {
    /// Sets the topic ID to retrieve messages for.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data.topic_id = Some(id.into());
        self
    }

    /// Set to include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    pub fn start_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data.start_time = Some(time);
        self
    }

    /// Set to include messages which reached consensus before this time.
    pub fn end_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data.end_time = Some(time);
        self
    }

    /// Sets the maximum number of messages to be returned, before closing the subscription.
    /// Defaults to _unlimited_.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.data.limit = limit;
        self
    }
}

impl From<TopicMessageQueryData> for AnyMirrorQueryData {
    fn from(data: TopicMessageQueryData) -> Self {
        Self::TopicMessage(data)
    }
}

#[async_trait]
impl MirrorQuerySubscribe for TopicMessageQueryData {
    type GrpcStream = tonic::Streaming<Self::GrpcMessage>;

    type GrpcMessage = mirror::ConsensusTopicResponse;

    type Message = TopicMessage;

    async fn subscribe(&self, channel: Channel) -> Result<Self::GrpcStream, tonic::Status> {
        let topic_id = self.topic_id.as_ref().map(TopicId::to_protobuf);
        let consensus_end_time = self.end_time.map(Into::into);
        let consensus_start_time = self.start_time.map(Into::into);

        let request = ConsensusTopicQuery {
            consensus_end_time,
            consensus_start_time,
            topic_id,
            limit: self.limit,
        };

        ConsensusServiceClient::new(channel)
            .subscribe_topic(request)
            .await
            .map(|response| response.into_inner())
    }

    async fn message(
        &self,
        stream: &mut Self::GrpcStream,
    ) -> Result<Option<Self::GrpcMessage>, tonic::Status> {
        stream.message().await
    }
}
