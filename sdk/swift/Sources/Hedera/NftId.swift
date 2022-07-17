/// The unique identifier for a non-fungible token (NFT) instance on Hedera.
// TODO: convert from string
// TODO: comparable
public class NftId: Encodable, CustomStringConvertible {
    /// The (non-fungible) token of which this NFT is an instance.
    public let tokenId: TokenId

    /// The unique identifier for this instance.
    public let serialNumber: UInt64

    /// Create a new `NftId` from the passed `tokenId` and `serialNumber`.
    public init(tokenId: TokenId, serialNumber: UInt64) {
        self.tokenId = tokenId
        self.serialNumber = serialNumber
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public var description: String {
        "\(tokenId)/\(serialNumber)"
    }
}
