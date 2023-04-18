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

import Foundation
import HederaProtobufs

/// Hedera follows semantic versioning for both the HAPI protobufs and
/// the Services software.
public struct SemanticVersion: ExpressibleByStringLiteral, LosslessStringConvertible {
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
    // internal API, do NOT expose.
    private init<S: StringProtocol>(parsing description: S) throws {
        let parts = description.split(separator: ".", maxSplits: 2)

        guard parts.count == 3 else {
            throw HError.basicParse("expected major.minor.patch for semver")
        }

        let patchStr: S.SubSequence
        let pre: S.SubSequence?
        let buildStr: S.SubSequence?

        do {
            // while it seems like this is a weird order,
            // it actually makes more sense than `pre` first,
            // because `pre` is in the middle of the string.
            var tmp = parts[2]
            (tmp, buildStr) = tmp.splitOnce(on: "+") ?? (tmp, nil)

            (patchStr, pre) = tmp.splitOnce(on: "-") ?? (tmp, nil)
        }

        let major = try parseVersion(parts[0], section: "major")
        let minor = try parseVersion(parts[1], section: "minor")
        let patch = try parseVersion(patchStr, section: "patch")

        let prerelease = try pre.map { String(try parsePrerelease($0)) } ?? ""
        let build = try buildStr.map { String(try parseBuild($0)) } ?? ""

        self.init(major: major, minor: minor, patch: patch, prerelease: prerelease, build: build)
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

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
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

    internal init(protobuf proto: Protobuf) {
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

extension Character {
    fileprivate var isASCIIAlphamumeric: Bool {
        isASCII && (isLetter || isNumber)
    }

    fileprivate var isASCIIDigit: Bool {
        isASCII && isNumber
    }

    fileprivate var isValidIdent: Bool {
        isASCIIAlphamumeric || self == "-"
    }
}

extension StringProtocol {
    fileprivate var isNumericWithLeadingZero: Bool {
        guard let rest = stripPrefix("0") else {
            return false
        }

        return !rest.isEmpty && rest.allSatisfy { $0.isASCIIDigit }
    }
}

private func parseVersion<S: StringProtocol>(_ string: S, section: String) throws -> UInt32 {
    if string.isNumericWithLeadingZero {
        throw HError.basicParse("semver section `\(section)` starts with leading 0: `\(string)`")
    }

    return try UInt32(parsing: string)
}

private func parsePrerelease<S: StringProtocol>(_ string: S) throws -> S {
    if string.isEmpty {
        throw HError.basicParse("semver with empty prerelease")
    }

    for identifier in string.split(separator: ".") {
        if identifier.isEmpty {
            throw HError.basicParse("semver with empty -pre identifier")
        }

        if !(identifier.allSatisfy { $0.isValidIdent }) {
            throw HError.basicParse("semver with invalid identifier for the -pre section: `\(identifier)`")
        }

        if identifier.isNumericWithLeadingZero {
            throw HError.basicParse("numeric pre-release identifier has leading zero: `\(identifier)`")
        }
    }

    return string
}

private func parseBuild<S: StringProtocol>(_ string: S) throws -> S {
    if string.isEmpty {
        throw HError.basicParse("semver with empty build")
    }

    for identifier in string.split(separator: ".") {
        if identifier.isEmpty {
            throw HError.basicParse("semver with empty build section identifier")
        }

        if !(identifier.allSatisfy { $0.isValidIdent }) {
            throw HError.basicParse("semver with invalid identifier for the build section: `\(identifier)`")
        }
    }

    return string
}
