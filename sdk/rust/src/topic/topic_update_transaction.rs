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
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    Key,
    LedgerId,
    TopicId,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

/// Change properties for the given topic.
///
/// Any null field is ignored (left unchanged).
///
pub type TopicUpdateTransaction = Transaction<TopicUpdateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct TopicUpdateTransactionData {
    /// The topic ID which is being updated in this transaction.
    topic_id: Option<TopicId>,

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,

    /// Short publicly visible memo about the topic. No guarantee of uniqueness.
    topic_memo: Option<String>,

    /// Access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    admin_key: Option<Key>,

    /// Access control for `TopicMessageSubmitTransaction`.
    submit_key: Option<Key>,

    /// The initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time, if
    /// the `auto_renew_account_id` is configured.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    auto_renew_period: Option<Duration>,

    /// Optional account to be used at the topic's expiration time to extend the life of the topic.
    auto_renew_account_id: Option<AccountId>,
}

impl TopicUpdateTransaction {
    /// Returns the topic ID which is being updated.
    #[must_use]
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.data().topic_id
    }

    /// Sets the topic ID which is being updated.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data_mut().topic_id = Some(id.into());
        self
    }

    /// Returns the new expiration time to extend to (ignored if equal to or before the current one).
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    pub fn expiration_time(&mut self, at: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(at);
        self
    }

    /// Returns the new topic memo for the topic.
    #[must_use]
    pub fn get_topic_memo(&self) -> Option<&str> {
        self.data().topic_memo.as_deref()
    }

    /// Sets the short publicly visible memo about the topic.
    ///
    /// No guarantee of uniqueness.
    pub fn topic_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().topic_memo = Some(memo.into());
        self
    }

    /// Returns the access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(key.into());
        self
    }

    /// Returns the access control for `TopicMessageSubmitTransaction`.
    #[must_use]
    pub fn get_submit_key(&self) -> Option<&Key> {
        self.data().submit_key.as_ref()
    }

    /// Sets the access control for `TopicMessageSubmitTransaction`.
    pub fn submit_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().submit_key = Some(key.into());
        self
    }

    /// Returns the initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(period);
        self
    }

    /// Returns the account to be used at the topic's expiration time to extend the life of the topic.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the account to be used at the topic's expiration time to extend the life of the topic.
    pub fn auto_renew_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(id);
        self
    }
}

#[async_trait]
impl TransactionExecute for TopicUpdateTransactionData {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.topic_id.validate_checksums_for_ledger_id(ledger_id)?;
        self.auto_renew_account_id.validate_checksums_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        ConsensusServiceClient::new(channel).update_topic(request).await
    }
}

impl ToTransactionDataProtobuf for TopicUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let topic_id = self.topic_id.to_protobuf();
        let expiration_time = self.expiration_time.map(Into::into);
        let admin_key = self.admin_key.to_protobuf();
        let submit_key = self.submit_key.to_protobuf();
        let auto_renew_period = self.auto_renew_period.map(Into::into);
        let auto_renew_account_id = self.auto_renew_account_id.to_protobuf();

        services::transaction_body::Data::ConsensusUpdateTopic(
            services::ConsensusUpdateTopicTransactionBody {
                auto_renew_account: auto_renew_account_id,
                memo: self.topic_memo.clone(),
                expiration_time,
                topic_id,
                admin_key,
                submit_key,
                auto_renew_period,
            },
        )
    }
}

impl From<TopicUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: TopicUpdateTransactionData) -> Self {
        Self::TopicUpdate(transaction)
    }
}

impl FromProtobuf<services::ConsensusUpdateTopicTransactionBody> for TopicUpdateTransactionData {
    fn from_protobuf(pb: services::ConsensusUpdateTopicTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            topic_id: Option::from_protobuf(pb.topic_id)?,
            expiration_time: pb.expiration_time.map(Into::into),
            topic_memo: pb.memo,
            admin_key: Option::from_protobuf(pb.admin_key)?,
            submit_key: Option::from_protobuf(pb.submit_key)?,
            auto_renew_period: pb.auto_renew_period.map(Into::into),
            auto_renew_account_id: Option::from_protobuf(pb.auto_renew_account)?,
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use std::str::FromStr;

        use assert_matches::assert_matches;
        use time::{
            Duration,
            OffsetDateTime,
        };

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            AccountId,
            Key,
            PublicKey,
            TopicId,
            TopicUpdateTransaction,
        };

        // language=JSON
        const TOPIC_UPDATE_TRANSACTION_JSON: &str = r#"{
  "$type": "topicUpdate",
  "topicId": "0.0.1001",
  "expirationTime": 1656352251277559886,
  "topicMemo": "A topic memo",
  "adminKey": {
    "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
  },
  "submitKey": {
    "single": "302a300506032b6570032100b5b4d9351ebdf266ef3989aed4fd8f0cfcf24b75ba3d0df19cd3946771b40500"
  },
  "autoRenewPeriod": 7776000,
  "autoRenewAccountId": "0.0.1001"
}"#;

        const ADMIN_KEY: &str =
            "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";
        const SUBMIT_KEY: &str =
            "302a300506032b6570032100b5b4d9351ebdf266ef3989aed4fd8f0cfcf24b75ba3d0df19cd3946771b40500";

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TopicUpdateTransaction::new();

            transaction
                .topic_id(TopicId::from(1001))
                .expiration_time(OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?)
                .topic_memo("A topic memo")
                .admin_key(PublicKey::from_str(ADMIN_KEY)?)
                .submit_key(PublicKey::from_str(SUBMIT_KEY)?)
                .auto_renew_period(Duration::days(90))
                .auto_renew_account_id(AccountId::from(1001));

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOPIC_UPDATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOPIC_UPDATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.into_body().data, AnyTransactionData::TopicUpdate(transaction) => transaction);

            assert_eq!(data.topic_id.unwrap(), TopicId::from(1001));
            assert_eq!(
                data.expiration_time.unwrap(),
                OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?
            );
            assert_eq!(data.topic_memo.unwrap(), "A topic memo");
            assert_eq!(data.auto_renew_period.unwrap(), Duration::days(90));

            let admin_key =
                assert_matches!(data.admin_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(admin_key, PublicKey::from_str(ADMIN_KEY)?);

            let submit_key =
                assert_matches!(data.submit_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(submit_key, PublicKey::from_str(SUBMIT_KEY)?);

            assert_eq!(data.auto_renew_account_id, Some(AccountId::from(1001)));

            Ok(())
        }
    }
}
