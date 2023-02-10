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

/// Create a new Hedera™ account.
public final class AccountCreateTransaction: Transaction {
    /// Create a new `AccountCreateTransaction` ready for configuration.
    public init(
        key: Key? = nil,
        initialBalance: Hbar = 0,
        receiverSignatureRequired: Bool = false,
        autoRenewPeriod: Duration? = nil,
        autoRenewAccountId: AccountId? = nil,
        accountMemo: String = "",
        maxAutomaticTokenAssociations: UInt32 = 0,
        alias: PublicKey? = nil,
        evmAddress: EvmAddress? = nil,
        stakedAccountId: AccountId? = nil,
        stakedNodeId: UInt64? = nil,
        declineStakingReward: Bool = false
    ) {
        self.key = key
        self.initialBalance = initialBalance
        self.receiverSignatureRequired = receiverSignatureRequired
        self.autoRenewPeriod = autoRenewPeriod
        self.accountMemo = accountMemo
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
        self.declineStakingReward = declineStakingReward

        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        key = try container.decodeIfPresent(.key)
        initialBalance = try container.decodeIfPresent(.initialBalance) ?? 0
        receiverSignatureRequired = try container.decodeIfPresent(.receiverSignatureRequired) ?? false
        accountMemo = try container.decodeIfPresent(.accountMemo) ?? ""
        autoRenewPeriod = try container.decodeIfPresent(.autoRenewPeriod)
        maxAutomaticTokenAssociations = try container.decodeIfPresent(.maxAutomaticTokenAssociations) ?? 0
        stakedAccountId = try container.decodeIfPresent(.stakedAccountId)
        stakedNodeId = try container.decodeIfPresent(.stakedNodeId)
        declineStakingReward = try container.decodeIfPresent(.declineStakingReward) ?? false
        alias = try container.decodeIfPresent(.alias)
        evmAddress = try container.decodeIfPresent(.evmAddress)
        autoRenewAccountId = try container.decodeIfPresent(.autoRenewAccountId)

        try super.init(from: decoder)
    }

    /// The key that must sign each transfer out of the account.
    public var key: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key that must sign each transfer out of the account.
    @discardableResult
    public func key(_ key: Key) -> Self {
        self.key = key

        return self
    }

    /// The initial number of Hbar to put into the account.
    public var initialBalance: Hbar {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the initial number of Hbar to put into the account.
    @discardableResult
    public func initialBalance(_ initialBalance: Hbar) -> Self {
        self.initialBalance = initialBalance

        return self
    }

    /// If true, this account's key must sign any transaction depositing into this account.
    public var receiverSignatureRequired: Bool {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set to true to require this account to sign any transfer of hbars to this account.
    @discardableResult
    public func receiverSignatureRequired(_ receiverSignatureRequired: Bool) -> Self {
        self.receiverSignatureRequired = receiverSignatureRequired

        return self
    }

    /// The period until the account will be charged to extend its expiration date.
    public var autoRenewPeriod: Duration? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the period until the account will be charged to extend its expiration date.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The account to be used at this account's expiration time to extend the
    /// life of the account.  If `nil`, this account pays for its own auto renewal fee.
    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account to be used at this account's expiration time to extend the
    /// life of the account.  If `nil`, this account pays for its own auto renewal fee.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The memo associated with the account.
    public var accountMemo: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the memo associated with the account.
    @discardableResult
    public func accountMemo(_ accountMemo: String) -> Self {
        self.accountMemo = accountMemo

        return self
    }

    /// The maximum number of tokens that an Account can be implicitly associated with.
    public var maxAutomaticTokenAssociations: UInt32 {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the maximum number of tokens that an Account can be implicitly associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// A key to be used as the account's alias.
    public var alias: PublicKey? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key to be used as the account's alias.
    @discardableResult
    public func alias(_ alias: PublicKey) -> Self {
        self.alias = alias

        return self
    }

    /// A 20-byte EVM address to be used as the account's evm address.
    public var evmAddress: EvmAddress? {
        willSet {
            ensureNotFrozen()
        }
    }

    @discardableResult
    public func evmAddress(_ evmAddress: EvmAddress) -> Self {
        self.evmAddress = evmAddress

        return self
    }

    /// ID of the account to which this account is staking.
    /// This is mutually exclusive with `stakedNodeId`.
    public var stakedAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the ID of the account to which this account is staking.
    /// This is mutually exclusive with `stakedNodeId`.
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountId) -> Self {
        self.stakedAccountId = stakedAccountId

        return self
    }

    /// ID of the node this account is staked to.
    /// This is mutually exclusive with `staked_account_id`.
    public var stakedNodeId: UInt64? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the ID of the node this account is staked to.
    /// This is mutually exclusive with `staked_account_id`.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64) -> Self {
        self.stakedNodeId = stakedNodeId

        return self
    }

    /// If true, the account declines receiving a staking reward. The default value is false.
    public var declineStakingReward: Bool {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set to true, the account declines receiving a staking reward. The default value is false.
    @discardableResult
    public func declineStakingReward(_ declineStakingReward: Bool) -> Self {
        self.declineStakingReward = declineStakingReward

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case key
        case initialBalance
        case receiverSignatureRequired
        case accountMemo
        case autoRenewPeriod
        case maxAutomaticTokenAssociations
        case stakedAccountId
        case stakedNodeId
        case declineStakingReward
        case alias
        case evmAddress
        case autoRenewAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(key, forKey: .key)
        try container.encode(initialBalance, forKey: .initialBalance)
        try container.encode(accountMemo, forKey: .accountMemo)
        try container.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try container.encode(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try container.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try container.encodeIfPresent(stakedNodeId, forKey: .stakedNodeId)
        try container.encode(declineStakingReward, forKey: .declineStakingReward)
        try container.encodeIfPresent(alias, forKey: .alias)
        try container.encodeIfPresent(evmAddress, forKey: .evmAddress)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try stakedAccountId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
