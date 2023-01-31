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

use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
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
    BoxGrpcFuture,
    Error,
    LedgerId,
    TopicId,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

/// Submit a message for consensus.
///
/// Valid and authorized messages on valid topics will be ordered by the consensus service, gossipped to the
/// mirror net, and published (in order) to all subscribers (from the mirror net) on this topic.
///
/// The `submit_key` (if any) must sign this transaction.
///
/// On success, the resulting `TransactionReceipt` contains the topic's updated `topic_sequence_number` and
/// `topic_running_hash`.
///
pub type TopicMessageSubmitTransaction = Transaction<TopicMessageSubmitTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct TopicMessageSubmitTransactionData {
    /// The topic ID to submit this message to.
    topic_id: Option<TopicId>,

    /// Message to be submitted.
    /// Max size of the Transaction (including signatures) is 6KiB.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::base64::Base64>>")
    )]
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

impl Default for TopicMessageSubmitTransactionData {
    fn default() -> Self {
        Self {
            message: None,
            chunk_number: 1,
            chunk_total: 1,
            initial_transaction_id: None,
            topic_id: None,
        }
    }
}

impl TopicMessageSubmitTransaction {
    /// Returns the ID of the topic this message will be submitted to.
    #[must_use]
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.data().topic_id
    }

    /// Sets the topic ID to submit this message to.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data_mut().topic_id = Some(id.into());
        self
    }

    /// Returns the message to be submitted.
    #[must_use]
    pub fn get_message(&self) -> Option<&[u8]> {
        self.data().message.as_deref()
    }

    /// Sets the message to be submitted.
    pub fn message(&mut self, bytes: impl Into<Vec<u8>>) -> &mut Self {
        self.data_mut().message = Some(bytes.into());
        self
    }

    /// Returns the `TransactionId` of the first chunk.
    #[must_use]
    pub fn get_initial_transaction_id(&self) -> Option<TransactionId> {
        self.data().initial_transaction_id
    }

    /// Sets the `TransactionId` of the first chunk.
    pub fn initial_transaction_id(&mut self, id: impl Into<TransactionId>) -> &mut Self {
        self.data_mut().initial_transaction_id = Some(id.into());
        self
    }

    /// Returns the total number of chunks in the message.
    #[must_use]
    pub fn get_chunk_total(&self) -> u32 {
        self.data().chunk_total as u32
    }

    /// Sets the total number of chunks in the message.
    pub fn chunk_total(&mut self, total: u32) -> &mut Self {
        self.data_mut().chunk_total = total as i32;
        self
    }

    /// Returns the sequence number (from 1 to total) of the current chunk in the message.
    #[must_use]
    pub fn get_chunk_number(&self) -> u32 {
        self.data().chunk_number as u32
    }

    /// Sets the sequence number (from 1 to total) of the current chunk in the message.
    pub fn chunk_number(&mut self, number: u32) -> &mut Self {
        self.data_mut().chunk_number = number as i32;
        self
    }
}

impl TransactionExecute for TopicMessageSubmitTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ConsensusServiceClient::new(channel).submit_message(request).await })
    }
}

impl ValidateChecksums for TopicMessageSubmitTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.topic_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TopicMessageSubmitTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let topic_id = self.topic_id.to_protobuf();

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

impl FromProtobuf<services::ConsensusSubmitMessageTransactionBody>
    for TopicMessageSubmitTransactionData
{
    fn from_protobuf(pb: services::ConsensusSubmitMessageTransactionBody) -> crate::Result<Self> {
        let (initial_transaction_id, chunk_total, chunk_number) = match pb.chunk_info {
            Some(pb) => (
                Some(TransactionId::from_protobuf(pb_getf!(pb, initial_transaction_id)?)?),
                pb.total,
                pb.number,
            ),
            None => (None, 1, 1),
        };

        Ok(Self {
            topic_id: Option::from_protobuf(pb.topic_id)?,
            message: (!pb.message.is_empty()).then(|| pb.message),
            initial_transaction_id,
            chunk_total,
            chunk_number,
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use std::str::FromStr;

        use assert_matches::assert_matches;

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            TopicId,
            TopicMessageSubmitTransaction,
            TransactionId,
        };

        // language=JSON
        const TOPIC_MESSAGE_SUBMIT_EMPTY: &str = r#"{
  "$type": "topicMessageSubmit"
}"#;

        // language=JSON
        const TOPIC_MESSAGE_SUBMIT_TRANSACTION_JSON: &str = r#"{
  "$type": "topicMessageSubmit",
  "topicId": "0.0.1001",
  "message": "TWVzc2FnZQ==",
  "initialTransactionId": "0.0.1001@1656352251.277559886",
  "chunkTotal": 1,
  "chunkNumber": 1
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TopicMessageSubmitTransaction::new();

            transaction
                .topic_id(TopicId::from(1001))
                .message("Message")
                .initial_transaction_id(TransactionId::from_str("1001@1656352251.277559886")?)
                .chunk_total(1)
                .chunk_number(1);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOPIC_MESSAGE_SUBMIT_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction =
                serde_json::from_str(TOPIC_MESSAGE_SUBMIT_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.into_body().data, AnyTransactionData::TopicMessageSubmit(transaction) => transaction);

            assert_eq!(data.topic_id.unwrap(), TopicId::from(1001));
            assert_eq!(
                data.initial_transaction_id.unwrap(),
                TransactionId::from_str("1001@1656352251.277559886")?
            );
            assert_eq!(data.chunk_total, 1);
            assert_eq!(data.chunk_number, 1);

            let bytes: Vec<u8> = "Message".into();
            assert_eq!(data.message.unwrap(), bytes);

            Ok(())
        }

        #[test]
        fn it_should_deserialize_empty() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOPIC_MESSAGE_SUBMIT_EMPTY)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TopicMessageSubmit(transaction) => transaction);

            assert_eq!(data.chunk_number, 1);
            assert_eq!(data.chunk_total, 1);

            Ok(())
        }
    }
}
