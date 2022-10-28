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
use prost::Message;

use crate::{
    FromProtobuf,
    SemanticVersion,
    ToProtobuf,
};

/// Versions of Hedera Services, and the protobuf schema.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct NetworkVersionInfo {
    /// Version of the protobuf schema in use by the network.
    pub protobuf_version: SemanticVersion,

    /// Version of the Hedera services in use by the network.
    pub services_version: SemanticVersion,
}

impl NetworkVersionInfo {
    /// Create a new `NetworkVersionInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::NetworkGetVersionInfoResponse>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        services::NetworkGetVersionInfoResponse {
            header: None,
            hapi_proto_version: Some(self.protobuf_version.to_protobuf()),
            hedera_services_version: Some(self.services_version.to_protobuf()),
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::response::Response> for NetworkVersionInfo {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, NetworkGetVersionInfo, services::response::Response);
        Self::from_protobuf(pb)
    }
}

impl FromProtobuf<services::NetworkGetVersionInfoResponse> for NetworkVersionInfo {
    fn from_protobuf(pb: services::NetworkGetVersionInfoResponse) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let protobuf_version = pb_getf!(pb, hapi_proto_version)?;
        let services_version = pb_getf!(pb, hedera_services_version)?;

        Ok(Self {
            protobuf_version: SemanticVersion::from_protobuf(protobuf_version)?,
            services_version: SemanticVersion::from_protobuf(services_version)?,
        })
    }
}
