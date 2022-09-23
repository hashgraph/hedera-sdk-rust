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
    Hbar,
    Status,
    TransactionId,
};

pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxStdError = Box<dyn StdError + Send + Sync + 'static>;

/// Represents any possible error from a fallible function in the Hedera SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to complete request within the maximum time allowed; most recent attempt failed with: {0}")]
    TimedOut(Box<Error>),

    #[error("grpc: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("failed to create a SDK type from a protobuf response: {0}")]
    FromProtobuf(BoxStdError),

    #[error("transaction `{transaction_id}` failed pre-check with status `{status:?}`")]
    TransactionPreCheckStatus { status: Status, transaction_id: TransactionId },

    #[error("transaction without transaction id failed pre-check with status `{status:?}`")]
    TransactionNoIdPreCheckStatus { status: Status },

    #[error("query for transaction `{transaction_id}` failed pre-check with status `{status:?}`")]
    QueryPreCheckStatus { status: Status, transaction_id: TransactionId },

    #[error(
    "query with payment transaction `{transaction_id}` failed pre-check with status `{status:?}`"
    )]
    QueryPaymentPreCheckStatus { status: Status, transaction_id: TransactionId },

    #[error("query with no payment transaction failed pre-check with status `{status:?}`")]
    QueryNoPaymentPreCheckStatus { status: Status },

    /// Failed to parse a basic type from string (ex. AccountId, ContractId, TransactionId, etc.).
    #[error("failed to parse: {0}")]
    BasicParse(BoxStdError),

    #[error("failed to parse a key: {0}")]
    KeyParse(BoxStdError),

    #[error("failed to parse a mnemonic: {reason}")]
    MnemonicParse { reason: MnemonicParseError, mnemonic: crate::Mnemonic },

    #[error("failed to convert a mnemonic to entropy: {0}")]
    MnemonicEntropy(MnemonicEntropyError),

    #[error("client must be configured with a payer account or requests must be given an explicit transaction id")]
    NoPayerAccountOrTransactionId,

    #[error("cost of {query_cost} without explicit payment is greater than the maximum allowed payment of {max_query_payment}")]
    MaxQueryPaymentExceeded { query_cost: Hbar, max_query_payment: Hbar },

    #[error("node account `{0}` was not found in the configured network")]
    NodeAccountUnknown(AccountId),

    #[error("received unrecognized status code: {0}, try updating your SDK")]
    ResponseStatusUnrecognized(i32),

    #[error("receipt for transaction `{transaction_id}` failed with status `{status:?}`")]
    ReceiptStatus { status: Status, transaction_id: TransactionId },

    #[error("failed to sign request: {0}")]
    Signature(BoxStdError),

    #[error("failed to verify a signature: {0}")]
    SignatureVerify(BoxStdError),

    #[cfg(feature = "ffi")]
    #[error("failed to parse a request from JSON: {0}")]
    RequestParse(BoxStdError),
}

impl Error {
    pub(crate) fn from_protobuf<E: Into<BoxStdError>>(error: E) -> Self {
        Self::FromProtobuf(error.into())
    }

    pub(crate) fn key_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::KeyParse(error.into())
    }

    pub(crate) fn basic_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::BasicParse(error.into())
    }

    #[cfg(feature = "ffi")]
    pub(crate) fn request_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::RequestParse(error.into())
    }

    pub(crate) fn signature<E: Into<BoxStdError>>(error: E) -> Self {
        Self::Signature(error.into())
    }

    pub(crate) fn signature_verify(error: impl Into<BoxStdError>) -> Self {
        Self::SignatureVerify(error.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MnemonicParseError {
    #[error("bad length: expected `12` or `24` words, found `{0}`")]
    BadLength(usize),

    #[error("unknown words at indecies: `{0:?}`")]
    UnknownWords(Vec<usize>),

    #[error("checksum mismatch: expected `{expected:02x}`, found `{actual:02x}`")]
    ChecksumMismatch { expected: u8, actual: u8 },
}

// todo: find a better name before release.
#[derive(Debug, thiserror::Error)]
pub enum MnemonicEntropyError {
    #[error("bad length: expected `{expected}` words, found {actual} words")]
    BadLength { expected: usize, actual: usize },

    #[error("checksum mismatch: expected `{expected:02x}`, found `{actual:02x}`")]
    ChecksumMismatch { expected: u8, actual: u8 },

    #[error("used a passphrase with a legacy mnemonic")]
    LegacyWithPassphrase,
}
