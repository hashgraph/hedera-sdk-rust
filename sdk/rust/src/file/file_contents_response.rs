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

use crate::{
    FileId,
    FromProtobuf,
};

/// Response from [`FileContentsQuery`][crate::FileContentsQuery].
#[derive(Debug, Clone)]
pub struct FileContentsResponse {
    /// The file ID of the file whose contents are being returned.
    pub file_id: FileId,

    // TODO: .contents vs .bytes (?)
    /// The bytes contained in the file.
    pub contents: Vec<u8>,
}

impl FromProtobuf<services::response::Response> for FileContentsResponse {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, FileGetContents, services::response::Response);
        let file_contents = pb_getf!(pb, file_contents)?;
        let file_id = pb_getf!(file_contents, file_id)?;

        let contents = file_contents.contents;
        let file_id = FileId::from_protobuf(file_id)?;

        Ok(Self { file_id, contents })
    }
}
