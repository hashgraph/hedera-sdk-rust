use hedera_proto::services;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

use crate::{FileId, FromProtobuf};

/// Response from [`FileContentsQuery`][crate::FileContentsQuery].
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContentsResponse {
    pub file_id: FileId,

    #[serde_as(as = "Base64")]
    pub contents: Vec<u8>,
}

impl FromProtobuf for FileContentsResponse {
    type Protobuf = services::response::Response;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
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
