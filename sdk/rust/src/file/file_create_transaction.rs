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
#[serde(rename_all = "camelCase")]
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
