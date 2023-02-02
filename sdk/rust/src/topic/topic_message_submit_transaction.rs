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
    ChunkData,
    ChunkInfo,
    ChunkedTransactionData,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
    TransactionExecuteChunked,
};
use crate::{
    BoxGrpcFuture,
    Error,
    LedgerId,
    TopicId,
    Transaction,
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
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct TopicMessageSubmitTransactionData {
    /// The topic ID to submit this message to.
    topic_id: Option<TopicId>,

    #[cfg_attr(feature = "ffi", serde(flatten))]
    chunk_data: ChunkData,
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
}

impl TransactionData for TopicMessageSubmitTransactionData {
    fn maybe_chunk_data(&self) -> Option<&ChunkData> {
        Some(self.chunk_data())
    }

    fn wait_for_receipt(&self) -> bool {
        false
    }
}

impl ChunkedTransactionData for TopicMessageSubmitTransactionData {
    fn chunk_data(&self) -> &ChunkData {
        &self.chunk_data
    }

    fn chunk_data_mut(&mut self) -> &mut ChunkData {
        &mut self.chunk_data
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

impl TransactionExecuteChunked for TopicMessageSubmitTransactionData {}

impl ValidateChecksums for TopicMessageSubmitTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.topic_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TopicMessageSubmitTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let topic_id = self.topic_id.to_protobuf();

        services::transaction_body::Data::ConsensusSubmitMessage(
            services::ConsensusSubmitMessageTransactionBody {
                topic_id,
                message: self.chunk_data.message_chunk(chunk_info).to_vec(),
                chunk_info: (chunk_info.total > 1).then(|| services::ConsensusMessageChunkInfo {
                    initial_transaction_id: Some(chunk_info.initial_transaction_id.to_protobuf()),
                    number: chunk_info.current as i32,
                    total: chunk_info.total as i32,
                }),
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
    fn from_protobuf(_pb: services::ConsensusSubmitMessageTransactionBody) -> crate::Result<Self> {
        todo!("fix to/from bytes for chunked transactions");
        // let (initial_transaction_id, chunk_total, chunk_number) = match pb.chunk_info {
        //     Some(pb) => (
        //         Some(TransactionId::from_protobuf(pb_getf!(pb, initial_transaction_id)?)?),
        //         pb.total,
        //         pb.number,
        //     ),
        //     None => (None, 1, 1),
        // };

        // Ok(Self {
        //     topic_id: Option::from_protobuf(pb.topic_id)?,
        //     message: (!pb.message.is_empty()).then(|| pb.message),
        //     initial_transaction_id,
        //     chunk_total,
        //     chunk_number,
        // })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use assert_matches::assert_matches;

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            TopicId,
            TopicMessageSubmitTransaction,
        };

        // language=JSON
        const TOPIC_MESSAGE_SUBMIT_EMPTY: &str = r#"{
  "$type": "topicMessageSubmit"
}"#;

        // language=JSON
        const TOPIC_MESSAGE_SUBMIT_TRANSACTION_JSON: &str = r#"{
  "$type": "topicMessageSubmit",
  "topicId": "0.0.1001",
  "maxChunks": 1,
  "message": "TWVzc2FnZQ=="
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TopicMessageSubmitTransaction::new();

            transaction.topic_id(TopicId::from(1001)).message("Message").max_chunks(1);

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

            assert_eq!(data.chunk_data.message, b"Message".to_vec());
            assert_eq!(data.chunk_data.max_chunks, 1);

            Ok(())
        }

        #[test]
        fn it_should_deserialize_empty() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOPIC_MESSAGE_SUBMIT_EMPTY)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TopicMessageSubmit(transaction) => transaction);

            assert_eq!(data.chunk_data.chunk_size.get(), 1024);
            assert_eq!(data.chunk_data.max_chunks, 20);
            assert_eq!(data.chunk_data.message, Vec::<u8>::new());

            Ok(())
        }
    }
}
