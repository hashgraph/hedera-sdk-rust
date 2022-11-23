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

/// The unique identifier for a cryptocurrency account on Hedera.
public final class AccountId: EntityId {
    public override func hash(into hasher: inout Hasher) {
        super.hash(into: &hasher)
        hasher.combine(alias)
    }

    public let alias: PublicKey?

    public init(shard: UInt64 = 0, realm: UInt64 = 0, alias: PublicKey) {
        self.alias = alias

        super.init(shard: shard, realm: realm, num: 0)
    }

    public required init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        alias = nil

        super.init(shard: shard, realm: realm, num: num)
    }

    private convenience init(parsing description: String) throws {
        var id = HederaAccountId()

        try HError.throwing(error: hedera_account_id_from_string(description, &id))

        self.init(unsafeFromCHedera: id)
    }

    public required convenience init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public required convenience init(integerLiteral value: IntegerLiteralType) {
        self.init(num: UInt64(value))
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    public required convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    internal init(unsafeFromCHedera hedera: HederaAccountId) {
        alias = hedera.alias.map(PublicKey.unsafeFromPtr)
        super.init(shard: hedera.shard, realm: hedera.realm, num: hedera.num)
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaAccountId) throws -> Result) rethrows -> Result {
        try body(HederaAccountId(shard: shard, realm: realm, num: num, alias: alias?.ptr))
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public override var description: String {
        if let alias = alias {
            return "\(shard).\(realm).\(alias)"
        }

        return super.description
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var id = HederaAccountId()

            try HError.throwing(error: hedera_account_id_from_bytes(pointer.baseAddress, pointer.count, &id))

            return Self(unsafeFromCHedera: id)
        }
    }

    public func toBytes() -> Data {
        self.unsafeWithCHedera { (hedera) in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_account_id_to_bytes(hedera, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
        }
    }

    public static func == (lhs: AccountId, rhs: AccountId) -> Bool {
        lhs.shard == rhs.shard && lhs.realm == rhs.realm && lhs.num == lhs.num && lhs.alias == rhs.alias
    }
}

// TODO: checksum
// TODO: to evm address
