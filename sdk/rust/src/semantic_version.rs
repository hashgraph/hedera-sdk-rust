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
    FromProtobuf,
    ToProtobuf,
};

/// Hedera follows [semantic versioning](https://semver.org) for both the HAPI protobufs and
/// the Services software.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SemanticVersion {
    /// Increases with incompatible API changes
    pub major: u32,

    /// Increases with backwards-compatible new functionality
    pub minor: u32,

    /// Increases with backwards-compatible bug fixes]
    pub patch: u32,

    /// A pre-release version MAY be denoted by appending a hyphen and a series of dot separated identifiers (https://semver.org/#spec-item-9);
    /// so given a semver 0.14.0-alpha.1+21AF26D3, this field would contain ‘alpha.1’
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prerelease: String,
    /// Build metadata MAY be denoted by appending a plus sign and a series of dot separated identifiers
    /// immediately following the patch or pre-release version (https://semver.org/#spec-item-10);
    /// so given a semver 0.14.0-alpha.1+21AF26D3, this field would contain ‘21AF26D3’
    #[serde(skip_serializing_if = "String::is_empty")]
    pub build: String,
}

impl SemanticVersion {
    /// Create a new `NetworkVersionInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
}

impl FromProtobuf<services::SemanticVersion> for SemanticVersion {
    fn from_protobuf(pb: services::SemanticVersion) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            major: pb.major as u32,
            minor: pb.minor as u32,
            patch: pb.patch as u32,
            prerelease: pb.pre,
            build: pb.build,
        })
    }
}

impl ToProtobuf for SemanticVersion {
    type Protobuf = services::SemanticVersion;
    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            major: self.major as i32,
            minor: self.minor as i32,
            patch: self.patch as i32,
            pre: self.prerelease.clone(),
            build: self.build.clone(),
        }
    }
}
