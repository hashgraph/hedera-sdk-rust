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

pub struct Signature(SignatureData);

impl Signature {
    pub(crate) fn as_ed25519(&self) -> Option<&ed25519_dalek::Signature> {
        if let SignatureData::Ed25519(v) = &self.0 {
            Some(v)
        } else {
            None
        }
    }

    pub(crate) fn as_ecdsa_secp256k1(&self) -> Option<&k256::ecdsa::Signature> {
        if let SignatureData::EcdsaSecp256k1(v) = &self.0 {
            Some(v)
        } else {
            None
        }
    }
}

// blame `Debug` inconsistency in different crypto libraries for this.
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            SignatureData::Ed25519(signature) => {
                write!(f, "Signature::Ed25519({})", hex::encode(signature.to_bytes()))
            }
            SignatureData::EcdsaSecp256k1(signature) => {
                write!(f, "Signature::EcdsaSecp256k1({})", hex::encode(signature.to_vec()))
            }
        }
    }
}

#[derive(Debug)]
enum SignatureData {
    Ed25519(ed25519_dalek::Signature),
    EcdsaSecp256k1(k256::ecdsa::Signature),
}

impl SignaturePair {
    pub(crate) fn ed25519(signature: ed25519_dalek::Signature, public: PublicKey) -> Self {
        Self { public, signature: Signature(SignatureData::Ed25519(signature)) }
    }

    pub(crate) fn ecdsa_secp256k1(signature: k256::ecdsa::Signature, public: PublicKey) -> Self {
        Self { public, signature: Signature(SignatureData::EcdsaSecp256k1(signature)) }
    }
}

impl ToProtobuf for SignaturePair {
    type Protobuf = services::SignaturePair;

    fn to_protobuf(&self) -> Self::Protobuf {
        let signature = match self.signature.0 {
            SignatureData::Ed25519(signature) => {
                services::signature_pair::Signature::Ed25519(signature.to_bytes().to_vec())
            }
            SignatureData::EcdsaSecp256k1(signature) => {
                services::signature_pair::Signature::EcdsaSecp256k1(signature.to_vec())
            }
        };

        services::SignaturePair {
            signature: Some(signature),
            // TODO: is there any way to utilize the _prefix_ nature of this field?
            pub_key_prefix: self.public.to_bytes_raw(),
        }
    }
}
