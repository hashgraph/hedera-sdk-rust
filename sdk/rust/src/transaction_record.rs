use hedera_proto::services;
use serde_with::{
    serde_as,
    skip_serializing_none,
};
use time::OffsetDateTime;

use crate::{
    FromProtobuf,
    PublicKey,
    ScheduleId,
    TokenAssociation,
    TransactionHash,
    TransactionId,
    TransactionReceipt,
};

/// The complete record for a transaction on Hedera that has reached consensus.
/// Response from [`TransactionRecordQuery`][crate::TransactionRecordQuery].
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRecord {
    /// The status (reach consensus, or failed, or is unknown) and the ID of
    /// any new account/file/instance created.
    pub receipt: TransactionReceipt,

    /// The hash of the Transaction that executed (not the hash of any Transaction that failed for
    /// having a duplicate TransactionID).
    pub transaction_hash: Vec<u8>,

    /// The consensus timestamp.
    pub consensus_timestamp: OffsetDateTime,

    /// The ID of the transaction this record represents.
    pub transaction_id: TransactionId,

    /// The memo that was submitted as part of the transaction.
    pub transaction_memo: String,

    /// The actual transaction fee charged.
    pub transaction_fee: u64,

    /// Reference to the scheduled transaction ID that this transaction record represents.
    pub schedule_ref: Option<ScheduleId>,

    // /// All custom fees that were assessed during a CryptoTransfer, and must be paid if the
    // /// transaction status resolved to SUCCESS.
    // TODO: pub assessed_custom_fees: Vec<AssessedCustomFee>,
    /// All token associations implicitly created while handling this transaction
    pub automatic_token_associations: Vec<TokenAssociation>,

    /// In the record of an internal transaction, the consensus timestamp of the user
    /// transaction that spawned it.
    pub parent_consensus_timestamp: Option<OffsetDateTime>,

    /// In the record of an internal CryptoCreate transaction triggered by a user
    /// transaction with a (previously unused) alias, the new account's alias.
    pub alias_key: Option<PublicKey>,

    /// The records of processing all child transaction spawned by the transaction with the given
    /// top-level id, in consensus order. Always empty if the top-level status is UNKNOWN.
    pub children: Vec<Self>,

    /// The records of processing all consensus transaction with the same id as the distinguished
    /// record above, in chronological order.
    pub duplicates: Vec<Self>,

    /// The keccak256 hash of the ethereumData. This field will only be populated for
    /// `EthereumTransaction`.
    pub ethereum_hash: Vec<u8>,
    // /// In the record of a PRNG transaction with no output range, a pseudorandom 384-bit string.
    // TODO: pub prng_bytes: Vec<u8>,
    //
    // /// In the record of a PRNG transaction with an output range, the output of a PRNG
    // /// whose input was a 384-bit string.
    // TODO: pub prng_number: i32,
}
// TODO: contractFunctionResult
// TODO: transfers
// TODO: tokenTransfers
// TODO: tokenTransferList
// TODO: tokenNftTransfers
// TODO: paid_staking_rewards

impl TransactionRecord {
    fn from_protobuf(
        record: services::TransactionRecord,
        duplicates: Vec<Self>,
        children: Vec<Self>,
    ) -> crate::Result<Self> {
        let receipt = pb_getf!(record, receipt)?;
        let receipt = <TransactionReceipt as FromProtobuf<_>>::from_protobuf(receipt)?;

        let consensus_timestamp = pb_getf!(record, consensus_timestamp)?;
        let transaction_id = pb_getf!(record, transaction_id)?;
        let schedule_ref = record.schedule_ref.map(ScheduleId::from_protobuf).transpose()?;
        let parent_consensus_timestamp = record.parent_consensus_timestamp.map(Into::into);

        let alias_key = if !record.alias.is_empty() {
            Some(PublicKey::from_bytes(&record.alias)?)
        } else {
            None
        };

        let automatic_token_associations = record
            .automatic_token_associations
            .into_iter()
            .map(TokenAssociation::from_protobuf)
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(Self {
            receipt,
            transaction_hash: record.transaction_hash,
            consensus_timestamp: consensus_timestamp.into(),
            transaction_id: TransactionId::from_protobuf(transaction_id)?,
            transaction_memo: record.memo,
            transaction_fee: record.transaction_fee,
            schedule_ref,
            automatic_token_associations,
            parent_consensus_timestamp,
            duplicates,
            ethereum_hash: record.ethereum_hash,
            children,
            alias_key,
        })
    }
}

impl FromProtobuf<services::response::Response> for TransactionRecord {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, TransactionGetRecord, services::response::Response);

        let record = pb_getf!(pb, transaction_record)?;

        let duplicates = pb
            .duplicate_transaction_records
            .into_iter()
            .map(<TransactionRecord as FromProtobuf<_>>::from_protobuf)
            .collect::<crate::Result<_>>()?;

        let children = pb
            .child_transaction_records
            .into_iter()
            .map(<TransactionRecord as FromProtobuf<_>>::from_protobuf)
            .collect::<crate::Result<_>>()?;

        Self::from_protobuf(record, duplicates, children)
    }
}

impl FromProtobuf<services::TransactionRecord> for TransactionRecord {
    fn from_protobuf(receipt: services::TransactionRecord) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Self::from_protobuf(receipt, Vec::new(), Vec::new())
    }
}
