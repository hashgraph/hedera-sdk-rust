use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use itertools::Itertools;
use serde_with::base64::Base64;
use serde_with::{serde_as, skip_serializing_none, TimestampNanoSeconds};
use time::{Duration, OffsetDateTime};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, Key, Transaction, TransactionId};

/// Create a new file, containing the given contents.
pub type FileCreateTransaction = Transaction<FileCreateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct FileCreateTransactionData {
    /// The memo associated with the file.
    #[serde(skip_serializing_if = "String::is_empty")]
    file_memo: String,

    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    keys: Option<Vec<Key>>,

    /// The bytes that are to be the contents of the file.
    #[serde_as(as = "Option<Base64>")]
    contents: Option<Vec<u8>>,

    /// The time at which this file should expire.
    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    expires_at: Option<OffsetDateTime>,
}

impl Default for FileCreateTransactionData {
    fn default() -> Self {
        Self {
            file_memo: String::new(),
            keys: None,
            contents: None,
            expires_at: Some(OffsetDateTime::now_utc() + Duration::days(90)),
        }
    }
}

impl FileCreateTransaction {
    /// Sets the memo associated with the file.
    pub fn file_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.file_memo = memo.into();
        self
    }

    /// Sets the bytes that are to be the contents of the file.
    pub fn contents(&mut self, contents: impl Into<Vec<u8>>) -> &mut Self {
        self.body.data.contents = Some(contents.into());
        self
    }

    /// Sets the keys for this file.
    ///
    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    ///
    pub fn keys<K: Into<Key>>(&mut self, keys: impl IntoIterator<Item = K>) -> &mut Self {
        self.body.data.keys = Some(keys.into_iter().map_into().collect());
        self
    }

    /// Sets the time at which this file should expire.
    pub fn expires_at(&mut self, at: OffsetDateTime) -> &mut Self {
        self.body.data.expires_at = Some(at);
        self
    }
}

#[async_trait]
impl TransactionExecute for FileCreateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        FileServiceClient::new(channel).create_file(request).await
    }
}

impl ToTransactionDataProtobuf for FileCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let expiration_time = self.expires_at.as_ref().map(OffsetDateTime::to_protobuf);

        let keys =
            self.keys.as_deref().unwrap_or_default().iter().map(Key::to_protobuf).collect_vec();

        let keys = services::KeyList { keys };

        services::transaction_body::Data::FileCreate(services::FileCreateTransactionBody {
            expiration_time,
            keys: Some(keys),
            contents: self.contents.clone().unwrap_or_default(),
            shard_id: None,
            realm_id: None,
            new_realm_admin_key: None,
            memo: self.file_memo.clone(),
        })
    }
}

impl From<FileCreateTransactionData> for AnyTransactionData {
    fn from(transaction: FileCreateTransactionData) -> Self {
        Self::FileCreate(transaction)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use assert_matches::assert_matches;
    use time::{Duration, OffsetDateTime};
    use crate::{FileCreateTransaction, Key, PublicKey};
    use crate::transaction::{AnyTransaction, AnyTransactionData};

    // language=JSON
    const FILE_CREATE_EMPTY: &str = r#"{
  "$type": "fileCreate"
}"#;

    // language=JSON
    const FILE_CREATE_TRANSACTION_JSON: &str = r#"{
  "$type": "fileCreate",
  "fileMemo": "File memo",
  "keys": [
    {
      "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
    }
  ],
  "contents": "SGVsbG8sIHdvcmxkIQ==",
  "expiresAt": 1656352251277559886
}"#;

    const SIGN_KEY: &str = "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = FileCreateTransaction::new();

        transaction
            .file_memo("File memo")
            .keys([PublicKey::from_str(SIGN_KEY)?])
            .contents("Hello, world!")
            .expires_at(OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?);

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, FILE_CREATE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(FILE_CREATE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::FileCreate(transaction) => transaction);

        assert_eq!(data.file_memo, "File memo");
        assert_eq!(data.expires_at.unwrap(), OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?);

        let sign_key = assert_matches!(data.keys.unwrap().remove(0), Key::Single(public_key) => public_key);
        assert_eq!(sign_key, PublicKey::from_str(SIGN_KEY)?);

        let bytes: Vec<u8> = "Hello, world!".into();
        assert_eq!(data.contents.unwrap(), bytes);

        Ok(())
    }

    #[test]
    fn it_should_deserialize_empty() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(FILE_CREATE_EMPTY)?;

        assert_matches!(transaction.body.data, AnyTransactionData::FileCreate(transaction) => transaction);

        Ok(())
    }
}
