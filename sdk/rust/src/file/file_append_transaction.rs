use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use serde_with::base64::Base64;
use serde_with::{
    serde_as,
    skip_serializing_none,
};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    FileId,
    Transaction,
    TransactionId,
};

/// Append the given contents to the end of the specified file.
///
pub type FileAppendTransaction = Transaction<FileAppendTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileAppendTransactionData {
    /// The file to which the bytes will be appended.
    file_id: Option<FileId>,

    /// The bytes that will be appended to the end of the specified file.
    #[serde_as(as = "Option<Base64>")]
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

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::transaction::{
        AnyTransaction,
        AnyTransactionData,
    };
    use crate::{
        FileAppendTransaction,
        FileId,
    };

    // language=JSON
    const FILE_APPEND_TRANSACTION_JSON: &str = r#"{
  "$type": "fileAppend",
  "fileId": "0.0.1001",
  "contents": "QXBwZW5kaW5nIHRoZXNlIGJ5dGVzIHRvIGZpbGUgMTAwMQ=="
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = FileAppendTransaction::new();

        transaction
            .file_id(FileId::from(1001))
            .contents("Appending these bytes to file 1001".into());

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, FILE_APPEND_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(FILE_APPEND_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::FileAppend(transaction) => transaction);

        assert_eq!(data.file_id.unwrap(), FileId::from(1001));

        let bytes: Vec<u8> = "Appending these bytes to file 1001".into();
        assert_eq!(data.contents.unwrap(), bytes);

        Ok(())
    }
}
