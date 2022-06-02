import Foundation

/// Change properties for the given account.
///
/// Any null field is ignored (left unchanged). This
/// transaction must be signed by the existing key for this account. If
/// the transaction is changing the key field, then the transaction must be
/// signed by both the old key (from before the change) and the new key.
///
public class AccountUpdateTransaction: Transaction {
    /// Create a new `AccountCreateTransaction` ready for configuration.
    public override init() {}

    /// The account ID which is being updated in this transaction.
    public private(set) var accountId: AccountIdOrAlias?

    /// Sets the account ID which is being updated in this transaction.
    @discardableResult
    public func accountId(_ accountId: AccountIdOrAlias) -> Self {
        self.accountId = accountId

        return self
    }

    /// The new key.
    public private(set) var key: Key?

    /// Sets the new key.
    @discardableResult
    public func key(_ key: Key) -> Self {
        self.key = key

        return self
    }

    /// If true, this account's key must sign any transaction depositing into this account.
    public private(set) var receiverSignatureRequired: Bool?

    /// Set to true, this account's key must sign any transaction depositing into this account.
    @discardableResult
    public func receiverSignatureRequired(_ receiverSignatureRequired: Bool) -> Self {
        self.receiverSignatureRequired = receiverSignatureRequired

        return self
    }

    /// The period until the account will be charged to extend its expiration date.
    public private(set) var autoRenewPeriod: TimeInterval?

    /// Sets the period until the account will be charged to extend its expiration date.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: TimeInterval) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    public private(set) var expiresAt: Date?

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    @discardableResult
    public func expiresAt(_ expiresAt: Date) -> Self {
        self.expiresAt = expiresAt

        return self
    }

    /// The memo associated with the account.
    public private(set) var accountMemo: String?

    /// Sets the memo associated with the account.
    @discardableResult
    public func accountMemo(_ accountMemo: String) -> Self {
        self.accountMemo = accountMemo

        return self
    }

    /// The maximum number of tokens that an Account can be implicitly associated with.
    public private(set) var maxAutomaticTokenAssociations: UInt32?

    /// Sets the maximum number of tokens that an Account can be implicitly associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// ID of the account to which this account is staking.
    /// This is mutually exclusive with `stakedNodeId`.
    public private(set) var stakedAccountId: AccountIdOrAlias?

    /// Sets the ID of the account to which this account is staking.
    /// This is mutually exclusive with `stakedNodeId`.
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountIdOrAlias) -> Self {
        self.stakedAccountId = stakedAccountId

        return self
    }

    /// ID of the node this account is staked to.
    /// This is mutually exclusive with `staked_account_id`.
    public private(set) var stakedNodeId: UInt64?

    /// Sets the ID of the node this account is staked to.
    /// This is mutually exclusive with `staked_account_id`.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64) -> Self {
        self.stakedNodeId = stakedNodeId

        return self
    }

    /// If true, the account declines receiving a staking reward. The default value is false.
    public private(set) var declineStakingReward: Bool?

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
        case expiresAt
        case maxAutomaticTokenAssociations
        case stakedAccountId
        case stakedNodeId
        case declineStakingReward
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .accountUpdate)

        try data.encodeIfPresent(key, forKey: .key)
        try data.encodeIfPresent(accountMemo, forKey: .accountMemo)
        try data.encodeIfPresent(autoRenewPeriod?.wholeSeconds, forKey: .autoRenewPeriod)
        try data.encodeIfPresent(expiresAt?.unixTimestampNanos, forKey: .expiresAt)
        try data.encodeIfPresent(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try data.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try data.encodeIfPresent(stakedNodeId, forKey: .stakedNodeId)
        try data.encodeIfPresent(declineStakingReward, forKey: .declineStakingReward)

        try super.encode(to: encoder)
    }
}
