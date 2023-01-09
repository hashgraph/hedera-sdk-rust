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

/// Deletes one or more non-fungible approved allowances from an owner's account. This operation
/// will remove the allowances granted to one or more specific non-fungible token serial numbers. Each owner account
/// listed as wiping an allowance must sign the transaction. Hbar and fungible token allowances
/// can be removed by setting the amount to zero in `AccountAllowanceApproveTransaction`.
///
public final class AccountAllowanceDeleteTransaction: Transaction {
    public private(set) var nftAllowances: [NftRemoveAllowance] = [] {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Create a new `AccountAllowanceDeleteTransaction`.
    public override init() {
    }

    /// Remove all nft token allowances.
    @discardableResult
    public func deleteAllTokenNftAllowances(_ nftId: NftId, _ ownerAccountId: AccountId) -> Self {
        ensureNotFrozen()
        if var allowance = nftAllowances.first(where: { (allowance) in
            allowance.tokenId == nftId.tokenId && allowance.ownerAccountId == ownerAccountId
        }) {
            allowance.serials.append(nftId.serial)
        } else {
            nftAllowances.append(
                NftRemoveAllowance(
                    tokenId: nftId.tokenId,
                    ownerAccountId: ownerAccountId,
                    serials: [nftId.serial]
                ))
        }

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case nftAllowances
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(nftAllowances, forKey: .nftAllowances)

        try super.encode(to: encoder)
    }

    public override func validateChecksums(on ledgerId: LedgerId) throws {
        try nftAllowances.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
