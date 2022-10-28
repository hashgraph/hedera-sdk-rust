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
use time::{
    Duration,
    OffsetDateTime,
};

use crate::{
    AccountId,
    FromProtobuf,
    Key,
    LedgerId,
    TopicId,
};

/// Response from [`TopicInfoQuery`][crate::TopicInfoQuery].

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TopicInfo {
    /// The ID of the topic for which information is requested.
    pub topic_id: TopicId,

    /// Short publicly visible memo about the topic. No guarantee of uniqueness
    pub topic_memo: String,

    /// SHA-384 running hash of (previousRunningHash, topicId, consensusTimestamp, sequenceNumber, message).
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    pub running_hash: Vec<u8>,

    /// Sequence number (starting at 1 for the first submitMessage) of messages on the topic.
    pub sequence_number: u64,

    /// Effective consensus timestamp at (and after) which submitMessage calls will no longer succeed on the topic.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
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

impl FromProtobuf<services::response::Response> for TopicInfo {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, ConsensusGetTopicInfo, services::response::Response);
        let topic_id = pb_getf!(response, topic_id)?;
        let info = pb_getf!(response, topic_info)?;
        let admin_key = info.admin_key.map(Key::from_protobuf).transpose()?;
        let submit_key = info.submit_key.map(Key::from_protobuf).transpose()?;
        let expiration_time = info.expiration_time.map(Into::into);
        let auto_renew_period = info.auto_renew_period.map(Into::into);
        let auto_renew_account_id =
            info.auto_renew_account.map(AccountId::from_protobuf).transpose()?;
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
