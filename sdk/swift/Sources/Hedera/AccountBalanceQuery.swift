/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than `AccountInfoQuery`, which returns the balance plus
/// additional information.
///
public class AccountBalanceQuery: Query<AccountBalanceResponse> {
    private var balanceSource: AccountBalanceSource?

    /// Create a new `AccountBalanceQuery` ready for configuration.
    public override init() {}

    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with `contractId`.
    ///
    @discardableResult
    public func accountId(_ id: AccountIdOrAlias) -> Self {
        balanceSource = .accountId(id)

        return self
    }

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with `accountId`.
    ///
    // TODO: Use ContractIdOrEvmAddress
    @discardableResult
    public func contractId(_ id: AccountIdOrAlias) -> Self {
        balanceSource = .contractId(id)

        return self
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyQueryCodingKeys.self)

        try container.encodeIfPresent(balanceSource, forKey: .accountBalance)
        try super.encode(to: encoder)
    }
}

private enum AccountBalanceSource: Encodable {
    case accountId(AccountIdOrAlias)
    // TODO: Use ContractIdOrEvmAddress
    case contractId(AccountIdOrAlias)
}
