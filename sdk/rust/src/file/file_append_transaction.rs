use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, FileId, Transaction, TransactionId};

/// Append the given contents to the end of the specified file.
///
pub type FileAppendTransaction = Transaction<FileAppendTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileAppendTransactionData {
    /// The file to which the bytes will be appended.
    file_id: Option<FileId>,

    /// The bytes that will be appended to the end of the specified file.
    contents: Option<Vec<u8>>,
}

impl FileAppendTransaction {
    /// Sets the file to which the bytes will be appended.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.body.data.file_id = Some(id.into());
        self
    }

    /// Sets the bytes that will be appended to the end of the specified file.
    pub fn contents(&mut self, contents: Vec<u8>) -> &mut Self {
        self.body.data.contents = Some(contents);
        self
    }
}

#[async_trait]
impl TransactionExecute for FileAppendTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        FileServiceClient::new(channel).append_content(request).await
    }
}

impl ToTransactionDataProtobuf for FileAppendTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let file_id = self.file_id.as_ref().map(FileId::to_protobuf);

        services::transaction_body::Data::FileAppend(services::FileAppendTransactionBody {
            file_id,
            contents: self.contents.clone().unwrap_or_default(),
        })
    }
}

impl From<FileAppendTransactionData> for AnyTransactionData {
    fn from(transaction: FileAppendTransactionData) -> Self {
        Self::FileAppend(transaction)
    }
}
