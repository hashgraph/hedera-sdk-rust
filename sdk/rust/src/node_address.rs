use std::net::SocketAddrV4;

use hedera_proto::services;
use serde_with::base64::Base64;
use serde_with::serde_as;

use crate::{AccountId, FromProtobuf};

/// The data about a node, including its service endpoints and the Hedera account to be paid for
/// services provided by the node (that is, queries answered and transactions submitted.).
#[serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    /// A non-sequential, unique, static identifier for the node
    pub node_id: u64,

    /// The node's X509 RSA public key used to sign stream files.
    #[serde_as(as = "Base64")]
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
    #[serde_as(as = "Base64")]
    pub tls_certificate_hash: Vec<u8>,

    /// A node's service IP addresses and ports.
    pub service_endpoints: Vec<SocketAddrV4>,

    /// A description of the node, up to 100 bytes.
    pub description: String,
}

impl FromProtobuf for NodeAddress {
    type Protobuf = services::NodeAddress;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        println!("NodeAddress.service_endpoint[0] = {:?}", pb.service_endpoint[0].ip_address_v4);

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
