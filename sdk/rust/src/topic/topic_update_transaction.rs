use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use serde_with::skip_serializing_none;
use time::{Duration, OffsetDateTime};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, Key, TopicId, Transaction, TransactionId};

/// Change properties for the given topic.
///
/// Any null field is ignored (left unchanged).
///
pub type TopicUpdateTransaction = Transaction<TopicUpdateTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicUpdateTransactionData {
    /// The topic ID which is being updated in this transaction.
    topic_id: Option<TopicId>,

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    expires_at: Option<OffsetDateTime>,

    /// Short publicly visible memo about the topic. No guarantee of uniqueness.
    topic_memo: Option<String>,

    /// Access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    admin_key: Option<Key>,

    /// Access control for `TopicMessageSubmitTransaction`.
    submit_key: Option<Key>,

    /// The initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time, if
    /// the `auto_renew_account_id` is configured.
    auto_renew_period: Option<Duration>,

    /// Optional account to be used at the topic's expiration time to extend the life of the topic.
    auto_renew_account_id: Option<AccountId>,
}

impl TopicUpdateTransaction {
    /// Set the account ID which is being updated.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.body.data.topic_id = Some(id.into());
        self
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    pub fn expires_at(&mut self, at: OffsetDateTime) -> &mut Self {
        self.body.data.expires_at = Some(at);
        self
    }

    /// Sets the short publicly visible memo about the topic.
    ///
    /// No guarantee of uniqueness.
    ///
    pub fn topic_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.topic_memo = Some(memo.into());
        self
    }

    /// Sets the access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.body.data.admin_key = Some(key.into());
        self
    }

    /// Sets the access control for `TopicMessageSubmitTransaction`.
    pub fn submit_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.body.data.submit_key = Some(key.into());
        self
    }

    /// Sets the initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.body.data.auto_renew_period = Some(period);
        self
    }

    /// Sets the account to be used at the topic's expiration time to extend the life of the topic.
    pub fn auto_renew_account_id(&mut self, id: impl Into<AccountId>) -> &mut Self {
        self.body.data.auto_renew_account_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TopicUpdateTransactionData {
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
        let topic_id = self.topic_id.as_ref().map(TopicId::to_protobuf);
        let expiration_time = self.expires_at.as_ref().map(OffsetDateTime::to_protobuf);
        let admin_key = self.admin_key.as_ref().map(Key::to_protobuf);
        let submit_key = self.submit_key.as_ref().map(Key::to_protobuf);
        let auto_renew_period = self.auto_renew_period.as_ref().map(Duration::to_protobuf);
        let auto_renew_account_id = self.auto_renew_account_id.as_ref().map(AccountId::to_protobuf);

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
