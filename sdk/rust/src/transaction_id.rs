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

use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use hedera_proto::services;
use rand::{
    thread_rng,
    Rng,
};
use time::{
    Duration,
    OffsetDateTime,
};

use crate::{
    AccountId,
    Error,
    FromProtobuf,
    LedgerId,
    ToProtobuf,
    ValidateChecksums,
};

/// The client-generated ID for a transaction.
///
/// This is used for retrieving receipts and records for a transaction, for appending to a file
/// right after creating it, for instantiating a smart contract with bytecode in a file just created,
/// and internally by the network for detecting when duplicate transactions are submitted.
///
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
pub struct TransactionId {
    /// The account that pays for this transaction.
    pub account_id: AccountId,

    /// The time from when this transaction is valid.
    ///
    /// When a transaction is submitted there is additionally a
    /// [`valid_duration`](crate::Transaction::transaction_valid_duration) (defaults to 120s)
    /// and together they define a time window that a transaction may be processed in.
    pub valid_start: OffsetDateTime,

    /// Nonce for this transaction.
    pub nonce: Option<i32>,

    /// `true` if the transaction is `scheduled`.
    pub scheduled: bool,
}

impl TransactionId {
    /// Generates a new transaction ID for the given account ID.
    #[must_use]
    pub fn generate(account_id: AccountId) -> Self {
        let valid_start = OffsetDateTime::now_utc()
            - Duration::nanoseconds(thread_rng().gen_range(5_000_000_000..8_000_000_000));

        Self { account_id, valid_start, scheduled: false, nonce: None }
    }

    /// Create a new `TransactionId` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
}

impl ValidateChecksums for TransactionId {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.account_id.validate_checksums_for_ledger_id(ledger_id)
    }
}

impl Debug for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
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
            self.nonce.map(|nonce| format!("/{nonce}")).as_deref().unwrap_or_default()
        )
    }
}

impl FromProtobuf<services::TransactionId> for TransactionId {
    fn from_protobuf(pb: services::TransactionId) -> crate::Result<Self> {
        let account_id = pb_getf!(pb, account_id)?;
        let account_id = AccountId::from_protobuf(account_id)?;

        let valid_start = pb_getf!(pb, transaction_valid_start)?;

        Ok(Self {
            account_id,
            valid_start: valid_start.into(),
            nonce: (pb.nonce != 0).then_some(pb.nonce),
            scheduled: pb.scheduled,
        })
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

// TODO: add unit tests to prove parsing
// TODO: potentially improve parsing with `nom` or `combine`
impl FromStr for TransactionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPECTED: &str = "expecting <accountId>@<validStart>[?scheduled][/<nonce>]";
        // parse route:
        // split_once('@') -> ("<accountId>", "<validStart>[?scheduled][/<nonce>]")
        // rsplit_once('/') -> Either ("<validStart>[?scheduled]", "<nonce>") or ("<validStart>[?scheduled]")
        // .strip_suffix("?scheduled") -> ("<validStart>") and the suffix was either removed or not.
        // (except it's better ux to do a `split_once('?')`... Except it doesn't matter that much)

        let (account_id, s) = s.split_once('@').ok_or_else(|| Error::basic_parse(EXPECTED))?;

        let account_id: AccountId = account_id.parse()?;

        let (s, nonce) = match s.rsplit_once('/') {
            Some((s, nonce)) => (s, Some(nonce)),
            None => (s, None),
        };

        let nonce = nonce.map(i32::from_str).transpose().map_err(Error::basic_parse)?;

        let (valid_start, scheduled) = match s.strip_suffix("?scheduled") {
            Some(rest) => (rest, true),
            None => (s, false),
        };

        let valid_start = {
            let (seconds, nanos) =
                valid_start.split_once('.').ok_or_else(|| Error::basic_parse(EXPECTED))?;

            let seconds = i64::from_str(seconds).map_err(Error::basic_parse)?;
            let nanos = i64::from_str(nanos).map_err(Error::basic_parse)?;

            OffsetDateTime::from_unix_timestamp(seconds).map_err(Error::basic_parse)?
                + Duration::nanoseconds(nanos)
        };

        Ok(Self { account_id, valid_start, nonce, scheduled })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use assert_matches::assert_matches;

    use crate::{
        AccountId,
        TransactionId,
    };

    #[test]
    fn from_str_wrong_field() {
        assert_matches!(TransactionId::from_str("0.0.31415?1641088801.2"), Err(_));
    }

    #[test]
    fn from_str_wrong_field2() {
        assert_matches!(TransactionId::from_str("0.0.31415/1641088801.2"), Err(_));
    }

    #[test]
    fn from_str_out_of_order() {
        assert_matches!(TransactionId::from_str("0.0.31415?scheduled/1412@1641088801.2"), Err(_));
    }

    #[test]
    fn from_str_single_digit_nanos() {
        let id = TransactionId {
            account_id: AccountId::from(31415),
            valid_start: time::Date::from_calendar_date(2022, time::Month::January, 2)
                .unwrap()
                .with_hms_nano(2, 0, 1, 2)
                .unwrap()
                .assume_utc(),
            nonce: None,
            scheduled: false,
        };

        assert_eq!(id, "0.0.31415@1641088801.2".parse().unwrap());
    }

    #[test]
    fn display_single_digit_nanos() {
        let id = TransactionId {
            account_id: AccountId::from(31415),
            valid_start: time::Date::from_calendar_date(2022, time::Month::January, 2)
                .unwrap()
                .with_hms_nano(2, 0, 1, 2)
                .unwrap()
                .assume_utc(),
            nonce: None,
            scheduled: false,
        };

        assert_eq!(id.to_string(), "0.0.31415@1641088801.2");
    }
}
