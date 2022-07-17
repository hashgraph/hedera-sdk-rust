/// At consensus, updates a token type's fee schedule to the given list of custom fees.
public class TokenFeeScheduleUpdateTransaction: Transaction {
    /// Create a new `TokenFeeScheduleUpdateTransaction`.
    public init(
        tokenId: TokenId? = nil,
        customFees: [CustomFee] = []
    ) {
        self.tokenId = tokenId
        self.customFees = customFees
    }

    /// The token whose fee schedule is to be updated.
    public var tokenId: TokenId?

    /// Sets the token whose fee schedule is to be updated.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The new custom fees to be assessed during a transfer.
    public var customFees: [CustomFee]

    /// Sets the new custom fees to be assessed during a transfer.
    @discardableResult
    public func customFees(_ customFees: [CustomFee]) -> Self {
        self.customFees = customFees

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case tokenId
        case customFees
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(tokenId, forKey: .tokenId)
        try container.encode(customFees, forKey: .customFees)

        try super.encode(to: encoder)
    }
}
