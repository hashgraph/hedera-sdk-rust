use hedera_proto::services;
use time::{Duration, OffsetDateTime};

use crate::{TopicId, FromProtobuf, Key};

/// Response from [`TopicInfoQuery`][crate::TopicInfoQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicInfo {
    /// The topic that is being referenced.
    pub topic_id: TopicId,

    /// The Contract Topic ID comprising of both the contract instance and the cryptocurrency
    /// topic owned by the contract instance, in the format used by Solidity.
    pub contract_topic_id: String,

    /// If true, then this topic has been deleted, it will disappear when it expires, and all
    /// transactions for it will fail except the transaction to extend its expiration date.
    pub deleted: bool,

    /// The topic ID of the topic to which this is proxy staked.
    #[deprecated]
    pub proxy_topic_id: Option<TopicId>,

    /// The total number of hbars proxy staked to this topic.
    pub proxy_received: u64, // TODO: Hbar

    /// The key for the topic, which must sign in order to transfer_transaction out, or to modify the
    /// topic in any way other than extending its expiration date.
    // TODO: serde
    #[serde(skip)]
    pub key: Key,

    /// Current balance of the referenced topic.
    // TODO: use Hbar type
    pub balance: u64,

    /// The threshold amount, in hbars, at which a record is created of any
    /// transaction that decreases the balance of this topic by more than the threshold.
    // TODO: use Hbar type
    #[deprecated]
    pub send_record_threshold: u64,

    /// The threshold amount, in hbars, at which a record is created of any
    /// transaction that increases the balance of this topic by more than the threshold.
    // TODO: use Hbar type
    #[deprecated]
    pub receive_record_threshold: u64,

    /// If true, no transaction can transfer_transaction to this topic unless signed by
    /// this topic's key.
    pub receiver_signature_required: bool,

    /// The TimeStamp time at which this topic is set to expire.
    pub expires_at: Option<OffsetDateTime>,

    /// The duration for expiration time will extend every this many seconds.
    pub auto_renew_period: Option<Duration>,
    //
    // All tokens related to this topic.
    // TODO: pub token_relationships: HashMap<TokenId, TokenRelationship>,
    //
    /// The memo associated with the topic.
    pub memo: String,

    /// The number of NFTs owned by this topic
    pub owned_nfts: u64,

    /// The maximum number of tokens that a Topic can be implicitly associated with.
    pub max_automatic_token_associations: u32,

    /// The alias of this topic.
    pub alias: Option<Vec<u8>>, // TODO: Option<PublicKey>,
    //
    // The ledger ID
    // TODO: pub ledger_id: LedgerId,
    /// The ethereum transaction nonce associated with this topic.
    pub ethereum_nonce: u64,
    //
    // TODO: pub staking: StakingInfo;
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
