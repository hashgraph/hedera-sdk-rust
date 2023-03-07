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

use std::error::Error as StdError;
use std::result::Result as StdResult;

use crate::{
    AccountId,
    Checksum,
    Hbar,
    Status,
    TransactionId,
};

/// `Result<T, Error>`
pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxStdError = Box<dyn StdError + Send + Sync + 'static>;

/// Represents any possible error from a fallible function in the Hedera SDK.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Request timed out.
    #[error("failed to complete request within the maximum time allowed; most recent attempt failed with: {0}")]
    TimedOut(#[source] Box<Error>),

    /// GRPC status code was an error.
    #[error("grpc: {0}")]
    GrpcStatus(#[from] tonic::Status),

    /// Failed to parse an SDK type from a protobuf response.
    #[error("failed to create a SDK type from a protobuf response: {0}")]
    FromProtobuf(#[source] BoxStdError),

    // todo: bikeshed this.
    /// Freeze failed due to there being no explicitly set node account IDs and no client being provided to generate them.
    #[error("freeze failed due to node account IDs being unset")]
    FreezeUnsetNodeAccountIds,

    /// A transaction failed pre-check.
    ///
    /// The transaction had the ID `transaction_id`.
    ///
    /// Caused by `status` being an error.
    #[error("transaction `{transaction_id}` failed pre-check with status `{status:?}`")]
    TransactionPreCheckStatus {
        /// The status that caused the [`Transaction`](crate::Transaction) to fail pre-check.
        status: Status,
        /// The `TransactionId` of the failed [`Transaction`](crate::Transaction) .
        transaction_id: TransactionId,
    },

    /// A transaction failed pre-check.
    ///
    /// The transaction had no `transaction_id`.
    ///
    /// Caused by `status` being an error.
    #[error("transaction without transaction id failed pre-check with status `{status:?}`")]
    TransactionNoIdPreCheckStatus {
        /// The `Status` that caused the [`Transaction`](crate::Transaction) to fail pre-check.
        status: Status,
    },

    /// A [`Query`](crate::Query) for `transaction_id` failed pre-check.
    ///
    /// Caused by `status` being an error.
    #[error("query for transaction `{transaction_id}` failed pre-check with status `{status:?}`")]
    QueryPreCheckStatus {
        /// The `Status` that caused the [`Query`](crate::Query) to fail pre-check.
        status: Status,
        /// The associated transaction's ID.
        transaction_id: TransactionId,
    },

    /// A [`Query`](crate::Query) failed pre-check.
    ///
    /// The query had an associated `PaymentTransaction` with ID `transaction_id`.
    ///
    /// Caused by `status` being an error.
    #[error(
    "query with payment transaction `{transaction_id}` failed pre-check with status `{status:?}`"
    )]
    QueryPaymentPreCheckStatus {
        /// The `Status` that caused the [`Query`](crate::Query) to fail pre-check.
        status: Status,
        /// The associated `PaymentTransaction`'s `TransactionId`.
        transaction_id: TransactionId,
    },

    /// A [`Query`](crate::Query) failed pre-check.
    ///
    /// The query had no `PaymentTransaction`.
    ///
    /// Caused by `status` being an error.
    #[error("query with no payment transaction failed pre-check with status `{status:?}`")]
    QueryNoPaymentPreCheckStatus {
        /// The `Status` that caused the [`Query`](crate::Query) to fail pre-check.
        status: Status,
    },

    /// Failed to parse a basic type from string
    /// (ex. [`AccountId`](crate::AccountId), [`ContractId`](crate::ContractId), [`TransactionId`](crate::TransactionId), etc.).
    #[error("failed to parse: {0}")]
    BasicParse(#[source] BoxStdError),

    /// An entity ID had an invalid checksum
    #[error("entity ID {shard}.{realm}.{num}-{present_checksum} was incorrect")]
    BadEntityId {
        /// The shard number
        shard: u64,
        /// The realm number
        realm: u64,
        /// The entity number
        num: u64,
        /// The (invalid) checksum that was present on the entity ID
        present_checksum: Checksum,
        /// The checksum that SHOULD HAVE BEEN on the entity ID
        expected_checksum: Checksum,
    },

    /// An entity ID cannot be converted to a string with a checksum, because it takes an alternate form,
    /// such as an `alias` or `evm_address`
    #[error("to_string_with_checksum() can't be applied to entity ID with alias or evm_address")]
    CannotToStringWithChecksum,

    /// A task can't be performed because the Client doesn't have a ledger ID set.
    #[error("can't {task} without knowing which ledger the entity ID is for; ensure client's ledger ID is set")]
    CannotPerformTaskWithoutLedgerId {
        /// The task that can't be performed
        task: &'static str,
    },

    /// A task cannot be performed because no `evm_address` field is present.
    #[error("can't {task} because evm_address is not present")]
    NoEvmAddressPresent {
        /// The task that can't be performed
        task: &'static str,
    },

    /// A task cannot be performed because a key is of the wrong type.
    #[error("can't {task} because key {key_enum} cannot be of type {key_variant}")]
    WrongKeyType {
        /// The task that can't be performed
        task: &'static str,
        /// The name of the key enum (EG `PublicKey` or `PrivateKey`)
        key_enum: &'static str,
        /// The type of the key enum
        key_variant: &'static str,
    },

    /// Failed to parse a [`PublicKey`](crate::PublicKey) or [`PrivateKey`](crate::PrivateKey).
    #[error("failed to parse a key: {0}")]
    KeyParse(#[source] BoxStdError),

    /// Failed to derive a [`PrivateKey`](crate::PrivateKey) from another `PrivateKey`.
    ///
    /// Examples of when this can happen (non-exhaustive):
    /// - [`PrivateKey::derive`](fn@crate::PrivateKey::derive) when the `PrivateKey` doesn't have a chain code.
    /// - [`PrivateKey::derive`](fn@crate::PrivateKey::derive)
    ///   or [`PrivateKey::legacy_derive`](fn@crate::PrivateKey::legacy_derive) on an `Ecsda` key.
    #[error("Failed to derive a key: {0}")]
    KeyDerive(#[source] BoxStdError),

    /// Failed to parse a [`Mnemonic`](crate::Mnemonic) due to the given `reason`.
    ///
    /// the `Mnemonic` is provided because invalid `Mnemonics`
    /// can technically still provide valid [`PrivateKeys`](crate::PrivateKey).
    #[error("failed to parse a mnemonic: {reason}")]
    MnemonicParse {
        /// This error's source.
        #[source]
        reason: MnemonicParseError,
        /// The `Mnemonic` in question.
        mnemonic: crate::Mnemonic,
    },

    /// An error occurred while attempting to convert a [`Mnemonic`](crate::Mnemonic) to a [`PrivateKey`](crate::PrivateKey)
    #[error("failed to convert a mnemonic to entropy: {0}")]
    MnemonicEntropy(#[from] MnemonicEntropyError),

    /// The [`Client`](crate::Client) had no payer account (operator)
    /// and the attempted request had no explicit [`TransactionId`].
    #[error("client must be configured with a payer account or requests must be given an explicit transaction id")]
    NoPayerAccountOrTransactionId,

    /// Cost of a [`Query`](crate::Query) is more expensive than `max_query_payment`.
    ///
    /// The actual cost of the `Query` is `query_cost`.
    #[error("cost of {query_cost} without explicit payment is greater than the maximum allowed payment of {max_query_payment}")]
    MaxQueryPaymentExceeded {
        /// the configured maximum query payment.
        max_query_payment: Hbar,

        /// How much the query actually cost.
        query_cost: Hbar,
    },

    /// The associated node account was not found in the network.
    #[error("node account `{0}` was not found in the configured network")]
    NodeAccountUnknown(AccountId),

    /// Received an unrecognized status code from the Hedera Network.
    ///
    /// This can happen when the SDK is outdated, try updating your SDK.
    #[error("received unrecognized status code: {0}, try updating your SDK")]
    ResponseStatusUnrecognized(i32),

    // fixme(sr): Citation needed (unsure if this is accurate).
    /// Getting the receipt for `transaction_id` failed with `status`.
    #[error("receipt for transaction `{transaction_id:?}` failed with status `{status:?}`")]
    ReceiptStatus {
        /// The Error's status code.
        status: Status,
        /// The [`Transaction`](crate::Transaction)'s ID.
        transaction_id: Option<TransactionId>,
    },

    /// Failed to sign a message.
    #[error("failed to sign message: {0}")]
    Signature(#[source] BoxStdError),

    /// Failed to verify a signature.
    #[error("failed to verify a signature: {0}")]
    SignatureVerify(#[source] BoxStdError),

    /// Failed to parse a request from JSON.
    #[cfg(feature = "ffi")]
    #[error("failed to parse a request from JSON: {0}")]
    RequestParse(#[source] BoxStdError),
}

impl Error {
    pub(crate) fn from_protobuf<E: Into<BoxStdError>>(error: E) -> Self {
        Self::FromProtobuf(error.into())
    }

    pub(crate) fn key_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::KeyParse(error.into())
    }

    pub(crate) fn key_derive<E: Into<BoxStdError>>(error: E) -> Self {
        Self::KeyDerive(error.into())
    }

    pub(crate) fn basic_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::BasicParse(error.into())
    }

    #[cfg(feature = "ffi")]
    pub(crate) fn request_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::RequestParse(error.into())
    }

    pub(crate) fn _signature<E: Into<BoxStdError>>(error: E) -> Self {
        Self::Signature(error.into())
    }

    pub(crate) fn signature_verify(error: impl Into<BoxStdError>) -> Self {
        Self::SignatureVerify(error.into())
    }
}

/// Failed to parse a mnemonic.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MnemonicParseError {
    /// The [`Mnemonic`](crate::Mnemonic) contains an unexpected length.
    #[error("bad length: expected `12` or `24` words, found `{0}`")]
    BadLength(usize),

    /// The [`Mnemonic`](crate::Mnemonic) contains words that aren't in the wordlist.
    #[error("unknown words at indecies: `{0:?}`")]
    UnknownWords(Vec<usize>),

    /// The [`Mnemonic`](crate::Mnemonic) has an invalid checksum.
    #[error("checksum mismatch: expected `{expected:02x}`, found `{actual:02x}`")]
    ChecksumMismatch {
        /// The checksum that was expected.
        expected: u8,
        /// The checksum that was actually found.
        actual: u8,
    },
}

/// Failed to convert a [`Mnemonic`](crate::Mnemonic) to a [`PrivateKey`](crate::PrivateKey)
// todo: find a better name before release.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MnemonicEntropyError {
    /// Encountered a [`Mnemonic`](crate::Mnemonic) of unexpected length.
    #[error("bad length: expected `{expected}` words, found {actual} words")]
    BadLength {
        /// The number of words that were expected (12, 22, or 24)
        expected: usize,
        /// The number of words that were actually found.
        actual: usize,
    },

    /// The [`Mnemonic`](crate::Mnemonic) has an invalid checksum.
    #[error("checksum mismatch: expected `{expected:02x}`, found `{actual:02x}`")]
    ChecksumMismatch {
        /// The checksum that was expected.
        expected: u8,
        /// The checksum that was actually found.
        actual: u8,
    },

    /// Used a passphrase with a legacy [`Mnemonic`](crate::Mnemonic).
    #[error("used a passphrase with a legacy mnemonic")]
    LegacyWithPassphrase,
}
