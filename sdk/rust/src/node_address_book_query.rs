use async_trait::async_trait;
use hedera_proto::{
    mirror,
    services,
};
use mirror::network_service_client::NetworkServiceClient;
use tonic::transport::Channel;

use crate::mirror_query::{
    AnyMirrorQueryData,
    MirrorQuerySubscribe,
};
use crate::{
    FileId,
    MirrorQuery,
    NodeAddress,
    ToProtobuf,
};

/// Query for an address book and return its nodes.
/// The nodes are returned in ascending order by node ID.
pub type NodeAddressBookQuery = MirrorQuery<NodeAddressBookQueryData>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NodeAddressBookQueryData {
    /// The ID of the address book file on the network.
    /// Can either be `0.0.101` or `0.0.102`. Defaults to `0.0.102`.
    file_id: FileId,

    /// The maximum number of node addresses to receive.
    /// Defaults to _all_.
    limit: u32,
}

impl Default for NodeAddressBookQueryData {
    fn default() -> Self {
        Self { file_id: FileId::from(102), limit: 0 }
    }
}

impl NodeAddressBookQuery {
    /// Sets the ID of the address book file on the network.
    /// Can either be `0.0.101` or `0.0.102`. Defaults to `0.0.102`.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data.file_id = id.into();
        self
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

#[async_trait]
impl MirrorQuerySubscribe for NodeAddressBookQueryData {
    type GrpcStream = tonic::Streaming<services::NodeAddress>;

    type GrpcMessage = services::NodeAddress;

    type Message = NodeAddress;

    async fn subscribe(&self, channel: Channel) -> Result<Self::GrpcStream, tonic::Status> {
        let file_id = self.file_id.to_protobuf();
        let request = mirror::AddressBookQuery { file_id: Some(file_id), limit: self.limit as i32 };

        NetworkServiceClient::new(channel)
            .get_nodes(request)
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
