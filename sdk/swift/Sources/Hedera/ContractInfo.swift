import Foundation

public final class ContractInfo: Codable {
    /// ID of the contract instance, in the format used by transactions.
    public let contractId: ContractId

    /// ID of the cryptocurrency account owned by the contract instance,
    /// in the format used in transactions.
    public let accountId: AccountId

    /// ID of both the contract instance and the cryptocurrency account owned by the contract
    /// instance, in the format used by Solidity.
    public let contractAccountId: String

    /// The admin key of the contract instance.
    public let adminKey: Key?

    /// The current time at which this contract instance (and its account) is set to expire.
    public let expirationTime: Date?

    /// The auto renew period for this contract instance.
    public let autoRenewPeriod: TimeInterval?

    /// Number of bytes of storage being used by this instance.
    public let storage: UInt64

    /// The memo associated with the contract.
    public let contractMemo: String

    // TODO: Use Hbar type
    /// The current balance, in tinybars.
    public let balance: UInt64

    /// Whether the contract has been deleted.
    public let isDeleted: Bool

    /// ID of the an account to charge for auto-renewal of this contract.
    public let autoRenewAccountId: AccountId?

    /// The maximum number of tokens that a contract can be implicitly associated with.
    public let maxAutomaticTokenAssociations: UInt32
}
