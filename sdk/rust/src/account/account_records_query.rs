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
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    AccountId,
    Error,
    FromProtobuf,
    LedgerId,
    Query,
    ToProtobuf,
    TransactionRecord,
    ValidateChecksums,
};

/// Get all the records for an account for any transfers into it and out of it,
/// that were above the threshold, during the last 25 hours.
pub type AccountRecordsQuery = Query<AccountRecordsQueryData>;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct AccountRecordsQueryData {
    account_id: Option<AccountId>,
}

impl From<AccountRecordsQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountRecordsQueryData) -> Self {
        Self::AccountRecords(data)
    }
}

impl AccountRecordsQuery {
    /// Gets the account ID for which the records should be retrieved.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data.account_id
    }

    /// Sets the account ID for which the records should be retrieved.
    pub fn account_id(&mut self, id: AccountId) -> &mut Self {
        self.data.account_id = Some(id);
        self
    }
}

impl ToQueryProtobuf for AccountRecordsQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let account_id = self.account_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::CryptoGetAccountRecords(
                services::CryptoGetAccountRecordsQuery { account_id, header: Some(header) },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for AccountRecordsQueryData {
    type Response = Vec<TransactionRecord>;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_account_records(request).await
    }
}

impl ValidateChecksums for AccountRecordsQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.account_id.validate_checksums(ledger_id)
    }
}

impl FromProtobuf<services::response::Response> for Vec<TransactionRecord> {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, CryptoGetAccountRecords, services::response::Response);

        Vec::<TransactionRecord>::from_protobuf(response.records)
    }
}
