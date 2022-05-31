use hedera_proto::mirror;
use serde::Serialize;
use serde_with::hex::Hex;
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::{FromProtobuf, TransactionId};

#[serde_as]
#[derive(Serialize, Debug)]
pub struct TopicMessage {
    /// The consensus timestamp of the message.
    pub consensus_at: OffsetDateTime,

    /// The content of the message.
    #[serde_as(as = "Hex")]
    pub contents: Vec<u8>,

    /// The new running hash of the topic that received the message.
    pub running_hash: Vec<u8>,

    /// Version of the SHA-384 digest used to update the running hash.
    pub running_hash_version: u64,

    /// The sequence number of the message relative to all other messages
    /// for the same topic.
    pub sequence_number: u64,

    /// The [`TransactionId`] of the first chunk, gets copied to every subsequent chunk in
    /// a fragmented message.
    pub initial_transaction_id: Option<TransactionId>,

    /// The sequence number (from 1 to total) of the current chunk in the message.
    pub chunk_number: u32,

    /// The total number of chunks in the message.
    pub chunk_total: u32,
}

impl FromProtobuf for TopicMessage {
    type Protobuf = mirror::ConsensusTopicResponse;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let consensus_at = pb_getf!(pb, consensus_timestamp)?.into();
        let sequence_number = pb.sequence_number;
        let running_hash = pb.running_hash;
        let running_hash_version = pb.running_hash_version;
        let contents = pb.message;

        let (initial_transaction_id, chunk_number, chunk_total) = if let Some(chunk_info) =
            pb.chunk_info
        {
            (chunk_info.initial_transaction_id, chunk_info.number as u32, chunk_info.total as u32)
        } else {
            (None, 1, 1)
        };

        let initial_transaction_id =
            initial_transaction_id.map(TransactionId::from_protobuf).transpose()?;

        Ok(Self {
            consensus_at,
            contents,
            running_hash,
            running_hash_version,
            sequence_number,
            initial_transaction_id,
            chunk_number,
            chunk_total,
        })
    }
}
