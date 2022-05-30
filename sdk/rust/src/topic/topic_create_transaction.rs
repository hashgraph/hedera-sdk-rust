use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use serde_with::skip_serializing_none;
use time::Duration;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, Key, Transaction, TransactionId};

/// Create a topic to be used for consensus.
///
/// If an `auto_renew_account` is specified, that account must also sign this transaction.
///
/// If an `admin_key` is specified, the adminKey must sign the transaction.
///
/// On success, the resulting `TransactionReceipt` contains the newly created `TopicId`.
///
pub type TopicCreateTransaction = Transaction<TopicCreateTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicCreateTransactionData {
    /// Short publicly visible memo about the topic. No guarantee of uniqueness.
    #[serde(skip_serializing_if = "String::is_empty")]
    topic_memo: String,

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

impl Default for TopicCreateTransactionData {
    fn default() -> Self {
        Self {
            topic_memo: String::new(),
            admin_key: None,
            submit_key: None,
            auto_renew_period: Some(Duration::days(90)),
            auto_renew_account_id: None,
        }
    }
}

impl TopicCreateTransaction {
    /// Sets the short publicly visible memo about the topic.
    ///
    /// No guarantee of uniqueness.
    ///
    pub fn topic_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.topic_memo = memo.into();
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
impl TransactionExecute for TopicCreateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        ConsensusServiceClient::new(channel).create_topic(request).await
    }
}

impl ToTransactionDataProtobuf for TopicCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let admin_key = self.admin_key.as_ref().map(Key::to_protobuf);
        let submit_key = self.submit_key.as_ref().map(Key::to_protobuf);
        let auto_renew_period = self.auto_renew_period.as_ref().map(Duration::to_protobuf);
        let auto_renew_account_id = self.auto_renew_account_id.as_ref().map(AccountId::to_protobuf);

        services::transaction_body::Data::ConsensusCreateTopic(
            services::ConsensusCreateTopicTransactionBody {
                auto_renew_account: auto_renew_account_id,
                memo: self.topic_memo.clone(),
                admin_key,
                submit_key,
                auto_renew_period,
            },
        )
    }
}

impl From<TopicCreateTransactionData> for AnyTransactionData {
    fn from(transaction: TopicCreateTransactionData) -> Self {
        Self::TopicCreate(transaction)
    }
}
