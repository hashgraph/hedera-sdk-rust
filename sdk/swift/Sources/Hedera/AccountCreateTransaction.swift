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
    public func initialBalance(_ balance: UInt64) -> Self {
        self.initialBalance = balance

        return self
    }

    /// If true, this account's key must sign any transaction depositing into this account.
    public private(set) var receiverSignatureRequired: Bool = false

    /// Set to true to require this account to sign any transfer of hbars to this account.
    @discardableResult
    public func receiverSignatureRequired(_ required: Bool) -> Self {
        self.receiverSignatureRequired = required

        return self
    }

    /// The account is charged to extend its expiration date every this many seconds.
    public private(set) var autoRenewPeriod: TimeInterval?

    /// Sets the account is charged to extend its expiration date every this many seconds.
    @discardableResult
    public func autoRenewPeriod(_ period: TimeInterval) -> Self {
        self.autoRenewPeriod = period

        return self
    }

    /// The memo associated with the account.
    public private(set) var accountMemo: String = ""

    /// Sets the memo associated with the account.
    @discardableResult
    public func accountMemo(_ memo: String) -> Self {
        self.accountMemo = memo

        return self
    }

    /// The maximum number of tokens that an Account can be implicitly associated with.
    public private(set) var maxAutomaticTokenAssociations: UInt32 = 0

    /// Sets the maximum number of tokens that an Account can be implicitly associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ max: UInt32) -> Self {
        self.maxAutomaticTokenAssociations = max

        return self
    }

    /// ID of the account to which this account is staking.
    public private(set) var stakedAccountId: AccountIdOrAlias?

    /// Sets the ID of the account to which this account is staking.
    @discardableResult
    public func stakedAccountId(_ accountId: AccountIdOrAlias) -> Self {
        self.stakedAccountId = accountId

        return self
    }

    /// If true, the account declines receiving a staking reward. The default value is false.
    public private(set) var declineStakingReward: Bool = false

    /// Set to true, the account declines receiving a staking reward. The default value is false.
    @discardableResult
    public func declineStakingReward(_ decline: Bool) -> Self {
        self.declineStakingReward = decline

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case key
        case initialBalance
        case accountMemo
        case autoRenewPeriod
        case maxAutomaticTokenAssociations
        case stakedAccountId
        case declineStakingReward
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .accountCreate)

        try data.encodeIfPresent(key, forKey: .key)
        try data.encode(initialBalance, forKey: .initialBalance)
        try data.encode(accountMemo, forKey: .accountMemo)
        try data.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try data.encode(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try data.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try data.encode(declineStakingReward, forKey: .declineStakingReward)

        try super.encode(to: encoder)
    }
}
