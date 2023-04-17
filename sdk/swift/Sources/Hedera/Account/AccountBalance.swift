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

private struct TokenBalance {
    fileprivate let id: TokenId
    fileprivate let balance: UInt64
    fileprivate let decimals: UInt32

    internal init(id: TokenId, balance: UInt64, decimals: UInt32) {
        self.id = id
        self.balance = balance
        self.decimals = decimals
    }
}

extension TokenBalance: ProtobufCodable {
    internal typealias Protobuf = Proto_TokenBalance

    internal init(protobuf proto: Protobuf) {
        self.init(id: .fromProtobuf(proto.tokenID), balance: proto.balance, decimals: proto.decimals)
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.tokenID = id.toProtobuf()
            proto.balance = balance
            proto.decimals = decimals
        }
    }
}

/// Response from ``AccountBalanceQuery``.
public struct AccountBalance {
    /// The account that is being referenced.
    public let accountId: AccountId

    /// Current balance of the referenced account.
    public let hbars: Hbar

    private let tokensInner: [TokenBalance]

    // hack to work around deprecated warning
    private var tokenBalancesInner: [TokenId: UInt64] {
        Dictionary(uniqueKeysWithValues: tokensInner.map { ($0.id, $0.balance) })
    }

    /// Token balances for the referenced account.
    ///
    /// This access is *`O(n)`*.
    @available(*, deprecated, message: "use a mirror query")
    public var tokenBalances: [TokenId: UInt64] { tokenBalancesInner }

    // hack to work around deprecated warning
    private var tokenDecimalsInner: [TokenId: UInt32] {
        Dictionary(uniqueKeysWithValues: tokensInner.map { ($0.id, $0.decimals) })
    }

    /// Token decimals for the referenced account.
    ///
    /// This access is *`O(n)`*.
    @available(*, deprecated, message: "use a mirror query")
    public var tokenDecimals: [TokenId: UInt32] { tokenDecimalsInner }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        self.toProtobufBytes()
    }

    public func toString() -> String {
        String(describing: self)
    }
}

extension AccountBalance: TryProtobufCodable {
    internal typealias Protobuf = Proto_CryptoGetAccountBalanceResponse

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            accountId: try .fromProtobuf(proto.accountID),
            hbars: .fromTinybars(Int64(proto.balance)),
            tokensInner: .fromProtobuf(proto.tokenBalances)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.accountID = accountId.toProtobuf()
            proto.balance = UInt64(hbars.toTinybars())
            proto.tokenBalances = tokensInner.toProtobuf()
        }
    }
}
