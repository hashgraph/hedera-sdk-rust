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

/// Associates the provided account with the provided tokens.
///
/// Must be signed by the provided account's key.
///
public final class TokenAssociateTransaction: Transaction {
    /// Create a new `TokenAssociateTransaction`.
    public init(
        accountId: AccountId? = nil,
        tokenIds: [TokenId] = []
    ) {
        self.accountId = accountId
        self.tokenIds = tokenIds
    }

    /// The account to be associated with the provided tokens.
    public var accountId: AccountId?

    /// Sets the account to be associated with the provided tokens.
    @discardableResult
    public func accountId(_ accountId: AccountId?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The tokens to be associated with the provided account.
    public var tokenIds: [TokenId]

    /// Sets the tokens to be associated with the provided account.
    @discardableResult
    public func tokenIds(_ tokenIds: [TokenId]) -> Self {
        self.tokenIds = tokenIds

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case accountId
        case tokenIds
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(accountId, forKey: .accountId)
        try container.encode(tokenIds, forKey: .tokenIds)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try accountId?.validateChecksums(on: ledgerId)
        try tokenIds.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

}
