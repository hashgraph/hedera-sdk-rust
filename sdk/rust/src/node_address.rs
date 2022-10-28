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

use std::net::SocketAddrV4;

use hedera_proto::services;
use serde_with::base64::Base64;

use crate::{
    AccountId,
    FromProtobuf,
};

/// The data about a node, including its service endpoints and the Hedera account to be paid for
/// services provided by the node (that is, queries answered and transactions submitted.).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    /// A non-sequential, unique, static identifier for the node
    pub node_id: u64,

    /// The node's X509 RSA public key used to sign stream files.
    #[serde(with = "serde_with::As::<Base64>")]
    pub rsa_public_key: Vec<u8>,

    /// The account to be paid for queries and transactions sent to this node.
    pub node_account_id: AccountId,

    /// Hash of the node's TLS certificate.
    ///
    /// Precisely, this field is a string of
    /// hexadecimal characters which, translated to binary, are the SHA-384 hash of
    /// the UTF-8 NFKD encoding of the node's TLS cert in PEM format.
    ///
    /// Its value can be used to verify the node's certificate it presents during TLS negotiations.
    #[serde(with = "serde_with::As::<Base64>")]
    pub tls_certificate_hash: Vec<u8>,

    /// A node's service IP addresses and ports.
    pub service_endpoints: Vec<SocketAddrV4>,

    /// A description of the node, up to 100 bytes.
    pub description: String,
}

impl FromProtobuf<services::NodeAddress> for NodeAddress {
    fn from_protobuf(pb: services::NodeAddress) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let node_account_id = AccountId::from_protobuf(pb_getf!(pb, node_account_id)?)?;

        Ok(Self {
            description: pb.description,
            rsa_public_key: pb.rsa_pub_key.into_bytes(),
            node_id: pb.node_id as u64,
            service_endpoints: vec![],
            tls_certificate_hash: pb.node_cert_hash,
            node_account_id,
        })
    }
}
