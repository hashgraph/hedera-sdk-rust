/// Marks a token as deleted, though it will remain in the ledger.
public final class TokenDeleteTransaction: Transaction {
    /// Create a new `TokenDeleteTransaction`.
    public init(
        tokenId: TokenId? = nil
    ) {
        self.tokenId = tokenId
    }

    /// The token to be deleted.
    public var tokenId: TokenId?

    /// Sets the token to be deleted.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case tokenId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(tokenId, forKey: .tokenId)

        try super.encode(to: encoder)
    }
}
