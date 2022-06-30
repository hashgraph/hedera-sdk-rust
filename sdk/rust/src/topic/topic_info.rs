use hedera_proto::services;
use hedera_proto::consensus_topic_info;
use time::{Duration, OffsetDateTime};

use crate::{TopicId, FromProtobuf, Key, AccountId};

/// Response from [`TopicInfoQuery`][crate::TopicInfoQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicInfo {

    pub memo: String,

    pub running_hash: Vec<u8>,

    pub sequence_number: u64,

    pub expiration_time: Option<time::OffsetDateTime>,

    pub admin_key: Option<Key>,

    pub submit_key: Option<Key>,

    pub auto_renew_period: Option<time::Duration>,

    pub auto_renew_account: Option<AccountId>,

    pub ledger_id: Vec<u8>


}

impl FromProtobuf for TopicInfo {
    type Protobuf = services::response::Response;

    #[allow(deprecated)]
    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
        where
            Self: Sized,
    {
        let response = pb_getv!(pb, CryptoGetInfo, services::response::Response);
        let info = pb_getf!(response, topic_info)?;
        let key = pb_getf!(info, key)?;
        let topic_id = pb_getf!(info, topic_id)?;

        let proxy_topic_id = info
            .proxy_topic_id
            .map(TopicId::from_protobuf)
            .transpose()?
            .filter(|id| id.num > 0);

        Ok(Self {
            topic_id: TopicId::from_protobuf(topic_id)?,
            contract_topic_id: info.contract_topic_id,
            deleted: info.deleted,
            #[allow(deprecated)]
            proxy_topic_id,
            proxy_received: info.proxy_received as u64,
            // FIXME: key
            key: Key::from_protobuf(key)?,
            balance: info.balance as u64,
            send_record_threshold: info.generate_send_record_threshold,
            #[allow(deprecated)]
            receive_record_threshold: info.generate_receive_record_threshold,
            receiver_signature_required: info.receiver_sig_required,
            // FIXME: expires_at
            expires_at: None,
            // FIXME: auto_renew_period
            auto_renew_period: None,
            memo: info.memo,
            owned_nfts: info.owned_nfts as u64,
            max_automatic_token_associations: info.max_automatic_token_associations as u32,
            // FIXME: alias
            alias: None,
            ethereum_nonce: info.ethereum_nonce as u64,
        })
    }
}
