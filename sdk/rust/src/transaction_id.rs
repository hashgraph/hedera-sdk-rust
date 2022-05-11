use std::fmt::{self, Debug, Display, Formatter};

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
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransactionId {
    /// The account that pays for this transaction.
    pub account_id: AccountId,

    /// The time from when this transaction is valid.
    ///
    /// When a transaction is submitted there is additionally a validDuration (defaults to 120s)
    /// and together they define a time window that a transaction may be processed in.
    ///
    pub valid_start: OffsetDateTime,

    pub nonce: Option<i32>,
    pub scheduled: bool,
}

impl TransactionId {
    /// Generates a new transaction ID for the given account ID.
    #[must_use]
    pub fn generate(account_id: AccountId) -> Self {
        let valid_start = OffsetDateTime::now_utc()
            - Duration::nanoseconds(thread_rng().gen_range(5_000_000_000, 8_000_000_000));

        Self { account_id, valid_start, scheduled: false, nonce: None }
    }
}

impl Debug for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@{}.{}{}{}",
            self.account_id,
            self.valid_start.unix_timestamp(),
            self.valid_start.nanosecond(),
            if self.scheduled { "?scheduled" } else { "" },
            self.nonce.map(|nonce| format!("/{}", nonce)).as_deref().unwrap_or_default()
        )
    }
}

impl ToProtobuf for TransactionId {
    type Protobuf = services::TransactionId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TransactionId {
            account_id: Some(self.account_id.to_protobuf()),
            scheduled: self.scheduled,
            nonce: self.nonce.unwrap_or_default(),
            transaction_valid_start: Some(self.valid_start.into()),
        }
    }
}
