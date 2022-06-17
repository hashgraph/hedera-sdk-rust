/// Get all the information about an account, including the balance.
///
/// This does not get the list of account records.
///
public class AccountInfoQuery: Query<AccountInfo> {
    /// Create a new ``AccountInfoQuery`` ready for configuration and execution.
    public override init() {}

    /// The account ID for which information is requested.
    public var accountId: AccountId?

    /// Sets the account ID for which information is requested.
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case accountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(accountId, forKey: .accountId)

        try super.encode(to: encoder)
    }
}
