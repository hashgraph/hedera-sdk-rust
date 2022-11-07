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

/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than `AccountInfoQuery`, which returns the balance plus
/// additional information.
public final class AccountBalanceQuery: Query<AccountBalance> {
    /// Create a new `AccountBalanceQuery`.
    public init(
        accountId: AccountId? = nil,
        contractId: ContractId? = nil
    ) {
        self.accountId = accountId
        self.contractId = contractId
    }

    /// The account ID for which information is requested.
    public var accountId: AccountId?

    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with `contractId`.
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId
        contractId = nil

        return self
    }

    /// The contract ID for which information is requested.
    public var contractId: ContractId?

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with `accountId`.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId
        accountId = nil

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case accountId
        case contractId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(accountId, forKey: .accountId)
        try container.encodeIfPresent(contractId, forKey: .contractId)

        try super.encode(to: encoder)
    }
}
