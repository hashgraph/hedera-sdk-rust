/// Associates the provided account with the provided tokens.
///
/// Must be signed by the provided account's key.
///
public class TokenAssociateTransaction: Transaction {
    /// Create a new `TokenAssociateTransaction`.
    public init(
        accountId: AccountAddress? = nil,
        tokenIds: [TokenId] = []
    ) {
        self.accountId = accountId
        self.tokenIds = tokenIds
    }

    /// The account to be associated with the provided tokens.
    public var accountId: AccountAddress?

    /// Sets the account to be associated with the provided tokens.
    @discardableResult
    public func accountId(_ accountId: AccountAddress?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The tokens to be associated with the provided account.
    public var tokenIds: [TokenId]

    /// Sets the tokens to be associated with the provided account.
    @discardableResult
    public func tokenIds(_ tokenIds: [TokenId]) -> Self {
        self.tokenIds = tokenIds

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case accountId
        case tokenIds
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(accountId, forKey: .accountId)
        try container.encode(tokenIds, forKey: .tokenIds)

        try super.encode(to: encoder)
    }
}
