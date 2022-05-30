use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, TopicId, Transaction, TransactionId};

pub type TopicMessageSubmitTransaction = Transaction<TopicMessageSubmitTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicMessageSubmitTransactionData {
    /// The topic ID to submit this message to.
    topic_id: Option<TopicId>,

    /// Message to be submitted.
    /// Max size of the Transaction (including signatures) is 6KiB.
    message: Option<Vec<u8>>,

    /// The `TransactionId` of the first chunk.
    ///
    /// Should get copied to every subsequent chunk in a fragmented message.
    initial_transaction_id: Option<TransactionId>,

    /// The total number of chunks in the message.
    /// Defaults to 1.
    chunk_total: i32,

    /// The sequence number (from 1 to total) of the current chunk in the message.
    /// Defaults to 1.
    chunk_number: i32,
}

impl TopicMessageSubmitTransaction {
    /// Set the account ID which is being deleted.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.body.data.topic_id = Some(id.into());
        self
    }

    /// Sets the message to be submitted.
    pub fn message(&mut self, bytes: impl Into<Vec<u8>>) -> &mut Self {
        self.body.data.message = Some(bytes.into());
        self
    }

    /// Sets the `TransactionId` of the first chunk.
    pub fn initial_transaction_id(&mut self, id: impl Into<TransactionId>) -> &mut Self {
        self.body.data.initial_transaction_id = Some(id.into());
        self
    }

    /// Sets the total number of chunks in the message.
    pub fn chunk_total(&mut self, total: u32) -> &mut Self {
        self.body.data.chunk_total = total as i32;
        self
    }

    /// Sets the sequence number (from 1 to total) of the current chunk in the message.
    pub fn chunk_number(&mut self, number: u32) -> &mut Self {
        self.body.data.chunk_number = number as i32;
        self
    }
}

#[async_trait]
impl TransactionExecute for TopicMessageSubmitTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        ConsensusServiceClient::new(channel).submit_message(request).await
    }
}

impl ToTransactionDataProtobuf for TopicMessageSubmitTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let topic_id = self.topic_id.as_ref().map(TopicId::to_protobuf);

        let chunk_info = if let Some(initial_id) = &self.initial_transaction_id {
            let initial_id = initial_id.to_protobuf();

            Some(services::ConsensusMessageChunkInfo {
                initial_transaction_id: Some(initial_id),
                number: self.chunk_number,
                total: self.chunk_total,
            })
        } else {
            None
        };

        services::transaction_body::Data::ConsensusSubmitMessage(
            services::ConsensusSubmitMessageTransactionBody {
                topic_id,
                message: self.message.clone().unwrap_or_default(),
                chunk_info,
            },
        )
    }
}

impl From<TopicMessageSubmitTransactionData> for AnyTransactionData {
    fn from(transaction: TopicMessageSubmitTransactionData) -> Self {
        Self::TopicMessageSubmit(transaction)
    }
}
