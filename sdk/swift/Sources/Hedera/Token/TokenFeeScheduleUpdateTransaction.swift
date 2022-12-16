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

/// At consensus, updates a token type's fee schedule to the given list of custom fees.
public final class TokenFeeScheduleUpdateTransaction: Transaction {
    /// Create a new `TokenFeeScheduleUpdateTransaction`.
    public init(
        tokenId: TokenId? = nil,
        customFees: [AnyCustomFee] = []
    ) {
        self.tokenId = tokenId
        self.customFees = customFees
    }

    /// The token whose fee schedule is to be updated.
    public var tokenId: TokenId?

    /// Sets the token whose fee schedule is to be updated.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The new custom fees to be assessed during a transfer.
    public var customFees: [AnyCustomFee]

    /// Sets the new custom fees to be assessed during a transfer.
    @discardableResult
    public func customFees(_ customFees: [AnyCustomFee]) -> Self {
        self.customFees = customFees

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case tokenId
        case customFees
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(tokenId, forKey: .tokenId)
        try container.encode(customFees, forKey: .customFees)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)
        try customFees.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

}
