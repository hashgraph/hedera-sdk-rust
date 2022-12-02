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

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }

    public func nft(_ serial: UInt64) -> NftId {
        NftId(tokenId: self, serial: serial)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try helper.validateChecksum(on: ledgerId)
    }
}

extension TokenId: ProtobufCodable {
    internal typealias Protobuf = HederaProtobufs.Proto_TokenID

    internal init(protobuf proto: Protobuf) {
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
