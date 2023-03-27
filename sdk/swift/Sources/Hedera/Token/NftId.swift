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

/// The unique identifier for a non-fungible token (NFT) instance on Hedera.
public struct NftId: LosslessStringConvertible, ExpressibleByStringLiteral, Equatable, ValidateChecksums {
    /// The (non-fungible) token of which this NFT is an instance.
    public let tokenId: TokenId

    /// The unique identifier for this instance.
    public let serial: UInt64

    /// Create a new `NftId` from the passed `tokenId` and `serial`.
    public init(tokenId: TokenId, serial: UInt64) {
        self.tokenId = tokenId
        self.serial = serial
    }

    private init<S: StringProtocol>(parsing description: S) throws {
        guard let (tokenId, serial) = description.rsplitOnce(on: "/") ?? description.rsplitOnce(on: "@") else {
            throw HError(
                kind: .basicParse,
                description: "unexpected NftId format - expected [tokenId]/[serial] or [tokenId]@[serial]"
            )
        }

        self.tokenId = try .fromString(tokenId)
        self.serial = try UInt64(parsing: serial)
    }

    public static func fromString(_ description: String) throws -> Self {
        try self.init(parsing: description)
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    public var description: String {
        "\(tokenId)/\(serial)"
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId.validateChecksums(on: ledgerId)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        self.toProtobufBytes()
    }
}

extension NftId: ProtobufCodable {
    internal typealias Protobuf = Proto_NftID

    internal init(protobuf proto: Protobuf) {
        self.init(
            tokenId: .fromProtobuf(proto.tokenID),
            serial: UInt64(proto.serialNumber)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.tokenID = tokenId.toProtobuf()
            proto.serialNumber = Int64(serial)
        }
    }
}
