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
import HederaProtobufs

/// The unique identifier for a cryptocurrency account on Hedera.
public struct AccountId: EntityId, ValidateChecksums {
    public let shard: UInt64
    public let realm: UInt64
    public let num: UInt64
    public let checksum: Checksum?
    public let alias: PublicKey?
    public let evmAddress: EvmAddress?

    public init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64, checksum: Checksum?) {
        self.shard = shard
        self.realm = realm
        self.num = num
        alias = nil
        self.checksum = checksum
        evmAddress = nil
    }

    public init(shard: UInt64 = 0, realm: UInt64 = 0, alias: PublicKey) {
        self.shard = shard
        self.realm = realm
        num = 0
        self.alias = alias
        checksum = nil
        evmAddress = nil
    }

    public init(shard: UInt64 = 0, realm: UInt64 = 0, num: UInt64) {
        self.init(shard: shard, realm: realm, num: num, checksum: nil)
    }

    public init(evmAddress: EvmAddress) {
        shard = 0
        realm = 0
        num = 0
        alias = nil
        self.evmAddress = evmAddress
        checksum = nil
    }

    public init<S: StringProtocol>(parsing description: S) throws {
        switch try PartialEntityId<S.SubSequence>(parsing: description) {
        case .short(let num):
            self.init(num: num)

        case .long(let shard, let realm, let last, let checksum):
            if let num = UInt64(last) {
                self.init(shard: shard, realm: realm, num: num, checksum: checksum)
            } else {
                guard checksum == nil else {
                    throw HError(
                        kind: .basicParse, description: "checksum not supported with `<shard>.<realm>.<alias>`")
                }

                // might have `alias`
                self.init(
                    shard: shard,
                    realm: realm,
                    alias: try PublicKey.fromString(String(last))
                )
            }

        case .other(let description):
            let evmAddress = try EvmAddress(parsing: description)
            self.init(evmAddress: evmAddress)
        }
    }

    internal init(unsafeFromCHedera hedera: HederaAccountId) {
        shard = hedera.shard
        realm = hedera.realm
        num = hedera.num
        alias = hedera.alias.map(PublicKey.unsafeFromPtr)
        checksum = nil
        evmAddress = hedera.evm_address.map { ptr in
            // swiftlint:disable:next force_try
            try! EvmAddress(Data(bytesNoCopy: ptr, count: 20, deallocator: .unsafeCHederaBytesFree))
        }
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaAccountId) throws -> Result) rethrows -> Result {
        if let evmAddress = evmAddress {
            return try evmAddress.data.withUnsafeTypedBytes { evmAddress in
                let evmAddress = UnsafeMutablePointer(mutating: evmAddress.baseAddress)
                return try body(
                    HederaAccountId(shard: shard, realm: realm, num: num, alias: alias?.ptr, evm_address: evmAddress))
            }
        }

        return try body(HederaAccountId(shard: shard, realm: realm, num: num, alias: alias?.ptr, evm_address: nil))
    }

    public var description: String {
        if let alias = alias {
            return "\(shard).\(realm).\(alias)"
        }

        return helper.description
    }

    public func toStringWithChecksum(_ client: Client) -> String {
        precondition(alias == nil, "cannot create a checksum for an `AccountId` with an alias")
        precondition(evmAddress == nil, "cannot create a checksum for an `AccountId` with an evmAddress")

        return helper.toStringWithChecksum(client)
    }

    public func validateChecksum(_ client: Client) throws {
        try validateChecksums(on: client.ledgerId!)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        if alias != nil || evmAddress != nil {
            return
        }

        try helper.validateChecksum(on: ledgerId)
    }

    /// Create an `AccountId` from an evm address.
    ///
    /// Accepts an Ethereum public address.
    public static func fromEvmAddress(_ evmAddress: EvmAddress) -> Self {
        Self(evmAddress: evmAddress)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }

    // todo: public func `toEvmAddress`/`getEvmAddress`
}

extension AccountId: TryProtobufCodable {
    internal typealias Protobuf = Proto_AccountID

    internal init(fromProtobuf proto: Protobuf) throws {
        let shard = UInt64(proto.shardNum)
        let realm = UInt64(proto.realmNum)

        if proto.alias.isEmpty {
            if proto.evmAddress.isEmpty {
                self.init(shard: shard, realm: realm, num: UInt64(proto.accountNum))
            } else {
                let evmAddress = try EvmAddress(proto.evmAddress)
                self.init(evmAddress: evmAddress)
            }
        } else {
            self.init(shard: shard, realm: realm, alias: try PublicKey(fromProtobufBytes: proto.alias))
        }
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            if let evmAddress = evmAddress {
                proto.evmAddress = evmAddress.data
                return
            }

            proto.shardNum = Int64(shard)
            proto.realmNum = Int64(realm)

            if let alias = alias {
                proto.alias = alias.toProtobufBytes()
            } else {
                proto.accountNum = Int64(num)
            }
        }
    }
}
