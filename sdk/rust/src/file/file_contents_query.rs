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

use crate::query::{
    AnyQueryData,
    Query,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    Error,
    FileContentsResponse,
    FileId,
    LedgerId,
    ToProtobuf,
    ValidateChecksums,
};

/// Get the contents of a file.
pub type FileContentsQuery = Query<FileContentsQueryData>;

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct FileContentsQueryData {
    /// The file ID for which contents are requested.
    file_id: Option<FileId>,
}

impl From<FileContentsQueryData> for AnyQueryData {
    #[inline]
    fn from(data: FileContentsQueryData) -> Self {
        Self::FileContents(data)
    }
}

impl FileContentsQuery {
    /// Returns the ID of the file for which contents are requested.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data.file_id
    }

    /// Sets the file ID for which contents are requested.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data.file_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for FileContentsQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        services::Query {
            query: Some(services::query::Query::FileGetContents(services::FileGetContentsQuery {
                header: Some(header),
                file_id: self.file_id.to_protobuf(),
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for FileContentsQueryData {
    type Response = FileContentsResponse;

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.file_id.validate_checksums_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        FileServiceClient::new(channel).get_file_content(request).await
    }
}
