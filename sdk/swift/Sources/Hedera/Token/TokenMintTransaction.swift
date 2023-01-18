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

/// Mint tokens to the token's treasury account.
public final class TokenMintTransaction: Transaction {
    /// Create a new `TokenMintTransaction`.
    public init(
        tokenId: TokenId? = nil,
        amount: UInt64 = 0,
        metadata: [Data] = []
    ) {
        self.tokenId = tokenId
        self.amount = amount
        self.metadata = metadata

        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        tokenId = try container.decodeIfPresent(.tokenId)
        amount = try container.decodeIfPresent(.amount) ?? 0
        metadata = try container.decodeIfPresent(.metadata) ?? []

        try super.init(from: decoder)
    }

    /// The token for which to mint tokens.
    public var tokenId: TokenId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token for which to mint tokens.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The amount of a fungible token to mint to the treasury account.
    public var amount: UInt64 {
        willSet {
            ensureNotFrozen()
        }
    }

    //// Sets the amount of a fungible token to mint to the treasury account.
    @discardableResult
    public func amount(_ amount: UInt64) -> Self {
        self.amount = amount

        return self
    }

    /// The list of metadata for a non-fungible token to mint to the treasury account.
    public var metadata: [Data] {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the list of metadata for a non-fungible token to mint to the treasury account.
    @discardableResult
    public func metadata(_ metadata: [Data]) -> Self {
        self.metadata = metadata

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case tokenId
        case amount
        case metadata
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(tokenId, forKey: .tokenId)
        try container.encode(amount, forKey: .amount)
        try container.encode(metadata.map { $0.base64EncodedString() }, forKey: .metadata)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
