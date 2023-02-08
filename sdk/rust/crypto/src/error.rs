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

/// `Result<T, Error>`
pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxStdError = Box<dyn StdError + Send + Sync + 'static>;

/// Represents any possible error from a fallible function in the Hedera SDK.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
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

    /// Failed to verify a signature.
    #[error("failed to verify a signature: {0}")]
    SignatureVerify(#[source] BoxStdError),
}

impl Error {
    pub(crate) fn key_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::KeyParse(error.into())
    }

    pub(crate) fn key_derive<E: Into<BoxStdError>>(error: E) -> Self {
        Self::KeyDerive(error.into())
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
