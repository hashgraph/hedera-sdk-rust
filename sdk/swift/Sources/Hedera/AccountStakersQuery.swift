/// Get all the accounts that are proxy staking to this account.
/// For each of them, give the amount currently staked.
public final class AccountStakersQuery: Query<[ProxyStaker]> {
    /// Create a new `AccountStakersQuery`.
    public init(
        accountId: AccountId? = nil
    ) {
        self.accountId = accountId
    }

    /// The account ID for which the records should be retrieved.
    public var accountId: AccountId?

    /// Sets the account ID for which the records should be retrieved.
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
