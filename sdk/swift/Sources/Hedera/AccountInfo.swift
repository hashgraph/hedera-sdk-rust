public final class AccountInfo: Codable {
    /// The account that is being referenced.
    public let accountId: AccountId

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    public let balance: UInt64
}
