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

use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    FileId,
    Key,
    KeyList,
    Transaction,
    ValidateChecksums,
};

/// Modify the metadata and/or the contents of a file.
///
/// If a field is not set in the transaction body, the
/// corresponding file attribute will be unchanged.
///
pub type FileUpdateTransaction = Transaction<FileUpdateTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct FileUpdateTransactionData {
    /// The file ID which is being updated in this transaction.
    file_id: Option<FileId>,

    /// The memo associated with the file.
    file_memo: Option<String>,

    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    keys: Option<KeyList>,

    /// The bytes that are to be the contents of the file.
    contents: Option<Vec<u8>>,

    /// The time at which this file should expire.
    expiration_time: Option<OffsetDateTime>,

    auto_renew_account_id: Option<AccountId>,

    auto_renew_period: Option<Duration>,
}

impl FileUpdateTransaction {
    /// Returns the ID of the file which is being updated.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data().file_id
    }

    /// Sets the ID of the file which is being updated.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data_mut().file_id = Some(id.into());
        self
    }

    /// Returns the new memo for the file.
    #[must_use]
    pub fn get_file_memo(&self) -> Option<&str> {
        self.data().file_memo.as_deref()
    }

    /// Sets the new memo to be associated with the file.
    pub fn file_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().file_memo = Some(memo.into());
        self
    }

    /// Returns the bytes that are to be the contents of the file.
    #[must_use]
    pub fn get_contents(&self) -> Option<&[u8]> {
        self.data().contents.as_deref()
    }

    /// Sets the bytes that are to be the contents of the file.
    pub fn contents(&mut self, contents: Vec<u8>) -> &mut Self {
        self.data_mut().contents = Some(contents);
        self
    }

    /// Returns the keys for this file.
    #[must_use]
    pub fn get_keys(&self) -> Option<&KeyList> {
        self.data().keys.as_ref()
    }

    /// Sets the keys for this file.
    ///
    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    pub fn keys<K: Into<Key>>(&mut self, keys: impl IntoIterator<Item = K>) -> &mut Self {
        self.data_mut().keys = Some(keys.into_iter().map(Into::into).collect());
        self
    }

    /// Returns the time at which this file should expire.
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the time at which this file should expire.
    pub fn expiration_time(&mut self, at: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(at);
        self
    }

    /// Returns the account to be used at the file's expiration time to extend the
    /// life of the file.
    ///
    /// # Network Support
    /// Please note that this not supported on any hedera network at this time.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the account to be used at the files's expiration time to extend the
    /// life of the file.
    ///
    /// # Network Support
    /// Please note that this not supported on any hedera network at this time.
    pub fn auto_renew_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(id);
        self
    }

    /// Returns the auto renew period for this file.
    ///
    /// # Network Support
    /// Please note that this not supported on any hedera network at this time.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the auto renew period for this file.
    ///
    /// # Network Support
    /// Please note that this not supported on any hedera network at this time.
    pub fn auto_renew_period(&mut self, duration: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(duration);
        self
    }
}

impl TransactionData for FileUpdateTransactionData {}

impl TransactionExecute for FileUpdateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { FileServiceClient::new(channel).update_file(request).await })
    }
}

impl ValidateChecksums for FileUpdateTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.file_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for FileUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::FileUpdate(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for FileUpdateTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::FileUpdate(self.to_protobuf())
    }
}

impl From<FileUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: FileUpdateTransactionData) -> Self {
        Self::FileUpdate(transaction)
    }
}

impl FromProtobuf<services::FileUpdateTransactionBody> for FileUpdateTransactionData {
    fn from_protobuf(pb: services::FileUpdateTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            file_id: Option::from_protobuf(pb.file_id)?,
            file_memo: pb.memo,
            keys: Option::from_protobuf(pb.keys)?,
            contents: Some(pb.contents),
            expiration_time: pb.expiration_time.map(Into::into),
            auto_renew_account_id: None,
            auto_renew_period: None,
        })
    }
}

impl ToProtobuf for FileUpdateTransactionData {
    type Protobuf = services::FileUpdateTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::FileUpdateTransactionBody {
            file_id: self.file_id.to_protobuf(),
            expiration_time: self.expiration_time.to_protobuf(),
            keys: self.keys.to_protobuf(),
            contents: self.contents.clone().unwrap_or_default(),
            memo: self.file_memo.clone(),
        }
    }
}
