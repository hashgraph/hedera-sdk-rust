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

public struct Checksum: LosslessStringConvertible, Hashable {
    internal let data: String

    public init?(_ description: String) {
        guard (description.allSatisfy { $0.isASCII && $0.isLowercase && $0.isLetter }) else {
            return nil
        }

        guard description.count == 5 else {
            return nil
        }

        self.data = description
    }

    internal init?(data: Data) {
        guard data.count == 5 else {
            return nil
        }

        let str = String(data: data, encoding: .ascii)!
        // fixme: check for ascii-alphanumeric

        self.data = str
    }

    // swift doesn't have any other way to do "fixed length array"
    // swiftlint:disable:next large_tuple
    internal init(bytes: (UInt8, UInt8, UInt8, UInt8, UInt8)) {
        // swiftlint:disable:next identifier_name
        let (a, b, c, d, e) = bytes
        // fixme: check for ascii-alphanumeric
        self.data = String(data: Data([a, b, c, d, e]), encoding: .ascii)!
    }

    public var description: String {
        data
    }

    internal static func generate<E: EntityId>(for entity: E, on ledgerId: LedgerId) -> Self {
        // todo: fix these
        // swiftlint:disable identifier_name
        // 3 digits in base 26
        let p3 = 26 * 26 * 26
        // 5 digits in base 26
        let p5 = 26 * 26 * 26 * 26 * 26

        // min prime greater than a million. Used for the final permutation.
        let m = 1_000_003

        // Sum s of digit values weights them by powers of W. Should be coprime to P5.
        let w = 31
        // W to the 6th power
        let w6 = w * w * w * w * w * w

        // don't need the six 0 bytes.
        let h = ledgerId.bytes

        let d = entity.description.map { char -> Int in
            if char == "." {
                return 10
            } else {
                return char.wholeNumberValue!
            }
        }

        // Weighted sum of all positions (mod P3)
        var s = 0
        // Sum of even positions (mod 11)
        var s0 = 0
        // Sum of odd positions (mod 11)
        var s1 = 0

        for (index, digit) in d.enumerated() {
            s = (w * s + digit) % p3
            if index.isOdd {
                s1 += digit
            } else {
                s0 += digit
            }
        }

        s0 = s0 % 11
        s1 = s1 % 11

        // instead of six 0 bytes, we compute this in two steps
        var sh = h.reduce(0) { (result, value) in (w * result + Int(value)) % p5 }
        // `(w * result + Int(0)) % p5` applied 6 times...
        // `(w * result + Int(0)) % p5 = (w * result) % p5` because 0 is the additive identity
        // then expanding out the full expression:
        // `((w * ((w * ((w * ((w * ((w * ((w * result) % p5)) % p5)) % p5)) % p5)) % p5)) % p5)`
        // ... and using the fact that `((x % y) * z) % y = (x * z) % y`
        // we get:
        sh = (sh * w6) % p5

        // original expression:
        // var c = ((((((entityIdString.count % 5) * 11 + s0) * 11 + s1) * p3 + s + sh) % p5) * m) % p5
        // but `((x % y) * z) % y = ((x * z) % y) % y = (x * z) % y`
        // checksum as a single number
        var c = (((((d.count % 5) * 11 + s0) * 11 + s1) * p3 + s + sh) * m) % p5

        var output: [UInt8] = [0, 0, 0, 0, 0]

        for i in (0..<5).reversed() {
            output[i] = UInt8(0x61 + c % 26)
            c /= 26
        }

        // thanks swift, for not having fixed length arrays
        return Checksum(bytes: (output[0], output[1], output[2], output[3], output[4]))

        // swiftlint:endable identifier_name
    }
}

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

            let last: S
            let checksum: Checksum?
            switch rest.splitOnce(on: "-") {
            case .some((let value, let cs)):
                last = value
                if let cs = Checksum(String(cs)) {
                    checksum = cs
                } else {
                    throw HError(kind: .basicParse, description: "Invalid checksum string \(cs)")
                }

            case .none:
                last = rest
                checksum = nil
            }

            self = .long(
                shard: try UInt64(parsing: shard),
                realm: try UInt64(parsing: realm),
                last: last,
                checksum: checksum
            )

        case .none:
            if let num = UInt64(description) {
                self = .short(num: num)
            } else {
                self = .other(description[...])
            }
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

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try helper.validateChecksum(on: ledgerId)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension FileId: ProtobufCodable {
    internal typealias Protobuf = HederaProtobufs.Proto_FileID

    internal init(fromProtobuf proto: Protobuf) {
        self.init(
            shard: UInt64(proto.shardNum),
            realm: UInt64(proto.realmNum),
            num: UInt64(proto.fileNum)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.shardNum = Int64(shard)
            proto.realmNum = Int64(realm)
            proto.fileNum = Int64(num)
        }
    }
}

/// The unique identifier for a topic on Hedera.
public struct TopicId: EntityId, ValidateChecksums {
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

    /// The topic number.
    public let num: UInt64

    public let checksum: Checksum?

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try helper.validateChecksum(on: ledgerId)
    }
}

extension TopicId: ProtobufCodable {
    internal typealias Protobuf = HederaProtobufs.Proto_TopicID

    internal init(fromProtobuf proto: Protobuf) {
        self.init(
            shard: UInt64(proto.shardNum),
            realm: UInt64(proto.realmNum),
            num: UInt64(proto.topicNum)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.shardNum = Int64(shard)
            proto.realmNum = Int64(realm)
            proto.topicNum = Int64(num)
        }
    }
}

/// The unique identifier for a token on Hedera.
public struct TokenId: EntityId, ValidateChecksums {
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

    /// The token number.
    public let num: UInt64

    public let checksum: Checksum?

    public func nft(_ serial: UInt64) -> NftId {
        NftId(tokenId: self, serial: serial)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try helper.validateChecksum(on: ledgerId)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension TokenId: ProtobufCodable {
    internal typealias Protobuf = HederaProtobufs.Proto_TokenID

    internal init(fromProtobuf proto: Protobuf) {
        self.init(
            shard: UInt64(proto.shardNum),
            realm: UInt64(proto.realmNum),
            num: UInt64(proto.tokenNum)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.shardNum = Int64(shard)
            proto.realmNum = Int64(realm)
            proto.tokenNum = Int64(num)
        }
    }
}

/// The unique identifier for a schedule on Hedera.
public struct ScheduleId: EntityId, ValidateChecksums {
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

    /// The token number.
    public let num: UInt64

    public let checksum: Checksum?

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try helper.validateChecksum(on: ledgerId)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension ScheduleId: ProtobufCodable {
    internal typealias Protobuf = Proto_ScheduleID

    internal init(fromProtobuf proto: Protobuf) {
        self.init(
            shard: UInt64(proto.shardNum),
            realm: UInt64(proto.realmNum),
            num: UInt64(proto.scheduleNum)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.shardNum = Int64(shard)
            proto.realmNum = Int64(realm)
            proto.scheduleNum = Int64(num)
        }
    }
}
