use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, FileId, Transaction, TransactionId};

/// Delete the given file.
///
/// After deletion, it will be marked as deleted and will have no contents.
/// Information about it will continue to exist until it expires.
///
pub type FileDeleteTransaction = Transaction<FileDeleteTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDeleteTransactionData {
    /// The file to delete. It will be marked as deleted until it expires.
    /// Then it will disappear.
    file_id: Option<FileId>,
}

impl FileDeleteTransaction {
    /// Set the file to delete.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.body.data.file_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for FileDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        FileServiceClient::new(channel).delete_file(request).await
    }
}

impl ToTransactionDataProtobuf for FileDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let file_id = self.file_id.as_ref().map(FileId::to_protobuf);

        services::transaction_body::Data::FileDelete(services::FileDeleteTransactionBody {
            file_id,
        })
    }
}

impl From<FileDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: FileDeleteTransactionData) -> Self {
        Self::FileDelete(transaction)
    }
}
