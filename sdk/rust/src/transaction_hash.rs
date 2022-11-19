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

use sha2::{
    Digest,
    Sha384,
};

/// The client-generated SHA-384 hash of a transaction that was submitted.
///
/// This can be used to lookup the transaction in an explorer.
#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay))]
pub struct TransactionHash(pub [u8; 48]);

impl TransactionHash {
    #[must_use]
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self(Sha384::digest(bytes).into())
    }
}

impl Debug for TransactionHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for TransactionHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&hex::encode(self.0))
    }
}
