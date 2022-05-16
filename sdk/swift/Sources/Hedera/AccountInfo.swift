public final class AccountInfo: Decodable {
    /// The account that is being referenced.
    public let accountId: AccountId

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    public let balance: UInt64

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        let accountInfo = try container.nestedContainer(keyedBy: AccountInfoKeys.self, forKey: .accountInfo)

        accountId = try accountInfo.decode(AccountId.self, forKey: .accountId)
        balance = try accountInfo.decode(UInt64.self, forKey: .balance)
    }
}

private enum CodingKeys: String, CodingKey {
    case accountInfo
}

private enum AccountInfoKeys: String, CodingKey {
    case accountId
    case balance
}
