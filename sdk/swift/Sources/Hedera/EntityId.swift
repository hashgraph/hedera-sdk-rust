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

public protocol EntityId: LosslessStringConvertible, ExpressibleByIntegerLiteral, Codable,
    ExpressibleByStringLiteral, Hashable
where
    Self.IntegerLiteralType == UInt64,
    Self.StringLiteralType == String
{
    /// The shard number (non-negative).
    var shard: UInt64 { get }

    /// The realm number (non-negative).
    var realm: UInt64 { get }

    /// The entity (account, file, contract, token, topic, or schedule) number (non-negative).
    var num: UInt64 { get }

    /// The checksum for this entity ID with respect to *some* ledger ID.
    var checksum: Checksum? { get }

    /// Create an entity ID from the given entity number.
    ///
    /// - Parameters:
    ///   - num: the number for the new entity ID.
    init(num: UInt64)

    init(shard: UInt64, realm: UInt64, num: UInt64)

    init(shard: UInt64, realm: UInt64, num: UInt64, checksum: Checksum?)

    /// Parse an entity ID from a string.
    init<S: StringProtocol>(parsing description: S) throws

    /// Parse an entity ID from a string.
    static func fromString<S: StringProtocol>(_ description: S) throws -> Self

    /// Parse an entity ID from the given `bytes`.
    static func fromBytes(_ bytes: Data) throws -> Self

    /// Convert this entity ID to bytes.
    func toBytes() -> Data

    func toString() -> String

    func toStringWithChecksum(_ client: Client) -> String

    func validateChecksum(_ client: Client) throws

    /// Create `Self` from a solidity `address`.
    static func fromSolidityAddress<S: StringProtocol>(_ description: S) throws -> Self

    /// Convert `self` into a solidity `address`
    func toSolidityAddress() throws -> String
}

extension EntityId {
    internal typealias Helper = EntityIdHelper<Self>

    internal var helper: Helper { Helper(self) }

    public init(integerLiteral value: IntegerLiteralType) {
        self.init(num: value)
    }

    public init(num: UInt64) {
        self.init(shard: 0, realm: 0, num: num)
    }

    public init<S: StringProtocol>(parsing description: S) throws {
        self = try PartialEntityId<S.SubSequence>(parsing: description).intoNum()
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public init(stringLiteral value: StringLiteralType) {
        // Force try here because this is a logic error.
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    public static func fromString<S: StringProtocol>(_ description: S) throws -> Self {
        try Self(parsing: description)
    }

    public var description: String { helper.description }

    public init(from decoder: Decoder) throws {
        try self.init(parsing: decoder.singleValueContainer().decode(String.self))
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public static func fromSolidityAddress<S: StringProtocol>(_ description: S) throws -> Self {
        try SolidityAddress(parsing: description).toEntityId()
    }

    public func toString() -> String {
        self.description
    }

    internal func generateChecksum(for ledgerId: LedgerId) -> Checksum {
        Checksum.generate(for: self, on: ledgerId)
    }

    public func toStringWithChecksum(_ client: Client) -> String {
        helper.toStringWithChecksum(client)
    }

    public func validateChecksum(_ client: Client) throws {
        try helper.validateChecksum(on: client)
    }

    public func toSolidityAddress() throws -> String {
        try String(describing: SolidityAddress(self))
    }
}

// this exists purely for convinence purposes lol.
internal struct EntityIdHelper<E: EntityId> {
    internal init(_ id: E) {
        self.id = id
    }

    private let id: E

    internal var description: String {
        "\(id.shard).\(id.realm).\(id.num)"
    }

    // note: this *expicitly* ignores the current checksum.
    internal func toStringWithChecksum(_ client: Client) -> String {
        let checksum = id.generateChecksum(for: client.ledgerId!)
        return "\(description)-\(checksum)"
    }

    internal func validateChecksum(on ledgerId: LedgerId) throws {
        if let checksum = id.checksum {
            let expected = id.generateChecksum(for: ledgerId)
            if checksum != expected {
                throw HError(
                    kind: .badEntityId, description: "expected entity id `\(id)` to have checksum `\(expected)`")
            }
        }
    }

    internal func validateChecksum(on client: Client) throws {
        try validateChecksum(on: client.ledgerId!)
    }
}

internal enum PartialEntityId<S> {
    // entity ID in the form `<num>`
    case short(num: UInt64)
    // entity ID in the form `<shard>.<realm>.<last>`
    case long(shard: UInt64, realm: UInt64, last: S, checksum: Checksum?)
    // entity ID in some other format (for example `0x<evmAddress>`)
    case other(S)

    internal init<D: StringProtocol>(parsing description: D) throws where S == D.SubSequence {
        switch description.splitOnce(on: ".") {
        case .some((let shard, let rest)):
            // `shard.realm.num` format
            guard let (realm, rest) = rest.splitOnce(on: ".") else {
                throw HError(
                    kind: .basicParse, description: "expected `<shard>.<realm>.<num>` or `<num>`, got, \(description)")
            }

            let (last, checksum) = try rest.splitOnce(on: "-").map { ($0, try Checksum(parsing: $1)) } ?? (rest, nil)

            self = .long(
                shard: try UInt64(parsing: shard),
                realm: try UInt64(parsing: realm),
                last: last,
                checksum: checksum
            )

        case .none:
            self = UInt64(description).map(Self.short) ?? .other(description[...])
        }
    }

    internal func intoNum<E: EntityId>() throws -> E where S: StringProtocol {
        switch self {
        case .short(let num):
            return E(num: num)
        case .long(let shard, let realm, last: let num, let checksum):
            return E(shard: shard, realm: realm, num: try UInt64(parsing: num), checksum: checksum)
        case .other(let description):
            throw HError(
                kind: .basicParse, description: "expected `<shard>.<realm>.<num>` or `<num>`, got, \(description)")
        }
    }
}

// fixme(sr): How do DRY?

/// The unique identifier for a file on Hedera.
public struct FileId: EntityId, ValidateChecksums {
    public init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64, checksum: Checksum?) {
        self.shard = shard
        self.realm = realm
        self.num = num
        self.checksum = checksum
    }

    public init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        self.init(shard: shard, realm: realm, num: num, checksum: nil)
    }

    public let shard: UInt64
    public let realm: UInt64

    /// The file number.
    public let num: UInt64

    public let checksum: Checksum?

    public static let addressBook: FileId = 102
    public static let feeSchedule: FileId = 111
    public static let exchangeRates: FileId = 112

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var shard: UInt64 = 0
            var realm: UInt64 = 0
            var num: UInt64 = 0

            try HError.throwing(
                error: hedera_file_id_from_bytes(pointer.baseAddress, pointer.count, &shard, &realm, &num))

            return Self(shard: shard, realm: realm, num: num)
        }
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_file_id_to_bytes(shard, realm, num, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try helper.validateChecksum(on: ledgerId)
    }
}
