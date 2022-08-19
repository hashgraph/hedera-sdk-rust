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

use hedera_proto::services;

use crate::{
    PublicKey,
    ToProtobuf,
};

#[derive(Debug)]
pub struct SignaturePair {
    pub(crate) signature: Signature,
    pub(crate) public: PublicKey,
}

#[derive(Debug)]
pub struct Signature(SignatureData);

#[derive(Debug)]
enum SignatureData {
    Ed25519(ed25519_dalek::Signature),
}

impl SignaturePair {
    pub(crate) fn ed25519(signature: ed25519_dalek::Signature, public: PublicKey) -> Self {
        Self { public, signature: Signature(SignatureData::Ed25519(signature)) }
    }
}

impl ToProtobuf for SignaturePair {
    type Protobuf = services::SignaturePair;

    fn to_protobuf(&self) -> Self::Protobuf {
        let signature = match self.signature.0 {
            SignatureData::Ed25519(signature) => {
                services::signature_pair::Signature::Ed25519(signature.to_bytes().to_vec())
            }
        };

        services::SignaturePair {
            signature: Some(signature),
            // TODO: is there any way to utilize the _prefix_ nature of this field?
            pub_key_prefix: self.public.to_bytes_raw().to_vec(),
        }
    }
}
