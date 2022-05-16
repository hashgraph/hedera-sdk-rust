/// Response from ``AccountBalanceQuery``.
public final class AccountBalance: Decodable {
    /// The account that is being referenced.
    public let accountId: AccountId

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    public let balance: UInt64

    // TODO: tokens

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        let accountBalance = try container.nestedContainer(keyedBy: AccountBalanceKeys.self, forKey: .accountBalance)

        accountId = try accountBalance.decode(AccountId.self, forKey: .accountId)
        balance = try accountBalance.decode(UInt64.self, forKey: .balance)
    }
}

private enum CodingKeys: String, CodingKey {
    case accountBalance
}

private enum AccountBalanceKeys: String, CodingKey {
    case accountId
    case balance
}
