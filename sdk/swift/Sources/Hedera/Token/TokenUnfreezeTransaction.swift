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

/// Unfreezes transfers of the specified token for the account.
public final class TokenUnfreezeTransaction: Transaction {
    /// Create a new `TokenUnfreezeTransaction`.
    public init(
        accountId: AccountId? = nil,
        tokenId: TokenId? = nil
    ) {
        self.accountId = accountId
        self.tokenId = tokenId
    }

    /// The account to be unfrozen.
    public var accountId: AccountId?

    /// Sets the account to be unfrozen.
    @discardableResult
    public func accountId(_ accountId: AccountId?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The token for which this account will be unfrozen.
    public var tokenId: TokenId?

    /// Sets the token for which this account will be unfrozen.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case accountId
        case tokenId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(accountId, forKey: .accountId)
        try container.encode(tokenId, forKey: .tokenId)

        try super.encode(to: encoder)
    }
}
