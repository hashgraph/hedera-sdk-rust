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
use futures_util::{
    TryFutureExt,
    TryStreamExt,
};
use hedera_proto::{
    mirror,
    services,
};
use mirror::network_service_client::NetworkServiceClient;
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
    FileId,
    MirrorQuery,
    NodeAddress,
    NodeAddressBook,
    ToProtobuf,
};

// TODO: validate checksums after PR is merged

/// Query for an address book and return its nodes.
/// The nodes are returned in ascending order by node ID.
pub type NodeAddressBookQuery = MirrorQuery<NodeAddressBookQueryData>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct NodeAddressBookQueryData {
    /// The ID of the address book file on the network.
    /// Can either be `0.0.101` or `0.0.102`. Defaults to `0.0.102`.
    file_id: FileId,

    /// The maximum number of node addresses to receive.
    /// Defaults to _all_.
    limit: u32,
}

impl NodeAddressBookQueryData {
    fn map_stream<'a, S>(stream: S) -> impl Stream<Item = crate::Result<NodeAddress>>
    where
        S: Stream<Item = crate::Result<services::NodeAddress>> + Send + 'a,
    {
        stream.and_then(|it| std::future::ready(NodeAddress::from_protobuf(it)))
    }
}

impl Default for NodeAddressBookQueryData {
    fn default() -> Self {
        Self { file_id: FileId::from(102), limit: 0 }
    }
}

impl NodeAddressBookQuery {
    /// Returns the file ID of the address book file on the network.
    #[must_use]
    pub fn get_file_id(&self) -> FileId {
        self.data.file_id
    }

    /// Sets the ID of the address book file on the network.
    /// Can either be `0.0.101` or `0.0.102`. Defaults to `0.0.102`.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data.file_id = id.into();
        self
    }

    /// Returns the configured limit of node addresses to receive.
    #[must_use]
    pub fn get_limit(&self) -> u32 {
        self.data.limit
    }

    /// Sets the maximum number of node addresses to receive.
    /// Defaults to _all_.
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.data.limit = limit;
        self
    }
}

impl From<NodeAddressBookQueryData> for AnyMirrorQueryData {
    fn from(data: NodeAddressBookQueryData) -> Self {
        Self::NodeAddressBook(data)
    }
}

impl MirrorRequest for NodeAddressBookQueryData {
    type GrpcItem = services::NodeAddress;

    type ConnectStream = tonic::Streaming<Self::GrpcItem>;

    type Item = NodeAddress;

    type Response = NodeAddressBook;

    type ItemStream<'a> = BoxStream<'a, crate::Result<NodeAddress>>;

    fn connect(&self, channel: Channel) -> BoxFuture<'_, tonic::Result<Self::ConnectStream>> {
        Box::pin(async {
            let file_id = self.file_id.to_protobuf();
            let request =
                mirror::AddressBookQuery { file_id: Some(file_id), limit: self.limit as i32 };

            NetworkServiceClient::new(channel).get_nodes(request).await.map(Response::into_inner)
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
        Box::pin(
            Self::map_stream(stream)
                .try_collect()
                .map_ok(|addresses| NodeAddressBook { node_addresses: addresses }),
        )
    }
}

impl From<NodeAddress> for AnyMirrorQueryMessage {
    fn from(value: NodeAddress) -> Self {
        Self::NodeAddressBook(value)
    }
}

impl From<NodeAddressBook> for AnyMirrorQueryResponse {
    fn from(value: NodeAddressBook) -> Self {
        Self::NodeAddressBook(value)
    }
}
