use hedera_proto::services;
use time::OffsetDateTime;

use crate::{
    FileId,
    FromProtobuf,
};

// TODO: pub ledger_id: LedgerId,
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
            expires_at: info.expiration_time.map(Into::into),
            deleted: info.deleted,
            memo: info.memo,
        })
    }
}
