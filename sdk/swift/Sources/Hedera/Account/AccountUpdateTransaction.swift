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

/// Change properties for the given account.
///
/// Any null field is ignored (left unchanged). This
/// transaction must be signed by the existing key for this account. If
/// the transaction is changing the key field, then the transaction must be
/// signed by both the old key (from before the change) and the new key.
///
public final class AccountUpdateTransaction: Transaction {
    /// Create a new `AccountCreateTransaction` ready for configuration.
    public override init() {}

    /// The account ID which is being updated in this transaction.
    public var accountId: AccountId? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the account ID which is being updated in this transaction.
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId

        return self
    }

    /// The new key.
    public var key: Key? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the new key.
    @discardableResult
    public func key(_ key: Key) -> Self {
        self.key = key

        return self
    }

    /// If true, this account's key must sign any transaction depositing into this account.
    public var receiverSignatureRequired: Bool? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Set to true, this account's key must sign any transaction depositing into this account.
    @discardableResult
    public func receiverSignatureRequired(_ receiverSignatureRequired: Bool) -> Self {
        self.receiverSignatureRequired = receiverSignatureRequired

        return self
    }

    /// The period until the account will be charged to extend its expiration date.
    public var autoRenewPeriod: Duration? {
        willSet(_it) {
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
    public var autoRenewAccountId: AccountId? = nil {
        willSet(_it) {
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

    /// The ID of the account to which this account is proxy staked.
    ///
    /// If `proxy_account_id` is `None`, or is an invalid account, or is an account
    /// that isn't a node, then this account is automatically proxy staked to
    /// a node chosen by the network, but without earning payments.
    ///
    /// If the `proxy_account_id` account refuses to accept proxy staking, or
    /// if it is not currently running a node, then it
    /// will behave as if `proxy_account_id` was `None`.
    @available(*, deprecated)
    public var proxyAccountId: AccountId? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    ///  Set the proxy account ID for this account
    @available(*, deprecated)
    public func proxyAccountId(_ proxyAccountId: AccountId) -> Self {
        self.proxyAccountId = proxyAccountId

        return self
    }

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    public var expirationTime: Timestamp? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// The memo associated with the account.
    public var accountMemo: String? {
        willSet(_it) {
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
    public var maxAutomaticTokenAssociations: UInt32? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the maximum number of tokens that an Account can be implicitly associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// ID of the account to which this account is staking.
    /// This is mutually exclusive with `stakedNodeId`.
    public var stakedAccountId: AccountId? {
        willSet(_it) {
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
        willSet(_it) {
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
    public var declineStakingReward: Bool? {
        willSet(_it) {
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
        case accountId
        case key
        case accountMemo
        case autoRenewPeriod
        case expirationTime
        case maxAutomaticTokenAssociations
        case stakedAccountId
        case stakedNodeId
        case declineStakingReward
        case autoRenewAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(key, forKey: .key)
        try container.encodeIfPresent(accountMemo, forKey: .accountMemo)
        try container.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try container.encodeIfPresent(expirationTime, forKey: .expirationTime)
        try container.encodeIfPresent(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try container.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try container.encodeIfPresent(stakedNodeId, forKey: .stakedNodeId)
        try container.encodeIfPresent(declineStakingReward, forKey: .declineStakingReward)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try accountId?.validateChecksums(on: ledgerId)
        try stakedAccountId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
