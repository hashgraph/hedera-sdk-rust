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

use crate::entity_id::AutoValidateChecksum;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    FileId,
    LedgerId,
    Transaction,
    TransactionId,
};

/// Append the given contents to the end of the specified file.
///
pub type FileAppendTransaction = Transaction<FileAppendTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct FileAppendTransactionData {
    /// The file to which the bytes will be appended.
    file_id: Option<FileId>,

    /// The bytes that will be appended to the end of the specified file.
    contents: Option<Vec<u8>>,
}

impl FileAppendTransaction {
    /// Returns the file to which the bytes will be appended.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data().file_id
    }

    /// Sets the file to which the bytes will be appended.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.require_not_frozen();
        self.data_mut().file_id = Some(id.into());
        self
    }

    /// Retuns the bytes that will be appended to the end of the specified file.
    #[must_use]
    pub fn get_contents(&self) -> Option<&[u8]> {
        self.data().contents.as_deref()
    }

    /// Sets the bytes that will be appended to the end of the specified file.
    pub fn contents(&mut self, contents: Vec<u8>) -> &mut Self {
        self.require_not_frozen();
        self.data_mut().contents = Some(contents);
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

impl FromProtobuf<services::FileAppendTransactionBody> for FileAppendTransactionData {
    fn from_protobuf(pb: services::FileAppendTransactionBody) -> crate::Result<Self> {
        Ok(Self { file_id: Option::from_protobuf(pb.file_id)?, contents: Some(pb.contents) })
    }
}
