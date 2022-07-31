/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than `AccountInfoQuery`, which returns the balance plus
/// additional information.
///
public final class AccountBalanceQuery: Query<AccountBalanceResponse> {
    /// Create a new `AccountBalanceQuery`.
    public init(
        accountId: AccountId? = nil,
        contractId: AccountId? = nil
    ) {
        self.accountId = accountId
        self.contractId = contractId
    }

    /// The account ID for which information is requested.
    public var accountId: AccountId?

    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with `contractId`.
    ///
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId
        contractId = nil

        return self
    }

    /// The contract ID for which information is requested.
    // TODO: Use ContractIdOrEvmAddress
    public var contractId: AccountId?

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with `accountId`.
    ///
    // TODO: Use ContractIdOrEvmAddress
    @discardableResult
    public func contractId(_ contractId: AccountId) -> Self {
        self.contractId = contractId
        accountId = nil

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case accountId
        case contractId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(accountId, forKey: .accountId)
        try container.encodeIfPresent(contractId, forKey: .contractId)

        try super.encode(to: encoder)
    }
}
