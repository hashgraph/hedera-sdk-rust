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
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    BoxGrpcFuture,
    Error,
    FileId,
    FileInfo,
    LedgerId,
    Query,
    ToProtobuf,
    ValidateChecksums,
};

/// Get all the information about a file.
pub type FileInfoQuery = Query<FileInfoQueryData>;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct FileInfoQueryData {
    file_id: Option<FileId>,
}

impl From<FileInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: FileInfoQueryData) -> Self {
        Self::FileInfo(data)
    }
}

impl FileInfoQuery {
    /// Returns the ID of the file for which information is requested.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data.file_id
    }

    /// Sets the ID of the file for which information is requested.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data.file_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for FileInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let file_id = self.file_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::FileGetInfo(services::FileGetInfoQuery {
                file_id,
                header: Some(header),
            })),
        }
    }
}

impl QueryExecute for FileInfoQueryData {
    type Response = FileInfo;

    fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> BoxGrpcFuture<'_, services::Response> {
        Box::pin(async { FileServiceClient::new(channel).get_file_info(request).await })
    }
}

impl ValidateChecksums for FileInfoQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.file_id.validate_checksums(ledger_id)
    }
}

// hack(sr): these tests currently don't compile due to `payer_account_id`
#[cfg(feature = "false")]
mod tests {
    use assert_matches::assert_matches;

    use crate::query::AnyQueryData;
    use crate::{
        AccountId,
        AnyQuery,
        FileId,
        FileInfoQuery,
    };

    // language=JSON
    const FILE_INFO: &str = r#"{
  "$type": "fileInfo",
  "fileId": "0.0.1001",
  "payment": {
    "amount": 50,
    "transactionMemo": "query payment",
    "payerAccountId": "0.0.6189"
  }
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut query = FileInfoQuery::new();
        query
            .file_id(FileId::from(1001))
            .payer_account_id(AccountId::from(6189))
            .payment_amount(50)
            .payment_transaction_memo("query payment");

        let s = serde_json::to_string_pretty(&query)?;
        assert_eq!(s, FILE_INFO);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let query: AnyQuery = serde_json::from_str(FILE_INFO)?;

        let data = assert_matches!(query.data, AnyQueryData::FileInfo(query) => query);

        assert_eq!(data.file_id, Some(FileId { shard: 0, realm: 0, num: 1001 }));
        assert_eq!(query.payment.body.data.amount, Some(50));
        assert_eq!(query.payment.body.transaction_memo, "query payment");
        assert_eq!(query.payment.body.payer_account_id, Some(AccountId::from(6189)));

        Ok(())
    }
}
