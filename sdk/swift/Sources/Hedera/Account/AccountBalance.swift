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

private struct TokenBalance: Codable {
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

    internal init(fromProtobuf proto: Protobuf) {
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

extension Array where Element == TokenBalance {
    fileprivate static func from(balances: [TokenId: UInt64], decimals: [TokenId: UInt32]) -> Self {
        precondition(balances.count == decimals.count)

        var list: Self = []

        for (id, balance) in balances {
            let decimals = decimals[id]!
            list.append(TokenBalance(id: id, balance: balance, decimals: decimals))
        }

        return list
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
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        self.toProtobufBytes()
    }

    public func toString() -> String {
        String(describing: self)
    }
}

extension AccountBalance: Codable {
    public enum CodingKeys: CodingKey {
        case accountId
        case hbars
        case tokenBalances
        case tokenDecimals
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        let accountId = try container.decode(AccountId.self, forKey: .accountId)
        let hbars = try container.decode(Hbar.self, forKey: .hbars)
        let tokenBalances = try container.decodeIfPresent([TokenId: UInt64].self, forKey: .tokenBalances) ?? [:]
        let tokenDecimals = try container.decodeIfPresent([TokenId: UInt32].self, forKey: .tokenDecimals) ?? [:]

        self.init(
            accountId: accountId,
            hbars: hbars,
            tokensInner: [TokenBalance].from(balances: tokenBalances, decimals: tokenDecimals)
        )
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(accountId, forKey: .accountId)
        try container.encode(hbars, forKey: .hbars)
        try container.encode(tokenBalancesInner, forKey: .tokenBalances)
        try container.encode(tokenDecimalsInner, forKey: .tokenDecimals)
    }
}

extension AccountBalance: TryProtobufCodable {
    internal typealias Protobuf = Proto_CryptoGetAccountBalanceResponse

    internal init(fromProtobuf proto: Protobuf) throws {
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
