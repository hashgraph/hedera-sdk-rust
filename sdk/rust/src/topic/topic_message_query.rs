use std::pin::Pin;

use async_stream::stream;
use backoff::backoff::Backoff;
use backoff::ExponentialBackoff;
use futures_core::Stream;
use hedera_proto::mirror::consensus_service_client::ConsensusServiceClient;
use hedera_proto::mirror::{ConsensusTopicQuery, ConsensusTopicResponse};
use time::OffsetDateTime;
use tokio::time::sleep;

use crate::{Client, Error, ToProtobuf, TopicId};

// TODO: test, test, and test
// TODO: investigate failure scenarios

#[derive(Debug, Default, Clone)]
pub struct TopicMessageQuery {
    topic_id: Option<TopicId>,
    start_time: Option<OffsetDateTime>,
    end_time: Option<OffsetDateTime>,
    limit: u64,
}

impl TopicMessageQuery {
    /// Create a new query ready for configuration and execution.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the topic ID to retrieve messages for.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.topic_id = Some(id.into());
        self
    }

    /// Set to include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    pub fn start_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.start_time = Some(time);
        self
    }

    /// Set to include messages which reached consensus before this time.
    pub fn end_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.end_time = Some(time);
        self
    }

    /// Sets the maximum number of messages to be returned, before closing the subscription.
    /// Defaults to _unlimited_.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Subscribe to the configured topic.
    pub fn subscribe(
        &self,
        client: &Client,
    ) -> Pin<Box<dyn Stream<Item = crate::Result<ConsensusTopicResponse>>>> {
        let self_ = self.clone();
        let client = client.clone();
        let channel = client.mirror_network().channel();
        let mut client = ConsensusServiceClient::new(channel);

        Box::pin(stream! {
            let mut backoff = ExponentialBackoff::default();
            let mut attempt = 0;

            // remove maximum elapsed time for # of back-offs
            backoff.max_elapsed_time = None;

            loop {
                'backoff: loop {
                    let request = self_.make_request();
                    let response = client.subscribe_topic(request).await;

                    let mut stream = match response {
                        Ok(stream) => stream.into_inner(),

                        Err(status) if self_.should_retry_status(&status) => {
                            break 'backoff;
                        }

                        Err(status) => {
                            yield Err(Error::from(status));
                            return;
                        }
                    };

                    loop {
                        let response = stream.message().await;

                        let message = match response {
                            Ok(Some(message)) => message,

                            Ok(None) => {
                                // end of stream detected
                                // likely reached the "end" according to the limits set in
                                // the query
                                return;
                            }

                            Err(status) if self_.should_retry_status(&status) => {
                                break 'backoff;
                            }

                            Err(status) => {
                                yield Err(Error::from(status));
                                return;
                            }
                        };

                        if attempt > 0 {
                            // cycle success, reset the backoff
                            backoff.reset();
                            attempt = 0;
                        }

                        yield Ok(message);
                    }
                }

                attempt += 1;
                sleep(backoff.next_backoff().unwrap()).await;
            }
        })
    }

    fn should_retry_status(&self, status: &tonic::Status) -> bool {
        match status.code() {
            // UNAVAILABLE: Can occur when the mirror node's database or other downstream components are temporarily down.
            //
            // RESOURCE_EXHAUSTED: Can occur when the mirror node's resources (database, threads, etc.) are temporarily exhausted.
            //
            // NOT_FOUND: Can occur when a client creates a topic and attempts to subscribe to it immediately
            //  before it is available in the mirror node.
            //
            tonic::Code::Unavailable | tonic::Code::ResourceExhausted | tonic::Code::NotFound => {
                true
            }

            // UNKNOWN: Connection was reset, this means the server terminated our connection.
            tonic::Code::Unknown
                if status.message() == "error reading a body from connection: connection reset" =>
            {
                true
            }

            _ => false,
        }
    }

    fn make_request(&self) -> ConsensusTopicQuery {
        let topic_id = self.topic_id.as_ref().map(TopicId::to_protobuf);
        let consensus_end_time = self.end_time.map(Into::into);
        let consensus_start_time = self.start_time.map(Into::into);

        ConsensusTopicQuery {
            consensus_end_time,
            consensus_start_time,
            topic_id,
            limit: self.limit,
        }
    }
}
