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

use crate::{
    PrivateKey,
    PublicKey,
};

pub(crate) enum Signer {
    PrivateKey(PrivateKey),
    Arbitrary(PublicKey, Box<dyn Fn(&[u8]) -> Vec<u8> + Send + Sync>),
    // #[cfg(feature = "ffi")]
    // C(CSigner),
}

impl Signer {
    // *Cheap* Accessor to get the public key without signing the message first.
    pub(crate) fn public_key(&self) -> PublicKey {
        match self {
            Signer::PrivateKey(it) => it.public_key(),
            Signer::Arbitrary(it, _) => *it,
        }
    }

    pub(crate) fn sign(&self, message: &[u8]) -> (PublicKey, Vec<u8>) {
        match self {
            Signer::PrivateKey(it) => (it.public_key(), it.sign(message)),
            Signer::Arbitrary(public, signer) => {
                let bytes = signer(&message);
                (*public, bytes)
            }
        }
    }
}
