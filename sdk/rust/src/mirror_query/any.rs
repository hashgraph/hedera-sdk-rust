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
use hedera_proto::{
    mirror,
    services,
};
use tonic::transport::Channel;
use tonic::{
    Code,
    Status,
    Streaming,
};

use crate::mirror_query::MirrorQuerySubscribe;
use crate::topic::TopicMessageQueryData;
use crate::{
    FromProtobuf,
    MirrorQuery,
    NodeAddress,
    NodeAddressBookQueryData,
    TopicMessage,
};

/// Represents any possible query to the mirror network.
pub type AnyMirrorQuery = MirrorQuery<AnyMirrorQueryData>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", tag = "$type"))]
pub enum AnyMirrorQueryData {
    NodeAddressBook(NodeAddressBookQueryData),
    TopicMessage(TopicMessageQueryData),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", tag = "$type"))]
pub enum AnyMirrorQueryMessage {
    NodeAddressBook(NodeAddress),
    TopicMessage(TopicMessage),
}

pub enum AnyMirrorQueryGrpcMessage {
    NodeAddressBook(services::NodeAddress),
    TopicMessage(mirror::ConsensusTopicResponse),
}

pub enum AnyMirrorQueryGrpcStream {
    NodeAddressBook(Streaming<services::NodeAddress>),
    TopicMessage(Streaming<mirror::ConsensusTopicResponse>),
}

/// Represents the response of any possible query to the mirror network.
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(tag = "$type"))]
pub enum AnyMirrorQueryResponse {
    NodeAddressBook(<NodeAddressBookQueryData as MirrorQuerySubscribe>::Response),
    TopicMessage(<TopicMessageQueryData as MirrorQuerySubscribe>::Response),
}

#[async_trait]
impl MirrorQuerySubscribe for AnyMirrorQueryData {
    type GrpcStream = AnyMirrorQueryGrpcStream;

    type GrpcMessage = AnyMirrorQueryGrpcMessage;

    type Message = AnyMirrorQueryMessage;

    type Response = AnyMirrorQueryResponse;

    fn map_response(&self, response: Vec<Self::GrpcMessage>) -> crate::Result<Self::Response> {
        // yes this whole thing is every bit as stupid as it looks, someone please try to find a better design.
        match self {
            AnyMirrorQueryData::NodeAddressBook(it) => {
                let response = response
                    .into_iter()
                    .map(|it| match it {
                        AnyMirrorQueryGrpcMessage::NodeAddressBook(it) => it,
                        AnyMirrorQueryGrpcMessage::TopicMessage(_) => unreachable!(),
                    })
                    .collect();

                it.map_response(response).map(AnyMirrorQueryResponse::NodeAddressBook)
            }

            AnyMirrorQueryData::TopicMessage(it) => {
                let response = response
                    .into_iter()
                    .map(|it| match it {
                        AnyMirrorQueryGrpcMessage::NodeAddressBook(_) => unreachable!(),
                        AnyMirrorQueryGrpcMessage::TopicMessage(it) => it,
                    })
                    .collect();

                it.map_response(response).map(AnyMirrorQueryResponse::TopicMessage)
            }
        }
    }

    fn should_retry(&self, status_code: Code) -> bool {
        match self {
            Self::NodeAddressBook(query) => query.should_retry(status_code),
            Self::TopicMessage(query) => query.should_retry(status_code),
        }
    }

    async fn subscribe(&self, channel: Channel) -> Result<Self::GrpcStream, Status> {
        match self {
            Self::NodeAddressBook(query) => {
                query.subscribe(channel).await.map(AnyMirrorQueryGrpcStream::NodeAddressBook)
            }

            Self::TopicMessage(query) => {
                query.subscribe(channel).await.map(AnyMirrorQueryGrpcStream::TopicMessage)
            }
        }
    }

    async fn message(
        &self,
        stream: &mut Self::GrpcStream,
    ) -> Result<Option<Self::GrpcMessage>, Status> {
        match stream {
            AnyMirrorQueryGrpcStream::NodeAddressBook(stream) => stream
                .message()
                .await
                .map(|message| message.map(AnyMirrorQueryGrpcMessage::NodeAddressBook)),

            AnyMirrorQueryGrpcStream::TopicMessage(stream) => stream
                .message()
                .await
                .map(|message| message.map(AnyMirrorQueryGrpcMessage::TopicMessage)),
        }
    }
}

impl FromProtobuf<AnyMirrorQueryGrpcMessage> for AnyMirrorQueryMessage {
    fn from_protobuf(message: AnyMirrorQueryGrpcMessage) -> crate::Result<Self>
    where
        Self: Sized,
    {
        match message {
            AnyMirrorQueryGrpcMessage::NodeAddressBook(message) => {
                NodeAddress::from_protobuf(message).map(Self::NodeAddressBook)
            }

            AnyMirrorQueryGrpcMessage::TopicMessage(message) => {
                TopicMessage::from_protobuf(message).map(Self::TopicMessage)
            }
        }
    }
}

// NOTE: as we cannot derive serde on MirrorQuery<T> directly as `T`,
//  we create a proxy type that has the same layout but is only for AnyMirrorQueryData and does
//  derive(Deserialize).

#[cfg(feature = "ffi")]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
struct AnyMirrorQueryProxy {
    #[cfg_attr(feature = "ffi", serde(flatten))]
    data: AnyMirrorQueryData,
}

#[cfg(feature = "ffi")]
impl<D> serde::Serialize for MirrorQuery<D>
where
    D: MirrorQuerySubscribe,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: remove the clones, should be possible with Cows

        AnyMirrorQueryProxy { data: self.data.clone().into() }.serialize(serializer)
    }
}

#[cfg(feature = "ffi")]
impl<'de> serde::Deserialize<'de> for AnyMirrorQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <AnyMirrorQueryProxy as serde::Deserialize>::deserialize(deserializer)
            .map(|query| Self { data: query.data })
    }
}
