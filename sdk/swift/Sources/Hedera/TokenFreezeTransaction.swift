/// Freezes transfers of the specified token for the account.
public class TokenFreezeTransaction: Transaction {
    /// Create a new `TokenFreezeTransaction`.
    public init(
        accountId: AccountAddress? = nil,
        tokenId: TokenId? = nil
    ) {
        self.accountId = accountId
        self.tokenId = tokenId
    }

    /// The account to be frozen.
    public var accountId: AccountAddress?

    /// Sets the account to be frozen.
    @discardableResult
    public func accountId(_ accountId: AccountAddress?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The token for which this account will be frozen.
    public var tokenId: TokenId?

    /// Sets the token for which this account will be frozen.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case accountId
        case tokenId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(accountId, forKey: .accountId)
        try container.encode(tokenId, forKey: .tokenId)

        try super.encode(to: encoder)
    }
}
