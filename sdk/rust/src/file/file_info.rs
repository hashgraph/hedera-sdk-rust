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
use time::OffsetDateTime;

use crate::{
    FileId,
    FromProtobuf,
    LedgerId,
};

/// Response from [`FileInfoQuery`][crate::FileInfoQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    /// The file ID of the file for which information is requested.
    pub file_id: FileId,

    /// Number of bytes in contents.
    pub size: u64,

    /// Current time which this account is set to expire.
    pub expiration_time: Option<OffsetDateTime>,

    /// True if deleted but not yet expired.
    pub is_deleted: bool,

    /// One of these keys must sign in order to modify or delete the file.
    // TODO: pub keys: KeyList, (Not implemented in key.rs yet)

    /// Memo associated with the file.
    pub file_memo: String,

    /// The ledger ID the response was returned from
    pub ledger_id: LedgerId,
}

impl FromProtobuf<services::response::Response> for FileInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, FileGetInfo, services::response::Response);
        let info = pb_getf!(response, file_info)?;
        let file_id = pb_getf!(info, file_id)?;
        let ledger_id = LedgerId::from_bytes(info.ledger_id);

        // TODO: KeyList
        // let keys = info
        //     .keys
        //     .unwrap_or_default()
        //     .keys
        //     .into_iter()
        //     .map(Key::from_protobuf)
        //     .collect::<crate::Result<Vec<_>>>()?;

        Ok(Self {
            file_id: FileId::from_protobuf(file_id)?,
            size: info.size as u64,
            expiration_time: info.expiration_time.map(Into::into),
            is_deleted: info.deleted,
            file_memo: info.memo,
            ledger_id,
        })
    }
}
