/// Dissociates the provided account with the provided tokens.
///
/// Must be signed by the provided account's key.
///
public final class TokenDissociateTransaction: Transaction {
    /// Create a new `TokenDissociateTransaction`.
    public init(
        accountId: AccountId? = nil,
        tokenIds: [TokenId] = []
    ) {
        self.accountId = accountId
        self.tokenIds = tokenIds
    }

    /// The account to be dissociated with the provided tokens.
    public var accountId: AccountId?

    /// Sets the account to be dissociated with the provided tokens.
    @discardableResult
    public func accountId(_ accountId: AccountId?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The tokens to be dissociated with the provided account.
    public var tokenIds: [TokenId]

    /// Sets the tokens to be dissociated with the provided account.
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
