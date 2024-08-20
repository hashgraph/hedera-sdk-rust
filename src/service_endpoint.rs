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

use std::net::{
    Ipv4Addr,
    SocketAddrV4,
};

use hedera_proto::services;

use crate::protobuf::ToProtobuf;
use crate::{
    Error,
    FromProtobuf,
};

fn parse_socket_addr_v4(ip: Vec<u8>, port: i32) -> crate::Result<SocketAddrV4> {
    let octets: Result<[u8; 4], _> = ip.try_into();
    let octets = octets.map_err(|v| {
        Error::from_protobuf(format!("expected 4 byte ip address, got `{}` bytes", v.len()))
    })?;

    let port = u16::try_from(port).map_err(|_| {
        Error::from_protobuf(format!(
            "expected 16 bit non-negative port number, but the port was actually `{port}`",
        ))
    })?;

    Ok(SocketAddrV4::new(octets.into(), port))
}

/// Contains the IP address and the port representing a service endpoint of
/// a Node in a network. Used to reach the Hedera API and submit transactions
/// to the network.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceEndpoint {
    /// The 4-byte IPv4 address of the endpoint encoded in left to right order
    pub ip_address_v4: Option<Ipv4Addr>,

    /// The port of the service endpoint
    pub port: i32,

    /// A node domain name.<br/>
    /// This MUST be the fully qualified domain(DNS) name of the node.<br/>
    /// This value MUST NOT be more than 253 characters.
    /// domain_name and ipAddressV4 are mutually exclusive.
    /// When the `domain_name` field is set, the `ipAddressV4` field MUST NOT be set.<br/>
    /// When the `ipAddressV4` field is set, the `domain_name` field MUST NOT be set.
    pub domain_name: String,
}

impl FromProtobuf<services::ServiceEndpoint> for ServiceEndpoint {
    fn from_protobuf(pb: services::ServiceEndpoint) -> crate::Result<Self> {
        let socket_addr_v4 = parse_socket_addr_v4(pb.ip_address_v4, pb.port)?;

        Ok(Self {
            ip_address_v4: Some(socket_addr_v4.ip().to_owned()),
            port: socket_addr_v4.port() as i32,
            domain_name: pb.domain_name,
        })
    }
}

impl ToProtobuf for ServiceEndpoint {
    type Protobuf = services::ServiceEndpoint;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::ServiceEndpoint {
            ip_address_v4: self.ip_address_v4.unwrap().octets().to_vec(),
            port: self.port,
            domain_name: self.domain_name.clone(),
        }
    }
}
