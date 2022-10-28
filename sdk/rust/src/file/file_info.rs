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
use prost::Message;
use time::OffsetDateTime;

use crate::protobuf::ToProtobuf;
use crate::{
    FileId,
    FromProtobuf,
    LedgerId,
};

/// Response from [`FileInfoQuery`][crate::FileInfoQuery].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
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

impl FileInfo {
    /// Create a new `FileInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::file_get_info_response::FileInfo>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        services::file_get_info_response::FileInfo {
            file_id: Some(self.file_id.to_protobuf()),
            size: self.size as i64,
            expiration_time: self.expiration_time.as_ref().map(ToProtobuf::to_protobuf),
            deleted: self.is_deleted,
            memo: self.file_memo.clone(),
            ledger_id: self.ledger_id.to_bytes(),

            // unimplemented fields
            keys: None,
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::response::Response> for FileInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, FileGetInfo, services::response::Response);
        let info = pb_getf!(response, file_info)?;
        Self::from_protobuf(info)
    }
}

impl FromProtobuf<services::file_get_info_response::FileInfo> for FileInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::file_get_info_response::FileInfo) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let file_id = pb_getf!(pb, file_id)?;
        let ledger_id = LedgerId::from_bytes(pb.ledger_id);

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
            size: pb.size as u64,
            expiration_time: pb.expiration_time.map(Into::into),
            is_deleted: pb.deleted,
            file_memo: pb.memo,
            ledger_id,
        })
    }
}
