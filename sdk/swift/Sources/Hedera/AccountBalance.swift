/// Response from ``AccountBalanceQuery``.
public final class AccountBalanceResponse: Codable {
    /// The account that is being referenced.
    public let accountId: AccountId

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    public let hbars: UInt64
}
