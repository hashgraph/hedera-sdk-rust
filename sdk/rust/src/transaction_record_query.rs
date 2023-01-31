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
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use hedera_proto::services::response::Response;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    Error,
    FromProtobuf,
    LedgerId,
    Query,
    Status,
    ToProtobuf,
    TransactionId,
    TransactionRecord,
    ValidateChecksums,
};

/// Get the record of a transaction, given its transaction ID.
///
pub type TransactionRecordQuery = Query<TransactionRecordQueryData>;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TransactionRecordQueryData {
    transaction_id: Option<TransactionId>,
    include_children: bool,
    include_duplicates: bool,
    validate_status: bool,
}

impl From<TransactionRecordQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TransactionRecordQueryData) -> Self {
        Self::TransactionRecord(data)
    }
}

impl TransactionRecordQuery {
    /// Get the ID of the transaction for which the record is being requested.
    #[must_use]
    pub fn get_transaction_id(&self) -> Option<TransactionId> {
        self.data.transaction_id
    }

    /// Sets the ID of the transaction for which the record is being requested.
    pub fn transaction_id(&mut self, transaction_id: TransactionId) -> &mut Self {
        self.data.transaction_id = Some(transaction_id);
        self
    }

    /// Whether the response should include the records of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    #[must_use]
    pub fn get_include_children(&self) -> bool {
        self.data.include_children
    }

    /// Whether the response should include the records of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    pub fn include_children(&mut self, include: bool) -> &mut Self {
        self.data.include_children = include;
        self
    }

    /// Whether records of processing duplicate transactions should be returned.
    #[must_use]
    pub fn get_include_duplicates(&self) -> bool {
        self.data.include_duplicates
    }

    /// Whether records of processing duplicate transactions should be returned.
    pub fn include_duplicates(&mut self, include: bool) -> &mut Self {
        self.data.include_duplicates = include;
        self
    }

    /// Whether records of processing duplicate transactions should be returned.
    #[must_use]
    pub fn get_validate_status(&self) -> bool {
        self.data.validate_status
    }

    /// Whether the record status should be validated.
    pub fn validate_status(&mut self, validate: bool) -> &mut Self {
        self.data.validate_status = validate;
        self
    }
}

impl ToQueryProtobuf for TransactionRecordQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let transaction_id = self.transaction_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::TransactionGetRecord(
                services::TransactionGetRecordQuery {
                    header: Some(header),
                    transaction_id,
                    include_child_records: self.include_children,
                    include_duplicates: self.include_duplicates,
                },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for TransactionRecordQueryData {
    type Response = TransactionRecord;

    fn is_payment_required(&self) -> bool {
        false
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.transaction_id
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_tx_record_by_tx_id(request).await
    }

    fn should_retry_pre_check(&self, status: Status) -> bool {
        matches!(status, Status::ReceiptNotFound | Status::RecordNotFound)
    }

    fn make_response(&self, response: Response) -> crate::Result<Self::Response> {
        let record = TransactionRecord::from_protobuf(response)?;

        if self.validate_status && record.receipt.status != Status::Success {
            return Err(Error::ReceiptStatus {
                transaction_id: self.transaction_id,
                status: record.receipt.status,
            });
        }

        Ok(record)
    }
}

impl ValidateChecksums for TransactionRecordQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.transaction_id.validate_checksums(ledger_id)
    }
}
