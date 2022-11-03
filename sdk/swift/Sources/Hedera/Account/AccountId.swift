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
    public let alias: PublicKey?

    public init(shard: UInt64 = 0, realm: UInt64 = 0, alias: PublicKey) {
        self.alias = alias

        super.init(shard: shard, realm: realm, num: 0)
    }

    public required init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        alias = nil

        super.init(shard: shard, realm: realm, num: num)
    }

    public required init?(_ description: String) {
        var shard: UInt64 = 0
        var realm: UInt64 = 0
        var num: UInt64 = 0
        var alias: OpaquePointer?

        let err = hedera_account_id_from_string(description, &shard, &realm, &num, &alias)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        self.alias = alias != nil ? PublicKey.unsafeFromPtr(alias!) : nil

        super.init(shard: shard, realm: realm, num: num)
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
        try bytes.withUnsafeBytes { (pointer: UnsafeRawBufferPointer) in
            var shard: UInt64 = 0
            var realm: UInt64 = 0
            var num: UInt64 = 0
            var alias: OpaquePointer?

            let err = hedera_account_id_from_bytes(
                pointer.baseAddress,
                pointer.count,
                &shard,
                &realm,
                &num,
                &alias
            )

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return Self(shard: shard, realm: realm, num: num)
        }
    }

    public func toBytes() -> Data {
        var buf: UnsafeMutablePointer<UInt8>?
        let size = hedera_account_id_to_bytes(shard, realm, num, alias?.ptr, &buf)

        return Data(bytesNoCopy: buf!, count: size, deallocator: Data.unsafeCHederaBytesFree)
    }
}

// TODO: checksum
// TODO: to evm address
// TODO: hash
// TODO: equals
