/// Grants KYC to the account for the given token.
public final class TokenGrantKycTransaction: Transaction {
    /// Create a new `TokenGrantKycTransaction`.
    public init(
        accountId: AccountId? = nil,
        tokenId: TokenId? = nil
    ) {
        self.accountId = accountId
        self.tokenId = tokenId
    }

    /// The account to be granted KYC.
    public var accountId: AccountId?

    /// Sets the account to be granted KYC.
    @discardableResult
    public func accountId(_ accountId: AccountId?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The token for which this account will be granted KYC.
    public var tokenId: TokenId?

    /// Sets the token for which this account will be granted KYC.
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
