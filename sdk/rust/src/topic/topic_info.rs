use hedera_proto::services;
use serde_with::base64::Base64;
use serde_with::{
    serde_as,
    TimestampNanoSeconds,
};
use time::{
    Duration,
    OffsetDateTime,
};

use crate::{
    AccountId,
    FromProtobuf,
    Key,
    TopicId,
};

// TODO: ledgerId
/// Response from [`TopicInfoQuery`][crate::TopicInfoQuery].
#[serde_as]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicInfo {
    /// The ID of the topic for which information is requested.
    pub topic_id: TopicId,

    /// Short publicly visible memo about the topic. No guarantee of uniqueness
    pub topic_memo: String,

    /// SHA-384 running hash of (previousRunningHash, topicId, consensusTimestamp, sequenceNumber, message).
    #[serde_as(as = "Base64")]
    pub running_hash: Vec<u8>,

    /// Sequence number (starting at 1 for the first submitMessage) of messages on the topic.
    pub sequence_number: u64,

    /// Effective consensus timestamp at (and after) which submitMessage calls will no longer succeed on the topic.
    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    pub expiration_time: Option<OffsetDateTime>,

    /// Access control for update/delete of the topic.
    pub admin_key: Option<Key>,

    /// Access control for submit message.
    pub submit_key: Option<Key>,

    /// An account which will be automatically charged to renew the topic's expiration, at
    /// `auto_renew_period` interval.
    pub auto_renew_account_id: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the topic's expiry
    pub auto_renew_period: Option<Duration>,
}

impl FromProtobuf for TopicInfo {
    type Protobuf = services::response::Response;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
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
        })
    }
}
