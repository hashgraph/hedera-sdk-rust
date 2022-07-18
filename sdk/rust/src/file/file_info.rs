use hedera_proto::services;
use time::OffsetDateTime;

use crate::{
    FileId,
    FromProtobuf,
};

/// Response from [`FileInfoQuery`][crate::FileInfoQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    /// The file ID of the file for which information is requested
    pub file_id: FileId,

    /// Number of bytes in contents
    pub size: u64,

    /// Current time which this account is set to expire
    pub expires_at: Option<OffsetDateTime>,

    /// True if deleted but not yet expired
    pub deleted: bool,

    /// One of these keys must sign in order to modify or delete the file
    // TODO: pub keys: KeyList, (Not implemented in key.rs yet)

    /// Memo associated with the file
    pub memo: String,
    // Ledger ID the response was returned from
    // TODO: pub ledger_id: LedgerId,
}

impl FromProtobuf<services::response::Response> for FileInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, FileGetInfo, services::response::Response);
        let info = pb_getf!(response, file_info)?;
        let keys = pb_getf!(info, keys)?;
        let file_id = pb_getf!(info, file_id)?;

        Ok(Self {
            file_id: FileId::from_protobuf(file_id)?,
            size: info.size as u64,
            // FIXME: expires_at
            expires_at: None,
            deleted: info.deleted,
            // TODO: KeyList
            // keys: KeyList::from_protobuf(keys)?,
            memo: info.memo,
            // FIXME: ledger_id (not present in account_info.rs)
            // TODO: ledger_id
        })
    }
}
