/*
 * ‌
 * Hedera Swift SDK
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

import CHedera
import Foundation
import HederaProtobufs

/// Hedera follows semantic versioning for both the HAPI protobufs and
/// the Services software.
public struct SemanticVersion: Codable, ExpressibleByStringLiteral, LosslessStringConvertible {
    /// Increases with incompatible API changes
    public let major: UInt32

    /// Increases with backwards-compatible new functionality
    public let minor: UInt32

    /// Increases with backwards-compatible bug fixes
    public let patch: UInt32

    /// A pre-release version MAY be denoted by appending a hyphen and a series of dot separated identifiers (https://semver.org/#spec-item-9);
    /// so given a semver 0.14.0-alpha.1+21AF26D3, this field would contain ‘alpha.1’
    public let prerelease: String

    /// The build metadata, if any.
    ///
    /// Build metadata MAY be denoted by appending a plus sign and a series of dot separated identifiers
    /// immediately following the patch or pre-release version (https://semver.org/#spec-item-10);
    /// so given a semver `0.14.0-alpha.1+21AF26D3`, this field would contain `21AF26D3`
    public let build: String

    public init(major: UInt32, minor: UInt32, patch: UInt32, prerelease: String = "", build: String = "") {
        self.major = major
        self.minor = minor
        self.patch = patch
        self.prerelease = prerelease
        self.build = build
    }

    // internal API, do NOT expose.
    private init(parsing description: String) throws {
        var csemver = HederaSemanticVersion()

        try HError.throwing(error: hedera_semantic_version_from_string(description, &csemver))

        self.init(unsafeFromCHedera: csemver)
    }

    public init(stringLiteral value: StringLiteralType) {
        // If you're using a string literal this will either *always* fail or *never* fail, so, force try makes sense.
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    // semver parsing is shockingly hard. So the FFI really does carry its weight.
    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    internal init(unsafeFromCHedera hedera: HederaSemanticVersion) {
        major = hedera.major
        minor = hedera.minor
        patch = hedera.patch
        prerelease = hedera.prerelease.map { String(hString: $0) } ?? ""
        build = hedera.build.map { String(hString: $0) } ?? ""
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaSemanticVersion) throws -> Result) rethrows -> Result {
        try prerelease.withCString { (prerelease) in
            try build.withCString { (build) in
                let mutPrerelease = UnsafeMutablePointer(mutating: prerelease)
                let mutBuild = UnsafeMutablePointer(mutating: build)
                let csemver = HederaSemanticVersion(
                    major: major, minor: minor, patch: patch, prerelease: mutPrerelease, build: mutBuild)

                return try body(csemver)
            }
        }
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }

    public var description: String {
        let prerelease = prerelease.isEmpty ? "" : "-\(prerelease)"
        let build = build.isEmpty ? "" : "+\(build)"
        return "\(major).\(minor).\(patch)\(prerelease)\(build)"
    }

    public func toString() -> String {
        description
    }
}

extension SemanticVersion: ProtobufCodable {
    internal typealias Protobuf = Proto_SemanticVersion

    internal init(fromProtobuf proto: Protobuf) {
        self.init(
            major: UInt32(proto.major),
            minor: UInt32(proto.minor),
            patch: UInt32(proto.patch),
            prerelease: proto.pre,
            build: proto.build
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.major = Int32(major)
            proto.minor = Int32(minor)
            proto.patch = Int32(patch)
            proto.pre = prerelease
            proto.build = build
        }
    }
}
