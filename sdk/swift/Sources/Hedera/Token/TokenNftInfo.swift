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

/// Response from `TokenNftInfoQuery`.
public final class TokenNftInfo: Codable {
    /// The ID of the NFT.
    public let nftId: NftId

    /// The current owner of the NFT.
    public let accountId: AccountId

    /// Effective consensus timestamp at which the NFT was minted.
    public let creationTime: Timestamp

    /// The unique metadata of the NFT.
    public let metadata: Data

    /// If an allowance is granted for the NFT, its corresponding spender account.
    public let spenderId: AccountId?

    /// The ledger ID the response was returned from
    public let ledgerId: LedgerId

    internal init(
        nftId: NftId,
        accountId: AccountId,
        creationTime: Timestamp,
        metadata: Data,
        spenderId: AccountId?,
        ledgerId: LedgerId
    ) {
        self.nftId = nftId
        self.accountId = accountId
        self.creationTime = creationTime
        self.metadata = metadata
        self.spenderId = spenderId
        self.ledgerId = ledgerId
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension TokenNftInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_TokenNftInfo

    internal convenience init(fromProtobuf proto: Protobuf) throws {
        let spenderId = proto.hasSpenderID ? proto.spenderID : nil

        self.init(
            nftId: .fromProtobuf(proto.nftID),
            accountId: try .fromProtobuf(proto.accountID),
            creationTime: .fromProtobuf(proto.creationTime),
            metadata: proto.metadata,
            spenderId: try .fromProtobuf(spenderId),
            ledgerId: LedgerId(proto.ledgerID)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.nftID = nftId.toProtobuf()
            proto.accountID = accountId.toProtobuf()
            proto.creationTime = creationTime.toProtobuf()
            proto.metadata = metadata

            if let spenderId = spenderId?.toProtobuf() {
                proto.spenderID = spenderId
            }
        }
    }
}
