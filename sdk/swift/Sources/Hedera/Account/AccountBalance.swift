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

private struct TokenBalance: Codable {
    fileprivate let id: TokenId
    fileprivate let balance: UInt64
    fileprivate let decimals: UInt32

    internal init(id: TokenId, balance: UInt64, decimals: UInt32) {
        self.id = id
        self.balance = balance
        self.decimals = decimals
    }

    fileprivate init(fromCHedera hedera: HederaTokenBalance) {
        id = TokenId(shard: hedera.id_shard, realm: hedera.id_realm, num: hedera.id_num)
        balance = hedera.amount
        decimals = hedera.decimals
    }

    fileprivate func toCHedera() -> HederaTokenBalance {
        HederaTokenBalance(
            id_shard: id.shard,
            id_realm: id.realm,
            id_num: id.num,
            amount: balance,
            decimals: decimals
        )
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
        try bytes.withUnsafeTypedBytes { pointer in
            var balance = HederaAccountBalance()

            try HError.throwing(error: hedera_account_balance_from_bytes(pointer.baseAddress, pointer.count, &balance))

            return Self(unsafeFromCHedera: balance)
        }
    }

    public func toBytes() -> Data {
        self.unsafeWithCHedera { hedera in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_account_balance_to_bytes(hedera, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
        }
    }

    public func toString() -> String {
        String(describing: self)
    }
}

extension AccountBalance: Codable {
    public enum CodingKeys: CodingKey {
        case accountId
        case hbars
        case tokens
        case tokenDecimals
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        let accountId = try container.decode(AccountId.self, forKey: .accountId)
        let hbars = try container.decode(Hbar.self, forKey: .hbars)
        // hack around SE0320
        let tokenBalances: [TokenId: UInt64]
        do {
            let tokenBalancesStrings = try container.decodeIfPresent([String: UInt64].self, forKey: .tokens) ?? [:]
            tokenBalances = Dictionary(
                uniqueKeysWithValues: try tokenBalancesStrings.map { key, value in
                    (try TokenId.fromString(key), value)
                }
            )
        }

        let tokenDecimals: [TokenId: UInt32]

        do {
            let tokenDecimalStrings =
                try (container.decodeIfPresent([String: UInt32].self, forKey: .tokenDecimals) ?? [:])
            tokenDecimals = Dictionary(
                uniqueKeysWithValues: try tokenDecimalStrings.map { key, value in
                    (try TokenId.fromString(key), value)
                }
            )
        }

        self.init(
            accountId: accountId,
            hbars: hbars,
            tokensInner: .from(balances: tokenBalances, decimals: tokenDecimals)
        )
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(accountId, forKey: .accountId)
        try container.encode(hbars, forKey: .hbars)
        try container.encode(tokenBalancesInner, forKey: .tokens)
        try container.encode(tokenDecimalsInner, forKey: .tokenDecimals)
    }
}

extension AccountBalance {
    internal init(unsafeFromCHedera hedera: HederaAccountBalance) {
        accountId = AccountId(unsafeFromCHedera: hedera.id)
        hbars = Hbar.fromTinybars(hedera.hbars)

        tokensInner = UnsafeBufferPointer(start: hedera.token_balances, count: hedera.token_balances_len)
            .map(TokenBalance.init(fromCHedera:))

        hedera_account_balance_token_balances_free(
            UnsafeMutablePointer(mutating: hedera.token_balances),
            hedera.token_balances_len
        )
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaAccountBalance) throws -> Result) rethrows -> Result {
        try accountId.unsafeWithCHedera { hederaAccountId in
            try tokensInner
                .map { $0.toCHedera() }
                .withUnsafeBufferPointer { buffer in
                    try body(
                        HederaAccountBalance(
                            id: hederaAccountId,
                            hbars: hbars.toTinybars(),
                            token_balances: buffer.baseAddress,
                            token_balances_len: buffer.count
                        )
                    )
                }
        }
    }
}
