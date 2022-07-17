/// Gets information about the Token instance.
public final class TokenInfoQuery: Query<TokenInfo> {
    /// Create a new `TokenInfoQuery`.
    public init(
        tokenId: TokenId? = nil
    ) {
        self.tokenId = tokenId
    }

    /// The token ID for which information is requested.
    public var tokenId: TokenId?

    /// Sets the token ID for which information is requested.
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
