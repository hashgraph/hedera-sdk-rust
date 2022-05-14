/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than ``AccountInfoQuery``, which returns the balance plus
/// additional information.
///
public class AccountBalanceQuery: Query<AccountBalance> {
    private var balanceSource: AccountBalanceSource?

    /// Create a new ``AccountBalanceQuery`` ready for configuration and execution.
    public override init() {}

    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with ``contractId``.
    ///
    @discardableResult
    public func accountId(_ id: AccountIdOrAlias) -> Self {
        balanceSource = .account(id)

        return self
    }

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with ``accountId``.
    ///
    // TODO: Use ContractIdOrEvmAddress
    @discardableResult
    public func contractId(_ id: AccountIdOrAlias) -> Self {
        balanceSource = .account(id)

        return self
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        var accountBalance = container.nestedContainer(
            keyedBy: AccountBalanceKeys.self, forKey: .accountBalance)

        switch balanceSource {
        case .account(let id):
            try accountBalance.encode(String(describing: id), forKey: .accountId)

        // TODO: case .Contract
        case .none:
            break
        }

        try super.encode(to: encoder)
    }
}

private enum AccountBalanceSource {
    case account(AccountIdOrAlias)
    // TODO: case Contract
}

private enum CodingKeys: String, CodingKey {
    case accountBalance
}

private enum AccountBalanceKeys: String, CodingKey {
    case accountId
    case contractId
}
