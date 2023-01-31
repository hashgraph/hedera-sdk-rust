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
use hedera_proto::services::freeze_service_client::FreezeServiceClient;
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    FileId,
    FreezeType,
    LedgerId,
    ToProtobuf,
    Transaction,
    ValidateChecksums,
};

/// Sets the freezing period in which the platform will stop creating
/// events and accepting transactions.
///
/// This is used before safely shut down the platform for maintenance.
///
pub type FreezeTransaction = Transaction<FreezeTransactionData>;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct FreezeTransactionData {
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    start_time: Option<OffsetDateTime>,
    file_id: Option<FileId>,
    file_hash: Option<Vec<u8>>,
    freeze_type: FreezeType,
}

impl FreezeTransaction {
    /// Returns the start time.
    #[must_use]
    pub fn get_start_time(&self) -> Option<OffsetDateTime> {
        self.data().start_time
    }

    /// Sets the start time.
    pub fn start_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data_mut().start_time = Some(time);
        self
    }

    /// Returns the freeze type.
    #[must_use]
    pub fn get_freeze_type(&self) -> FreezeType {
        self.data().freeze_type
    }

    /// Sets the freeze type.
    pub fn freeze_type(&mut self, ty: FreezeType) -> &mut Self {
        self.data_mut().freeze_type = ty;
        self
    }

    /// Returns the file ID.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data().file_id
    }

    /// Sets the file ID.
    pub fn file_id(&mut self, id: FileId) -> &mut Self {
        self.data_mut().file_id = Some(id);
        self
    }

    /// Returns the file hash.
    #[must_use]
    pub fn get_file_hash(&self) -> Option<&[u8]> {
        self.data().file_hash.as_deref()
    }

    /// Sets the file hash.
    pub fn file_hash(&mut self, hash: Vec<u8>) -> &mut Self {
        self.data_mut().file_hash = Some(hash);
        self
    }
}

impl TransactionExecute for FreezeTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { FreezeServiceClient::new(channel).freeze(request).await })
    }
}

impl ValidateChecksums for FreezeTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.file_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for FreezeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let start_time = self.start_time.map(Into::into);
        let file_id = self.file_id.to_protobuf();

        services::transaction_body::Data::Freeze(services::FreezeTransactionBody {
            update_file: file_id,
            file_hash: self.file_hash.clone().unwrap_or_default(),
            start_time,
            freeze_type: self.freeze_type as _,
            ..Default::default()
        })
    }
}

impl From<FreezeTransactionData> for AnyTransactionData {
    fn from(transaction: FreezeTransactionData) -> Self {
        Self::Freeze(transaction)
    }
}

impl FromProtobuf<services::FreezeTransactionBody> for FreezeTransactionData {
    fn from_protobuf(pb: services::FreezeTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            start_time: pb.start_time.map(Into::into),
            file_id: Option::from_protobuf(pb.update_file)?,
            file_hash: Some(pb.file_hash),
            freeze_type: FreezeType::from(pb.freeze_type),
        })
    }
}
