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

/// Burns tokens from the token's treasury account.
public final class TokenBurnTransaction: Transaction {
    /// Create a new `TokenBurnTransaction`.
    public init(
        tokenId: TokenId? = nil,
        amount: UInt64 = 0,
        serialNumbers: [UInt64] = []
    ) {
        self.tokenId = tokenId
        self.amount = amount
        self.serialNumbers = serialNumbers
    }

    /// The token for which to burn tokens.
    public var tokenId: TokenId?

    /// Sets the token for which to burn tokens.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The amount of a fungible token to burn from the treasury account.
    public var amount: UInt64

    //// Sets the amount of a fungible token to burn from the treasury account.
    @discardableResult
    public func amount(_ amount: UInt64) -> Self {
        self.amount = amount

        return self
    }

    /// The serial numbers of a non-fungible token to burn from the treasury account.
    public var serialNumbers: [UInt64]

    /// Sets the serial numbers of a non-fungible token to burn from the treasury account.
    @discardableResult
    public func serialNumbers(_ serialNumbers: [UInt64]) -> Self {
        self.serialNumbers = serialNumbers

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case tokenId
        case amount
        case serialNumbers
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(tokenId, forKey: .tokenId)
        try container.encode(amount, forKey: .amount)
        try container.encode(serialNumbers, forKey: .serialNumbers)

        try super.encode(to: encoder)
    }
}
