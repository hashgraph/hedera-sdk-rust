/// Gets info on an NFT for a given TokenID and serial number.
public final class TokenNftInfoQuery: Query<TokenNftInfo> {
    /// Create a new `TokenNftInfoQuery`.
    public init(
        nftId: NftId? = nil
    ) {
        self.nftId = nftId
    }

    /// The nft ID for which information is requested.
    public var nftId: NftId?

    /// Sets the nft ID for which information is requested.
    @discardableResult
    public func nftId(_ nftId: NftId) -> Self {
        self.nftId = nftId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case nftId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(nftId, forKey: .nftId)

        try super.encode(to: encoder)
    }
}
