import Foundation

/// Create a new Hedera™ account.
public final class AccountCreateTransaction: Transaction {
    /// Create a new `AccountCreateTransaction` ready for configuration.
    public override init() {}

    /// The key that must sign each transfer out of the account.
    public private(set) var key: Key?

    /// Sets the key that must sign each transfer out of the account.
    @discardableResult
    public func key(_ key: Key) -> Self {
        self.key = key

        return self
    }

    // TODO: Hbar
    /// The initial number of Hbar to put into the account.
    public private(set) var initialBalance: UInt64 = 0

    /// Sets the initial number of Hbar to put into the account.
    @discardableResult
    public func initialBalance(_ initialBalance: UInt64) -> Self {
        self.initialBalance = initialBalance

        return self
    }

    /// If true, this account's key must sign any transaction depositing into this account.
    public private(set) var receiverSignatureRequired: Bool = false

    /// Set to true to require this account to sign any transfer of hbars to this account.
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

    /// The memo associated with the account.
    public private(set) var accountMemo: String = ""

    /// Sets the memo associated with the account.
    @discardableResult
    public func accountMemo(_ accountMemo: String) -> Self {
        self.accountMemo = accountMemo

        return self
    }

    /// The maximum number of tokens that an Account can be implicitly associated with.
    public private(set) var maxAutomaticTokenAssociations: UInt32 = 0

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
    public private(set) var declineStakingReward: Bool = false

    /// Set to true, the account declines receiving a staking reward. The default value is false.
    @discardableResult
    public func declineStakingReward(_ declineStakingReward: Bool) -> Self {
        self.declineStakingReward = declineStakingReward

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case key
        case initialBalance
        case accountMemo
        case autoRenewPeriod
        case maxAutomaticTokenAssociations
        case stakedAccountId
        case stakedNodeId
        case declineStakingReward
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .accountCreate)

        try data.encodeIfPresent(key, forKey: .key)
        try data.encode(initialBalance, forKey: .initialBalance)
        try data.encode(accountMemo, forKey: .accountMemo)
        try data.encodeIfPresent(autoRenewPeriod?.wholeSeconds, forKey: .autoRenewPeriod)
        try data.encode(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try data.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try data.encodeIfPresent(stakedNodeId, forKey: .stakedNodeId)
        try data.encode(declineStakingReward, forKey: .declineStakingReward)

        try super.encode(to: encoder)
    }
}
