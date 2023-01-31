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
    LedgerId,
    Query,
    Status,
    ToProtobuf,
    TransactionId,
    TransactionReceipt,
    ValidateChecksums,
};

/// Get the receipt of a transaction, given its transaction ID.
///
/// Once a transaction reaches consensus, then information about whether it succeeded or failed
/// will be available until the end of the receipt period.
///
pub type TransactionReceiptQuery = Query<TransactionReceiptQueryData>;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TransactionReceiptQueryData {
    transaction_id: Option<TransactionId>,
    include_children: bool,
    include_duplicates: bool,
    validate_status: bool,
}

impl From<TransactionReceiptQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TransactionReceiptQueryData) -> Self {
        Self::TransactionReceipt(data)
    }
}

impl TransactionReceiptQuery {
    /// Get the ID of the transaction for which the receipt is being requested.
    #[must_use]
    pub fn get_transaction_id(&self) -> Option<TransactionId> {
        self.data.transaction_id
    }

    /// Sets the ID of the transaction for which the receipt is being requested.
    pub fn transaction_id(&mut self, transaction_id: TransactionId) -> &mut Self {
        self.data.transaction_id = Some(transaction_id);
        self
    }

    /// Whether the response should include the receipts of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    #[must_use]
    pub fn get_include_children(&self) -> bool {
        self.data.include_children
    }

    /// Whether the response should include the receipts of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    pub fn include_children(&mut self, include: bool) -> &mut Self {
        self.data.include_children = include;
        self
    }

    /// Whether receipts of processing duplicate transactions should be returned.
    #[must_use]
    pub fn get_include_duplicates(&self) -> bool {
        self.data.include_duplicates
    }

    /// Whether receipts of processing duplicate transactions should be returned.
    pub fn include_duplicates(&mut self, include: bool) -> &mut Self {
        self.data.include_duplicates = include;
        self
    }

    /// Whether the receipt status should be validated.
    #[must_use]
    pub fn get_validate_status(&self) -> bool {
        self.data.validate_status
    }

    /// Whether the receipt status should be validated.
    pub fn validate_status(&mut self, validate: bool) -> &mut Self {
        self.data.validate_status = validate;
        self
    }
}

impl ToQueryProtobuf for TransactionReceiptQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let transaction_id = self.transaction_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::TransactionGetReceipt(
                services::TransactionGetReceiptQuery {
                    header: Some(header),
                    transaction_id,
                    include_child_receipts: self.include_children,
                    include_duplicates: self.include_duplicates,
                },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for TransactionReceiptQueryData {
    type Response = TransactionReceipt;

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
        CryptoServiceClient::new(channel).get_transaction_receipts(request).await
    }

    fn should_retry_pre_check(&self, status: Status) -> bool {
        matches!(status, Status::ReceiptNotFound | Status::RecordNotFound)
    }

    fn should_retry(&self, response: &services::Response) -> bool {
        // extract the receipt status from the receipt
        // without altering or freeing the memory from the response

        let receipt_status = {
            let r = match &response.response {
                Some(services::response::Response::TransactionGetReceipt(r)) => r,
                _ => return false,
            };

            match r.receipt.as_ref().and_then(|it| Status::from_i32(it.status)) {
                Some(receipt_status) => receipt_status,
                None => return false,
            }
        };

        matches!(receipt_status, Status::Unknown)
    }

    fn make_response(&self, response: Response) -> crate::Result<Self::Response> {
        let receipt = TransactionReceipt::from_response_protobuf(response, self.transaction_id)?;

        if self.validate_status && receipt.status != Status::Success {
            return Err(Error::ReceiptStatus {
                transaction_id: self.transaction_id,
                status: receipt.status,
            });
        }

        Ok(receipt)
    }
}

impl ValidateChecksums for TransactionReceiptQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.transaction_id.validate_checksums(ledger_id)
    }
}
