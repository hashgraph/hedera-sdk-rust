public final class AccountInfo: Decodable {
    /// The account that is being referenced.
    public let accountId: AccountId

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    public let balance: UInt64

    private enum CodingKeys: String, CodingKey {
        case accountId
        case balance
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: AnyQueryResponseCodingKeys.self)
        let data = try container.nestedContainer(keyedBy: CodingKeys.self, forKey: .accountInfo)

        accountId = try data.decode(AccountId.self, forKey: .accountId)
        balance = try data.decode(UInt64.self, forKey: .balance)
    }
}
