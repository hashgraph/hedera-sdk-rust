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

/// Delete a topic.
///
/// No more transactions or queries on the topic will succeed.
///
/// If an `admin_key` is set, this transaction must be signed by that key.
/// If there is no `admin_key`, this transaction will fail `UNAUTHORIZED`.
///
pub type TopicDeleteTransaction = Transaction<TopicDeleteTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct TopicDeleteTransactionData {
    /// The topic ID which is being deleted in this transaction.
    topic_id: Option<TopicId>,
}

impl TopicDeleteTransaction {
    /// Returns the ID of the topic which is being deleted in this transaction.
    #[must_use]
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.data().topic_id
    }

    /// Sets the topic ID which is being deleted in this transaction.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data_mut().topic_id = Some(id.into());
        self
    }
}

impl TransactionExecute for TopicDeleteTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ConsensusServiceClient::new(channel).delete_topic(request).await })
    }
}

impl ValidateChecksums for TopicDeleteTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.topic_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TopicDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let topic_id = self.topic_id.to_protobuf();

        services::transaction_body::Data::ConsensusDeleteTopic(
            services::ConsensusDeleteTopicTransactionBody { topic_id },
        )
    }
}

impl From<TopicDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: TopicDeleteTransactionData) -> Self {
        Self::TopicDelete(transaction)
    }
}

impl FromProtobuf<services::ConsensusDeleteTopicTransactionBody> for TopicDeleteTransactionData {
    fn from_protobuf(pb: services::ConsensusDeleteTopicTransactionBody) -> crate::Result<Self> {
        Ok(Self { topic_id: Option::from_protobuf(pb.topic_id)? })
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
            TopicDeleteTransaction,
            TopicId,
        };

        // language=JSON
        const TOPIC_DELETE_TRANSACTION_JSON: &str = r#"{
  "$type": "topicDelete",
  "topicId": "0.0.1001"
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TopicDeleteTransaction::new();

            transaction.topic_id(TopicId::from(1001));

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOPIC_DELETE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOPIC_DELETE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TopicDelete(transaction) => transaction);

            assert_eq!(data.topic_id.unwrap(), TopicId::from(1001));

            Ok(())
        }
    }
}
