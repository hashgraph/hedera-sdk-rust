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
    private static func fromString(_ description: String) throws -> Self {
        var csemver = HederaSemanticVersion()
        let err = hedera_semantic_version_from_string(description, &csemver)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return Self(unsafeFromCHedera: csemver)
    }

    public init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    // semver parsing is shockingly hard. So the FFI really does carry its weight.
    public init?(_ description: String) {
        let res = try? Self.fromString(description)

        if res == nil {
            return nil
        }

        self = res!
    }

    internal init(unsafeFromCHedera hedera: HederaSemanticVersion) {
        major = hedera.major
        minor = hedera.minor
        patch = hedera.patch
        prerelease = hedera.prerelease == nil ? "" : String(hString: hedera.prerelease!)
        build = hedera.build == nil ? "" : String(hString: hedera.build!)
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
        try bytes.withUnsafeTypedBytes { pointer in
            var semver = HederaSemanticVersion()

            let err = hedera_semantic_version_from_bytes(pointer.baseAddress, pointer.count, &semver)

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(unsafeFromCHedera: semver)
        }
    }

    public func toBytes() -> Data {
        unsafeWithCHedera { info in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_semantic_version_to_bytes(info, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
        }
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
