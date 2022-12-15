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
public struct AccountId: EntityId {
    public let shard: UInt64
    public let realm: UInt64
    public let num: UInt64
    public let alias: PublicKey?

    public init(shard: UInt64 = 0, realm: UInt64 = 0, alias: PublicKey) {
        self.shard = shard
        self.realm = realm
        num = 0
        self.alias = alias
    }

    public init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        self.shard = shard
        self.realm = realm
        self.num = num
        alias = nil
    }

    public init<S: StringProtocol>(parsing description: S) throws {
        switch try PartialEntityId<S.SubSequence>(parsing: description) {
        case .short(let num):
            self.init(num: num)

        case .long(let shard, let realm, let last):
            if let num = UInt64(last) {
                self.init(shard: shard, realm: realm, num: num)
            } else {
                // might have `evmAddress`
                self.init(
                    shard: shard,
                    realm: realm,
                    alias: try PublicKey.fromString(String(last))
                )
            }

        // check for `evmAddress` eventually
        case .other(let description):
            throw HError(
                kind: .basicParse, description: "expected `<shard>.<realm>.<num>` or `<num>`, got, \(description)")
        }
    }

    internal init(unsafeFromCHedera hedera: HederaAccountId) {
        shard = hedera.shard
        realm = hedera.realm
        num = hedera.num
        alias = hedera.alias.map(PublicKey.unsafeFromPtr)
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaAccountId) throws -> Result) rethrows -> Result {
        try body(HederaAccountId(shard: shard, realm: realm, num: num, alias: alias?.ptr))
    }

    public var description: String {
        if let alias = alias {
            return "\(shard).\(realm).\(alias)"
        }

        return defaultDescription
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
