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

public class EntityId: LosslessStringConvertible, ExpressibleByIntegerLiteral, Equatable, Codable,
    ExpressibleByStringLiteral, Hashable
{
    /// The shard number (non-negative).
    public let shard: UInt64

    /// The realm number (non-negative).
    public let realm: UInt64

    /// The entity (account, file, contract, token, topic, or schedule) number (non-negative).
    public let num: UInt64

    public required init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        self.shard = shard
        self.realm = realm
        self.num = num
    }

    public required convenience init?(_ description: String) {
        var shard: UInt64 = 0
        var realm: UInt64 = 0
        var num: UInt64 = 0

        let err = hedera_entity_id_from_string(description, &shard, &realm, &num)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        self.init(shard: shard, realm: realm, num: num)
    }

    public required convenience init(integerLiteral value: IntegerLiteralType) {
        self.init(num: UInt64(value))
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public required convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public var description: String {
        "\(shard).\(realm).\(num)"
    }

    public static func == (lhs: EntityId, rhs: EntityId) -> Bool {
        lhs.num == rhs.num && lhs.shard == rhs.shard && lhs.realm == rhs.realm
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(shard)
        hasher.combine(realm)
        hasher.combine(num)
    }
}

// fixme(sr): How do DRY?

/// The unique identifier for a file on Hedera.
public final class FileId: EntityId {
    public static let addressBook: FileId = FileId(num: 102)
    public static let feeSchedule: FileId = FileId(num: 111)
    public static let exchangeRates: FileId = FileId(num: 112)

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var shard: UInt64 = 0
            var realm: UInt64 = 0
            var num: UInt64 = 0

            let err = hedera_file_id_from_bytes(pointer.baseAddress, pointer.count, &shard, &realm, &num)

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(shard: shard, realm: realm, num: num)
        }
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_file_id_to_bytes(shard, realm, num, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
    }
}

/// The unique identifier for a smart contract on Hedera.
public final class ContractId: EntityId {
    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var shard: UInt64 = 0
            var realm: UInt64 = 0
            var num: UInt64 = 0

            let err = hedera_contract_id_from_bytes(pointer.baseAddress, pointer.count, &shard, &realm, &num)

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(shard: shard, realm: realm, num: num)
        }
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_contract_id_to_bytes(shard, realm, num, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
    }
}

/// The unique identifier for a topic on Hedera.
public final class TopicId: EntityId {
    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var shard: UInt64 = 0
            var realm: UInt64 = 0
            var num: UInt64 = 0

            let err = hedera_topic_id_from_bytes(pointer.baseAddress, pointer.count, &shard, &realm, &num)

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(shard: shard, realm: realm, num: num)
        }
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_topic_id_to_bytes(shard, realm, num, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
    }
}

/// The unique identifier for a token on Hedera.
public final class TokenId: EntityId {
    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var shard: UInt64 = 0
            var realm: UInt64 = 0
            var num: UInt64 = 0

            let err = hedera_token_id_from_bytes(pointer.baseAddress, pointer.count, &shard, &realm, &num)

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(shard: shard, realm: realm, num: num)
        }
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_token_id_to_bytes(shard, realm, num, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
    }
}

/// The unique identifier for a schedule on Hedera.
public final class ScheduleId: EntityId {
    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var shard: UInt64 = 0
            var realm: UInt64 = 0
            var num: UInt64 = 0

            let err = hedera_schedule_id_from_bytes(pointer.baseAddress, pointer.count, &shard, &realm, &num)

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(shard: shard, realm: realm, num: num)
        }
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_schedule_id_to_bytes(shard, realm, num, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
    }
}
