/// Pauses the token from being involved in any kind of transaction until it is unpaused.
public final class TokenPauseTransaction: Transaction {
    /// Create a new `TokenPauseTransaction`.
    public init(
        tokenId: TokenId? = nil
    ) {
        self.tokenId = tokenId
    }

    /// The token to be paused.
    public var tokenId: TokenId?

    /// Sets the token to be paused.
    @discardableResult
    public func tokenId(_ tokenId: TokenId?) -> Self {
        self.tokenId = tokenId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case tokenId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(tokenId, forKey: .tokenId)

        try super.encode(to: encoder)
    }
}
