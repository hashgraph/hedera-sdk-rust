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
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    Key,
    KeyList,
    LedgerId,
    Transaction,
    TransactionId,
};

/// Create a new file, containing the given contents.
pub type FileCreateTransaction = Transaction<FileCreateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct FileCreateTransactionData {
    /// The memo associated with the file.
    #[cfg_attr(feature = "ffi", serde(skip_serializing_if = "String::is_empty"))]
    file_memo: String,

    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    keys: Option<KeyList>,

    /// The bytes that are to be the contents of the file.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::base64::Base64>>")
    )]
    contents: Option<Vec<u8>>,

    auto_renew_period: Option<Duration>,

    auto_renew_account_id: Option<AccountId>,

    /// The time at which this file should expire.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,
}

impl Default for FileCreateTransactionData {
    fn default() -> Self {
        Self {
            file_memo: String::new(),
            keys: None,
            contents: None,
            auto_renew_period: None,
            auto_renew_account_id: None,
            expiration_time: Some(OffsetDateTime::now_utc() + Duration::days(90)),
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
        self.body.data.keys = Some(keys.into_iter().map(Into::into).collect());
        self
    }

    /// Set the auto renew period for this file.
    pub fn auto_renew_period(&mut self, duration: Duration) -> &mut Self {
        self.body.data.auto_renew_period = Some(duration);
        self
    }

    /// Sets the account to be used at the files's expiration time to extend the
    /// life of the file.
    pub fn auto_renew_account_id(&mut self, id: AccountId) -> &mut Self {
        self.body.data.auto_renew_account_id = Some(id);
        self
    }

    /// Sets the time at which this file should expire.
    pub fn expiration_time(&mut self, at: OffsetDateTime) -> &mut Self {
        self.body.data.expiration_time = Some(at);
        self
    }
}

#[async_trait]
impl TransactionExecute for FileCreateTransactionData {
    fn validate_checksums_for_ledger_id(&self, _ledger_id: &LedgerId) -> Result<(), Error> {
        Ok(())
    }

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
        let expiration_time = self.expiration_time.to_protobuf();

        services::transaction_body::Data::FileCreate(services::FileCreateTransactionBody {
            auto_renew_period: self.auto_renew_period.to_protobuf(),
            auto_renew_account: self.auto_renew_account_id.to_protobuf(),
            expiration_time,
            keys: self.keys.to_protobuf(),
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
    #[cfg(feature = "ffi")]
    mod ffi {
        use std::str::FromStr;

        use assert_matches::assert_matches;
        use time::OffsetDateTime;

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            FileCreateTransaction,
            Key,
            PublicKey,
        };

        // language=JSON
        const FILE_CREATE_EMPTY: &str = r#"{
  "$type": "fileCreate"
}"#;

        // language=JSON
        const FILE_CREATE_TRANSACTION_JSON: &str = r#"{
  "$type": "fileCreate",
  "fileMemo": "File memo",
  "keys": {
    "keys": [
      {
        "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
      }
    ]
  },
  "contents": "SGVsbG8sIHdvcmxkIQ==",
  "expirationTime": 1656352251277559886
}"#;

        const SIGN_KEY: &str =
        "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = FileCreateTransaction::new();

            transaction
                .file_memo("File memo")
                .keys([PublicKey::from_str(SIGN_KEY)?])
                .contents("Hello, world!")
                .expiration_time(OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, FILE_CREATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(FILE_CREATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.body.data, AnyTransactionData::FileCreate(transaction) => transaction);

            assert_eq!(data.file_memo, "File memo");
            assert_eq!(
                data.expiration_time.unwrap(),
                OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?
            );

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
}
