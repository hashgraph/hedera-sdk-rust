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

use std::pin::Pin;

use async_stream::stream;
use async_trait::async_trait;
use backoff::backoff::Backoff;
use backoff::ExponentialBackoff;
use futures_core::Stream;
use futures_util::TryStreamExt;
use tokio::time::sleep;
use tonic::transport::Channel;

use crate::mirror_query::AnyMirrorQueryData;
use crate::{
    Client,
    Error,
    FromProtobuf,
    MirrorQuery,
};

#[async_trait]
pub trait MirrorQuerySubscribe: 'static + Into<AnyMirrorQueryData> + Send + Sync + Clone {
    type GrpcStream: Send;

    type GrpcMessage: Send;

    type Message: Send + FromProtobuf<Self::GrpcMessage>;

    /// Return `true` to retry establishing the stream, up to a configurable maximum timeout.
    #[allow(unused_variables)]
    fn should_retry(&self, status_code: tonic::Code) -> bool {
        false
    }

    async fn subscribe(&self, channel: Channel) -> Result<Self::GrpcStream, tonic::Status>;

    async fn message(
        &self,
        stream: &mut Self::GrpcStream,
    ) -> Result<Option<Self::GrpcMessage>, tonic::Status>;
}

impl<D> MirrorQuery<D>
where
    D: MirrorQuerySubscribe,
{
    /// Execute this query against the provided client of the Hedera network.
    // todo:
    #[allow(clippy::missing_errors_doc)]
    pub async fn execute(&mut self, client: &Client) -> crate::Result<Vec<D::Message>> {
        self.subscribe(client).try_collect().await
    }

    /// Subscribe to this query with the provided client of the Hedera network.
    #[allow(unused_labels)]
    pub fn subscribe(
        &self,
        client: &Client,
    ) -> Pin<Box<dyn Stream<Item = crate::Result<D::Message>> + Send>> {
        let client = client.clone();
        let self_ = self.clone();

        Box::pin(stream! {
            let mut backoff = ExponentialBackoff::default();
            let mut backoff_inf = ExponentialBackoff::default();

            // remove maximum elapsed time for # of back-offs on inf.
            backoff_inf.max_elapsed_time = None;

            loop {
                let status = 'request: loop {
                    // attempt to establish the stream
                    let channel = client.mirror_network().channel();
                    let response = self_.data.subscribe(channel).await;

                    let mut stream = match response {
                        // success, we now have a stream and may begin waiting for messages
                        Ok(stream) => stream,

                        Err(status) => {
                            break 'request status;
                        }
                    };

                    backoff.reset();
                    backoff_inf.reset();

                    'message: loop {
                        let message = self_.data.message(&mut stream).await;

                        let message = match message {
                            Ok(Some(message)) => message,
                            Ok(None) => {
                                // end of stream
                                // hopefully due to configured limits or expected conditions
                                return;
                            }

                            Err(status) => {
                                break 'request status;
                            }
                        };

                        yield D::Message::from_protobuf(message);
                    }
                };

                match status.code() {
                    tonic::Code::Unavailable | tonic::Code::ResourceExhausted => {
                        // encountered a temporarily down or overloaded service
                        sleep(backoff_inf.next_backoff().unwrap()).await;
                    }

                    tonic::Code::Unknown if status.message() == "error reading a body from connection: connection reset" => {
                        // connection was aborted by the server
                        sleep(backoff_inf.next_backoff().unwrap()).await;
                    }

                    code if self_.data.should_retry(code) => {
                        if let Some(duration) = backoff.next_backoff() {
                            sleep(duration).await;
                        } else {
                            // maximum time allowed has elapsed
                            // NOTE: it should be impossible to reach here without capturing at least one error
                            yield Err(Error::TimedOut(Error::from(status).into()));
                            return;
                        }
                    }

                    _ => {
                        // encountered an un-recoverable failure when attempting
                        // to establish the stream
                        yield Err(Error::from(status));
                        return;
                    }
                }
            }
        })
    }
}
