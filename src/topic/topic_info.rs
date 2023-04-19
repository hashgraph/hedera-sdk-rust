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
use prost::Message;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::protobuf::ToProtobuf;
use crate::{
    AccountId,
    FromProtobuf,
    Key,
    LedgerId,
    TopicId,
};

/// Response from [`TopicInfoQuery`][crate::TopicInfoQuery].

#[derive(Debug, Clone)]
pub struct TopicInfo {
    /// The ID of the topic for which information is requested.
    pub topic_id: TopicId,

    /// Short publicly visible memo about the topic. No guarantee of uniqueness
    pub topic_memo: String,

    /// SHA-384 running hash of (previousRunningHash, topicId, consensusTimestamp, sequenceNumber, message).
    pub running_hash: Vec<u8>,

    /// Sequence number (starting at 1 for the first submitMessage) of messages on the topic.
    pub sequence_number: u64,

    /// Effective consensus timestamp at (and after) which submitMessage calls will no longer succeed on the topic.
    pub expiration_time: Option<OffsetDateTime>,

    /// Access control for update/delete of the topic.
    pub admin_key: Option<Key>,

    /// Access control for submit message.
    pub submit_key: Option<Key>,

    /// An account which will be automatically charged to renew the topic's expiration, at
    /// `auto_renew_period` interval.
    pub auto_renew_account_id: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the topic's expiry.
    pub auto_renew_period: Option<Duration>,

    /// The ledger ID the response was returned from
    pub ledger_id: LedgerId,
}

impl TopicInfo {
    /// Create a new `TopicInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::ConsensusGetTopicInfoResponse>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        services::ConsensusGetTopicInfoResponse {
            topic_id: Some(self.topic_id.to_protobuf()),
            topic_info: Some(services::ConsensusTopicInfo {
                memo: self.topic_memo.clone(),
                running_hash: self.running_hash.clone(),
                sequence_number: self.sequence_number,
                expiration_time: self.expiration_time.to_protobuf(),
                admin_key: self.admin_key.to_protobuf(),
                submit_key: self.submit_key.to_protobuf(),
                auto_renew_period: self.auto_renew_period.to_protobuf(),
                auto_renew_account: self.auto_renew_account_id.to_protobuf(),
                ledger_id: self.ledger_id.to_bytes(),
            }),
            header: None,
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::response::Response> for TopicInfo {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, ConsensusGetTopicInfo, services::response::Response);
        Self::from_protobuf(response)
    }
}

impl FromProtobuf<services::ConsensusGetTopicInfoResponse> for TopicInfo {
    fn from_protobuf(pb: services::ConsensusGetTopicInfoResponse) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let topic_id = pb_getf!(pb, topic_id)?;
        let info = pb_getf!(pb, topic_info)?;
        let admin_key = Option::from_protobuf(info.admin_key)?;
        let submit_key = Option::from_protobuf(info.submit_key)?;
        let expiration_time = info.expiration_time.map(Into::into);
        let auto_renew_period = info.auto_renew_period.map(Into::into);
        let auto_renew_account_id = Option::from_protobuf(info.auto_renew_account)?;
        let ledger_id = LedgerId::from_bytes(info.ledger_id);

        Ok(Self {
            topic_id: TopicId::from_protobuf(topic_id)?,
            admin_key,
            submit_key,
            auto_renew_period,
            auto_renew_account_id,
            running_hash: info.running_hash,
            sequence_number: info.sequence_number,
            expiration_time,
            topic_memo: info.memo,
            ledger_id,
        })
    }
}
