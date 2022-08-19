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

use crate::FromProtobuf;

/// Hedera follows semantic versioning for both the HAPI protobufs and
/// the Services software.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SemanticVersion {
    /// Increases with incompatible API changes
    pub major: u32,

    /// Increases with backwards-compatible new functionality
    pub minor: u32,

    /// Increases with backwards-compatible bug fixes
    pub patch: u32,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVersionInfo {
    /// Version of the protobuf schema in use by the network.
    pub protobuf_version: SemanticVersion,

    /// Version of the Hedera services in use by the network.
    pub services_version: SemanticVersion,
}

impl FromProtobuf<services::response::Response> for NetworkVersionInfo {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, NetworkGetVersionInfo, services::response::Response);
        let protobuf_version = pb_getf!(pb, hapi_proto_version)?;
        let services_version = pb_getf!(pb, hedera_services_version)?;

        Ok(Self {
            protobuf_version: SemanticVersion::from_protobuf(protobuf_version)?,
            services_version: SemanticVersion::from_protobuf(services_version)?,
        })
    }
}

impl FromProtobuf<services::SemanticVersion> for SemanticVersion {
    fn from_protobuf(pb: services::SemanticVersion) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { major: pb.major as u32, minor: pb.minor as u32, patch: pb.patch as u32 })
    }
}
