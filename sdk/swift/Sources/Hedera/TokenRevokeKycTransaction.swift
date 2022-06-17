/// Revokes KYC from the account for the given token.
public class TokenRevokeKycTransaction: Transaction {
    /// Create a new `TokenRevokeKycTransaction`.
    public init(
        accountId: AccountAddress? = nil,
        tokenId: TokenId? = nil
    ) {
        self.accountId = accountId
        self.tokenId = tokenId
    }

    /// The account to have their KYC revoked.
    public var accountId: AccountAddress?

    /// Sets the account to have their KYC revoked.
    @discardableResult
    public func accountId(_ accountId: AccountAddress?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The token for which this account will have their KYC revoked.
    public var tokenId: TokenId?

    /// Sets the token for which this account will have their KYC revoked.
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
