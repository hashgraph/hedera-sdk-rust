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
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{AccountId, Error, FileId, LedgerId, Transaction, TransactionId};
use crate::entity_id::AutoValidateChecksum;

/// Append the given contents to the end of the specified file.
///
pub type FileAppendTransaction = Transaction<FileAppendTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct FileAppendTransactionData {
    /// The file to which the bytes will be appended.
    file_id: Option<FileId>,

    /// The bytes that will be appended to the end of the specified file.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::base64::Base64>>")
    )]
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
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.file_id.validate_checksum_for_ledger_id(ledger_id)
    }

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
        let file_id = self.file_id.to_protobuf();

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
    #[cfg(feature = "ffi")]
    mod ffi {
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
}
