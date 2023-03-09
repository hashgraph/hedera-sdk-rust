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
    internal init(
        accountId: AccountId? = nil,
        key: Key? = nil,
        receiverSignatureRequired: Bool? = nil,
        autoRenewPeriod: Duration? = nil,
        autoRenewAccountId: AccountId? = nil,
        proxyAccountId: AccountId? = nil,
        expirationTime: Timestamp? = nil,
        accountMemo: String? = nil,
        maxAutomaticTokenAssociations: UInt32? = nil,
        stakedAccountId: AccountId? = nil,
        stakedNodeId: UInt64? = nil,
        declineStakingReward: Bool? = nil
    ) {
        self.accountId = accountId
        self.key = key
        self.receiverSignatureRequired = receiverSignatureRequired
        self.autoRenewPeriod = autoRenewPeriod
        self.autoRenewAccountId = autoRenewAccountId
        proxyAccountIdInner = proxyAccountId
        self.expirationTime = expirationTime
        self.accountMemo = accountMemo
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
        self.declineStakingReward = declineStakingReward

        super.init()
    }

    /// Create a new `AccountCreateTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        accountId = try container.decodeIfPresent(.accountId)
        key = try container.decodeIfPresent(.key)
        receiverSignatureRequired = try container.decodeIfPresent(.receiverSignatureRequired)
        autoRenewPeriod = try container.decodeIfPresent(.autoRenewPeriod)
        autoRenewAccountId = try container.decodeIfPresent(.autoRenewAccountId)
        proxyAccountIdInner = try container.decodeIfPresent(.proxyAccountId)
        expirationTime = try container.decodeIfPresent(.expirationTime)
        accountMemo = try container.decodeIfPresent(.accountMemo)
        maxAutomaticTokenAssociations = try container.decodeIfPresent(.maxAutomaticTokenAssociations)
        stakedAccountId = try container.decodeIfPresent(.stakedAccountId)
        stakedNodeId = try container.decodeIfPresent(.stakedNodeId)
        declineStakingReward = try container.decodeIfPresent(.declineStakingReward)
        receiverSignatureRequired = try container.decodeIfPresent(.receiverSignatureRequired)
        proxyAccountIdInner = try container.decodeIfPresent(.proxyAccountId)

        try super.init(from: decoder)
    }

    /// The account ID which is being updated in this transaction.
    public var accountId: AccountId? {
        willSet {
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
        willSet {
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
        willSet {
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
    ///
    /// > Warning: This not supported on any hedera network at this time.
    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account to be used at this account's expiration time to extend the
    /// life of the account.  If `nil`, this account pays for its own auto renewal fee.
    ///
    /// > Warning: This not supported on any hedera network at this time.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    // this is the official recommendation for deprecation while not getting warnings internally.
    private var proxyAccountIdInner: AccountId? {
        willSet {
            ensureNotFrozen()
        }
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
        get { proxyAccountIdInner }
        set(value) { proxyAccountIdInner = value }
    }

    ///  Set the proxy account ID for this account
    @available(*, deprecated)
    public func proxyAccountId(_ proxyAccountId: AccountId) -> Self {
        self.proxyAccountId = proxyAccountId

        return self
    }

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    public var expirationTime: Timestamp? {
        willSet {
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
    public var maxAutomaticTokenAssociations: UInt32? {
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
    /// This is mutually exclusive with `stakedAccountId`.
    public var stakedNodeId: UInt64? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the ID of the node this account is staked to.
    /// This is mutually exclusive with `stakedAccountId`.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64) -> Self {
        self.stakedNodeId = stakedNodeId

        return self
    }

    /// If true, the account declines receiving a staking reward. The default value is false.
    public var declineStakingReward: Bool? {
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
        case receiverSignatureRequired
        case proxyAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(accountId, forKey: .accountId)
        try container.encodeIfPresent(key, forKey: .key)
        try container.encodeIfPresent(accountMemo, forKey: .accountMemo)
        try container.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try container.encodeIfPresent(expirationTime, forKey: .expirationTime)
        try container.encodeIfPresent(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try container.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try container.encodeIfPresent(stakedNodeId, forKey: .stakedNodeId)
        try container.encodeIfPresent(declineStakingReward, forKey: .declineStakingReward)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encodeIfPresent(receiverSignatureRequired, forKey: .receiverSignatureRequired)
        try container.encodeIfPresent(proxyAccountIdInner, forKey: .proxyAccountId)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try accountId?.validateChecksums(on: ledgerId)
        try stakedAccountId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try proxyAccountIdInner?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
