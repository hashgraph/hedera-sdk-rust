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

use std::fmt;
use std::sync::Arc;

use crate::{
    PrivateKey,
    PublicKey,
};

#[derive(Clone)]
pub(crate) enum AnySigner {
    PrivateKey(PrivateKey),
    // public key is 216 bytes.
    // Here be a story of dragons.
    // Once an engineer attempted to downgrade this `Arc` to a mere `Box`, alas it was not meant to be.
    // For the Fn must be cloned, and `dyn Fn` must not.
    // The plan to not pay the price of Arc was doomed from the very beginning.
    // Attempts to avoid the arc, the cloning of the `Fn`, all end in misery,
    // for the `Client` must have `AnySigner`, not a `PrivateKey`, and the `ContractCreateFlow`...
    // Well, it must be executable multiple times, for ownership reasons.
    Arbitrary(Box<PublicKey>, Arc<dyn Fn(&[u8]) -> Vec<u8> + Send + Sync>),
}

impl fmt::Debug for AnySigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrivateKey(_) => f.debug_tuple("PrivateKey").field(&"[redacted]").finish(),
            Self::Arbitrary(arg0, _) => {
                f.debug_tuple("Arbitrary").field(arg0).field(&"Fn").finish()
            }
        }
    }
}

impl AnySigner {
    // *Cheap* Accessor to get the public key without signing the message first.
    pub(crate) fn public_key(&self) -> PublicKey {
        match self {
            AnySigner::PrivateKey(it) => it.public_key(),
            AnySigner::Arbitrary(it, _) => **it,
        }
    }

    pub(crate) fn sign(&self, message: &[u8]) -> (PublicKey, Vec<u8>) {
        match self {
            AnySigner::PrivateKey(it) => (it.public_key(), it.sign(message)),
            AnySigner::Arbitrary(public, signer) => {
                let bytes = signer(message);

                (**public, bytes)
            }
        }
    }
}
