import Foundation

/// Start a new smart contract instance.
public final class ContractCreateTransaction: Transaction {
    /// Create a new `ContractCreateTransaction`.
    public init(
        bytecode: Data? = nil,
        bytecodeFileId: FileId? = nil,
        adminKey: Key? = nil,
        gas: UInt64 = 0,
        initialBalance: UInt64 = 0,
        autoRenewPeriod: TimeInterval? = nil,
        constructorParameters: Data? = nil,
        contractMemo: String = "",
        maxAutomaticTokenAssociations: UInt32 = 0,
        autoRenewAccountId: AccountId? = nil,
        stakedAccountId: AccountId? = nil,
        stakedNodeId: UInt64? = nil,
        declineStakingReward: Bool = false
    ) {
        self.bytecode = bytecode
        self.bytecodeFileId = bytecodeFileId
        self.adminKey = adminKey
        self.gas = gas
        self.initialBalance = initialBalance
        self.autoRenewPeriod = autoRenewPeriod
        self.constructorParameters = constructorParameters
        self.contractMemo = contractMemo
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.autoRenewAccountId = autoRenewAccountId
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
        self.declineStakingReward = declineStakingReward
    }

    /// The bytes of the smart contract.
    public var bytecode: Data?

    /// Set the bytes of the smart contract.
    @discardableResult
    public func bytecode(_ bytecode: Data) -> Self {
        self.bytecode = bytecode

        return self
    }

    /// The file to use as the bytes for the smart contract.
    public var bytecodeFileId: FileId?

    /// Sets the file to use as the bytes for the smart contract.
    @discardableResult
    public func bytecodeFileId(_ bytecodeFileId: FileId) -> Self {
        self.bytecodeFileId = bytecodeFileId

        return self
    }

    /// The admin key.
    public var adminKey: Key?

    /// Sets the admin key.
    @discardableResult
    public func adminKey(_ adminKey: Key) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The gas limit to deploy the smart contract.
    public var gas: UInt64

    /// Sets the gas limit to deploy the smart contract.
    @discardableResult
    public func gas(_ gas: UInt64) -> Self {
        self.gas = gas

        return self
    }

    /// The initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    public var initialBalance: UInt64

    /// Sets the initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    @discardableResult
    public func initialBalance(_ initialBalance: UInt64) -> Self {
        self.initialBalance = initialBalance

        return self
    }

    /// The auto renew period for this smart contract.
    public var autoRenewPeriod: TimeInterval?

    /// Sets the auto renew period for this smart contract.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: TimeInterval) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The parameters to pass to the constructor.
    public var constructorParameters: Data?

    /// Sets the parameters to pass to the constructor.
    @discardableResult
    public func constructorParameters(_ constructorParameters: Data) -> Self {
        self.constructorParameters = constructorParameters

        return self
    }

    /// The memo for the new smart contract.
    public var contractMemo: String

    /// Sets the memo for the new smart contract.
    @discardableResult
    public func contractMemo(_ contractMemo: String) -> Self {
        self.contractMemo = contractMemo

        return self
    }

    /// The maximum number of tokens that this contract can be automatically associated with.
    public var maxAutomaticTokenAssociations: UInt32

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// The account to be used at the contract's expiration time to extend the life of the contract.
    public var autoRenewAccountId: AccountId?

    /// Sets the account to be used at the contract's expiration time to extend the life of the contract.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The ID of the account to which this contract is staking.
    public var stakedAccountId: AccountId?

    /// Sets the ID of the account to which this contract is staking.
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountId) -> Self {
        self.stakedAccountId = stakedAccountId

        return self
    }

    /// The ID of the node to which this contract is staking.
    public var stakedNodeId: UInt64?

    /// Sets the ID of the node to which this contract is staking.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64) -> Self {
        self.stakedNodeId = stakedNodeId

        return self
    }

    /// If true, the contract declines receiving a staking reward. The default value is false.
    public var declineStakingReward: Bool

    /// Set to true, the contract declines receiving a staking reward. The default value is false.
    @discardableResult
    public func declineStakingReward(_ declineStakingReward: Bool) -> Self {
        self.declineStakingReward = declineStakingReward

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case bytecode
        case bytecodeFileId
        case adminKey
        case gas
        case initialBalance
        case autoRenewPeriod
        case constructorParameters
        case contractMemo
        case maxAutomaticTokenAssociations
        case autoRenewAccountId
        case stakedAccountId
        case stakedNodeId
        case declineStakingReward
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(bytecode?.base64EncodedString(), forKey: .bytecode)
        try container.encodeIfPresent(bytecodeFileId, forKey: .bytecodeFileId)
        try container.encodeIfPresent(adminKey, forKey: .adminKey)
        try container.encode(gas, forKey: .gas)
        try container.encode(initialBalance, forKey: .initialBalance)
        try container.encodeIfPresent(autoRenewPeriod?.wholeSeconds, forKey: .autoRenewPeriod)
        try container.encodeIfPresent(constructorParameters?.base64EncodedString(), forKey: .constructorParameters)
        try container.encode(contractMemo, forKey: .contractMemo)
        try container.encode(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encodeIfPresent(stakedAccountId, forKey: .stakedAccountId)
        try container.encodeIfPresent(stakedNodeId, forKey: .stakedNodeId)
        try container.encode(declineStakingReward, forKey: .declineStakingReward)

        try super.encode(to: encoder)
    }
}
