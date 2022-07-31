/// Get all the records for an account for any transfers into it and out of it,
/// that were above the threshold, during the last 25 hours.
public final class AccountRecordsQuery: Query<[TransactionRecord]> {
    /// Create a new `AccountRecordsQuery`.
    public init(
        accountId: AccountId? = nil
    ) {
        self.accountId = accountId
    }

    /// The account ID for which records are requested.
    public var accountId: AccountId?

    /// Sets the account ID for which records are requested.
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
