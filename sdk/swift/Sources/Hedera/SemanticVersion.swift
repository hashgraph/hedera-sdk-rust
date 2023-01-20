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
        try Self(parsing: description)
    }

    public init(stringLiteral value: StringLiteralType) {
        // swiftlint:allow:next force_try
        try! self.init(parsing: value)
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
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

            try HError.throwing(error: hedera_semantic_version_from_bytes(pointer.baseAddress, pointer.count, &semver))

            return Self(unsafeFromCHedera: semver)
        }
    }

    public func toBytes() -> Data {
        unsafeWithCHedera { info in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_semantic_version_to_bytes(info, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
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

private func isValidIdentChar(_ ch: Character) -> Bool {
    (ch.isASCII && (ch.isLetter || ch.isNumber)) || ch == "-"
}

private func isNumericWithLeadingZero<S: StringProtocol>(_ string: S) -> Bool {
    guard let string = string.stripPrefix("0") else {
        return false
    }

    if string.isEmpty {
        return false
    }

    return string.allSatisfy { char in char.isASCII && char.isNumber }
}

private func parseVersion<S: StringProtocol>(_ string: S, section: StaticString) throws -> UInt32 {
    if isNumericWithLeadingZero(string) {
        throw HError.basicParse("semver section `\(section)` starts with leading 0: `\(string)`")
    }

    return try UInt32(parsing: string)
}

private func parsePrerelease<S: StringProtocol>(_ string: S) throws -> S {
    if string.isEmpty {
        throw HError.basicParse("semver with empty prerelease")
    }

    for identifier: S.SubSequence in string.split(separator: ".") {
        if identifier.isEmpty {
            throw HError.basicParse("semver with empty -pre identifier")
        }

        if !identifier.allSatisfy(isValidIdentChar) {
            throw HError.basicParse("semver with invalid identifier for the -pre section: `\(identifier)`")
        }

        if isNumericWithLeadingZero(string) {
            throw HError.basicParse("numeric pre-release identifier has leading zero: `\(identifier)`")
        }
    }

    return string
}

private func parseBuild<S: StringProtocol>(_ string: S) throws -> S {
    if string.isEmpty {
        throw HError.basicParse("semver with empty build")
    }

    for identifier: S.SubSequence in string.split(separator: ".") {
        if identifier.isEmpty {
            throw HError.basicParse("semver with empty build-section identifier")
        }

        if !identifier.allSatisfy(isValidIdentChar) {
            throw HError.basicParse("semver with invalid identifier for the build section: `\(identifier)`")
        }
    }

    return string
}

extension SemanticVersion {
    // its probably useless doing strict parsing when Hedera probably accepts loose parsing anyway, but lets at least *try* not to make it worse.
    fileprivate init<S: StringProtocol>(parsing description: S) throws {
        let parts = description.split(separator: ".", maxSplits: 3)

        guard parts.count == 3 else {
            throw HError.basicParse("expected `major.minor.patch` for semver")
        }

        let majorStr = parts[0]
        let minorStr = parts[1]
        var rest = parts[2]
        let buildStr: S.SubSequence?

        // while it seems like this is a weird order,
        // it actually makes more sense than `pre` first,
        // because `pre` is in the middle of the string.
        (rest, buildStr) = rest.splitOnce(on: "+") ?? (rest, nil)

        let (patchStr, preStr) = rest.splitOnce(on: "-") ?? (rest, nil)

        let major = try parseVersion(majorStr, section: "major")
        let minor = try parseVersion(minorStr, section: "minor")
        let patch = try parseVersion(patchStr, section: "patch")
        let pre = try preStr.map(parsePrerelease) ?? ""
        let build = try buildStr.map(parseBuild) ?? ""

        self.init(
            major: major,
            minor: minor,
            patch: patch,
            prerelease: String(pre),
            build: String(build)
        )
    }
}
