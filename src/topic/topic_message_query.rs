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

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_core::Stream;
use futures_util::TryStreamExt;
use hedera_proto::mirror;
use hedera_proto::mirror::consensus_service_client::ConsensusServiceClient;
use hedera_proto::mirror::ConsensusTopicQuery;
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;
use tonic::Response;

use crate::mirror_query::{
    AnyMirrorQueryData,
    AnyMirrorQueryMessage,
    MirrorRequest,
};
use crate::protobuf::FromProtobuf;
use crate::{
    AnyMirrorQueryResponse,
    MirrorQuery,
    ToProtobuf,
    TopicId,
    TopicMessage,
};

// TODO: test, test, and test
// TODO: investigate failure scenarios

// TODO: validate checksums after PR is merged

#[derive(Default)]
pub struct TopicMessageQueryContext {
    start_time: Option<OffsetDateTime>,
}

/// Query a stream of Hedera Consensus Service (HCS)
/// messages for an HCS Topic via a specific (possibly open-ended) time range.
pub type TopicMessageQuery = MirrorQuery<TopicMessageQueryData>;

#[derive(Debug, Default, Clone)]
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

impl TopicMessageQueryData {
    fn map_stream<'a, S>(stream: S) -> impl Stream<Item = crate::Result<TopicMessage>>
    where
        S: Stream<Item = crate::Result<mirror::ConsensusTopicResponse>> + Send + 'a,
    {
        stream.and_then(|it| std::future::ready(TopicMessage::from_protobuf(it)))
    }
}

impl TopicMessageQuery {
    /// Returns the ID of the topic to retrieve messages for.
    #[must_use]
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.data.topic_id
    }

    /// Sets the topic ID to retrieve messages for.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data.topic_id = Some(id.into());
        self
    }

    /// Returns the minimum `consensus_timestamp` of the messages to return.
    #[must_use]
    pub fn get_start_time(&self) -> Option<OffsetDateTime> {
        self.data.start_time
    }

    /// Sets to include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    pub fn start_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data.start_time = Some(time);
        self
    }

    /// Returns the maximum `consensus_timestamp` of the messages to return.
    #[must_use]
    pub fn get_end_time(&self) -> Option<OffsetDateTime> {
        self.data.end_time
    }

    /// Sets to include messages which reached consensus before this time.
    pub fn end_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data.end_time = Some(time);
        self
    }

    /// Returns maximum number of messages to be returned.
    #[must_use]
    pub fn get_limit(&self) -> u64 {
        self.data.limit
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

impl MirrorRequest for TopicMessageQueryData {
    type GrpcItem = mirror::ConsensusTopicResponse;

    type ConnectStream = tonic::Streaming<Self::GrpcItem>;

    type Context = TopicMessageQueryContext;

    type Item = TopicMessage;

    type Response = Vec<TopicMessage>;

    type ItemStream<'a> = BoxStream<'a, crate::Result<TopicMessage>>;

    fn connect(
        &self,
        context: &Self::Context,
        channel: Channel,
    ) -> BoxFuture<'_, tonic::Result<Self::ConnectStream>> {
        let topic_id = self.topic_id.to_protobuf();

        let consensus_end_time = self.end_time.map(Into::into);

        // If we had to reconnect, we want to start 1ns after the last message we recieved.
        // We don't want to start *at* the last message we recieved because that'd give us that message again.
        let consensus_start_time = context
            .start_time
            .map(|it| it.checked_add(Duration::nanoseconds(1)).unwrap())
            .or(self.start_time)
            .map(Into::into);

        let request = ConsensusTopicQuery {
            consensus_end_time,
            consensus_start_time,
            topic_id,
            limit: self.limit,
        };

        Box::pin(async {
            ConsensusServiceClient::new(channel)
                .subscribe_topic(request)
                .await
                .map(Response::into_inner)
        })
    }

    fn make_item_stream<'a, S>(stream: S) -> Self::ItemStream<'a>
    where
        S: Stream<Item = crate::Result<Self::GrpcItem>> + Send + 'a,
    {
        Box::pin(Self::map_stream(stream))
    }

    fn try_collect<'a, S>(stream: S) -> BoxFuture<'a, crate::Result<Self::Response>>
    where
        S: Stream<Item = crate::Result<Self::GrpcItem>> + Send + 'a,
    {
        // this doesn't reuse the work in `make_item_stream`
        Box::pin(Self::map_stream(stream).try_collect())
    }

    fn update_context(context: &mut Self::Context, item: &Self::GrpcItem) {
        context.start_time =
            item.consensus_timestamp.map(OffsetDateTime::from).or(context.start_time);
    }
}

impl From<TopicMessage> for AnyMirrorQueryMessage {
    fn from(value: TopicMessage) -> Self {
        Self::TopicMessage(value)
    }
}

impl From<Vec<TopicMessage>> for AnyMirrorQueryResponse {
    fn from(value: Vec<TopicMessage>) -> Self {
        Self::TopicMessage(value)
    }
}
