import Foundation

/// Updates the fields of a smart contract to the given values.
public final class ContractUpdateTransaction: Transaction {
    /// Create a new `ContractUpdateTransaction`.
    public init(
        contractId: ContractId? = nil,
        expirationTime: Date? = nil,
        adminKey: Key? = nil,
        autoRenewPeriod: TimeInterval? = nil,
        contractMemo: String? = nil,
        maxAutomaticTokenAssociations: UInt32? = nil,
        autoRenewAccountId: AccountId? = nil,
        stakedAccountId: AccountId? = nil,
        stakedNodeId: UInt64? = nil,
        declineStakingReward: Bool? = nil
    ) {
        self.contractId = contractId
        self.expirationTime = expirationTime
        self.adminKey = adminKey
        self.autoRenewPeriod = autoRenewPeriod
        self.contractMemo = contractMemo
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.autoRenewAccountId = autoRenewAccountId
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
        self.declineStakingReward = declineStakingReward
    }

    /// The contract to be updated.
    public var contractId: ContractId?

    /// Sets the contract to be updated.
    @discardableResult
    public func contractId(_ contractId: ContractId?) -> Self {
        self.contractId = contractId

        return self
    }

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    public var expirationTime: Date?

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    @discardableResult
    public func expirationTime(_ expirationTime: Date?) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// The new admin key.
    public var adminKey: Key?

    /// Sets the new admin key.
    @discardableResult
    public func adminKey(_ adminKey: Key?) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The auto renew period for this smart contract.
    public var autoRenewPeriod: TimeInterval?

    /// Set the auto renew period for this smart contract.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: TimeInterval?) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The memo for the new smart contract.
    public var contractMemo: String?

    /// Sets the memo for the new smart contract.
    @discardableResult
    public func contractMemo(_ contractMemo: String?) -> Self {
        self.contractMemo = contractMemo

        return self
    }

    /// The maximum number of tokens that this contract can be automatically associated with.
    public var maxAutomaticTokenAssociations: UInt32?

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32?) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// The account to be used at the contract's expiration time to extend the
    /// life of the contract.
    public var autoRenewAccountId: AccountId?

    /// Sets the account to be used at the contract's expiration time to extend the
    /// life of the contract.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId?) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The ID of the account to which this contract is staking.
    /// This is mutually exclusive with `staked_node_id`.
    public var stakedAccountId: AccountId?

    /// Set the ID of the account to which this contract is staking.
    /// This is mutually exclusive with `staked_node_id`.
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountId?) -> Self {
        self.stakedAccountId = stakedAccountId

        return self
    }

    /// The ID of the node to which this contract is staking.
    /// This is mutually exclusive with `staked_account_id`.
    public var stakedNodeId: UInt64?

    /// Set the ID of the node to which this contract is staking.
    /// This is mutually exclusive with `staked_account_id`.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64?) -> Self {
        self.stakedNodeId = stakedNodeId

        return self
    }

    /// If true, the contract declines receiving a staking reward. The default value is false.
    public var declineStakingReward: Bool?

    /// Set to true, the contract declines receiving a staking reward. The default value is false.
    @discardableResult
    public func declineStakingReward(_ declineStakingReward: Bool?) -> Self {
        self.declineStakingReward = declineStakingReward

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case contractId
        case expirationTime
        case adminKey
        case autoRenewPeriod
        case contractMemo
        case maxAutomaticTokenAssociations
        case autoRenewAccountId
        case stakedAccountId
        case stakedNodeId
        case declineStakingReward
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(contractId, forKey: .contractId)
        try container.encodeIfPresent(expirationTime?.unixTimestampNanos, forKey: .adminKey)
        try container.encodeIfPresent(adminKey, forKey: .adminKey)
        try container.encodeIfPresent(autoRenewPeriod?.wholeSeconds, forKey: .autoRenewPeriod)
        try container.encodeIfPresent(contractMemo, forKey: .contractMemo)
        try container.encodeIfPresent(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try container.encodeIfPresent(stakedNodeId, forKey: .stakedNodeId)
        try container.encodeIfPresent(declineStakingReward, forKey: .declineStakingReward)

        try super.encode(to: encoder)
    }
}
