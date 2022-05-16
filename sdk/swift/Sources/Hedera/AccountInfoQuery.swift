/// Get all the information about an account, including the balance.
///
/// This does not get the list of account records.
///
public class AccountInfoQuery: Query<AccountInfo> {
    /// Create a new ``AccountInfoQuery`` ready for configuration and execution.
    public override init() {}

    public private(set) var accountId: AccountId?

    /// Sets the account ID for which information is requested.
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId

        return self
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        var accountBalance = container.nestedContainer(
            keyedBy: AccountInfoKeys.self, forKey: .accountInfo)

        if accountId != nil {
            try accountBalance.encode(accountId, forKey: .accountId)
        }

        try super.encode(to: encoder)
    }
}

private enum CodingKeys: String, CodingKey {
    case accountInfo
}

private enum AccountInfoKeys: String, CodingKey {
    case accountId
}
