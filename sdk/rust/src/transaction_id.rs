use hedera_proto::services;
use rand::{thread_rng, Rng};
use time::{Duration, OffsetDateTime};

use crate::{AccountId, ToProtobuf};

/// The client-generated ID for a transaction.
///
/// This is used for retrieving receipts and records for a transaction, for appending to a file
/// right after creating it, for instantiating a smart contract with bytecode in a file just created,
/// and internally by the network for detecting when duplicate transactions are submitted.
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransactionId {
    /// The account that pays for this transaction.
    pub account_id: AccountId,

    /// The time from when this transaction is valid.
    ///
    /// When a transaction is submitted there is additionally a validDuration (defaults to 120s)
    /// and together they define a time window that a transaction may be processed in.
    ///
    pub valid_start: OffsetDateTime,

    pub nonce: u32,
    pub scheduled: bool,
}

impl TransactionId {
    /// Generates a new transaction ID for the given account ID.
    pub fn generate(account_id: AccountId) -> Self {
        let valid_start = OffsetDateTime::now_utc()
            - Duration::nanoseconds(thread_rng().gen_range(5000000000, 8000000000));

        Self { account_id, valid_start, scheduled: false, nonce: 0 }
    }
}

impl ToProtobuf for TransactionId {
    type Protobuf = services::TransactionId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TransactionId {
            account_id: Some(self.account_id.to_protobuf()),
            scheduled: self.scheduled,
            nonce: self.nonce as i32,
            transaction_valid_start: Some(self.valid_start.into()),
        }
    }
}
