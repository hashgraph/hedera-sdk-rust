/// Represents a transfer of an NFT from one account to another.

public struct TokenNftTransfer: Equatable, Codable {
    /// The ID of the NFT's token.
    public let tokenId: TokenId

    /// The account that the NFT is being transferred from.
    public let sender: AccountId

    /// The account that the NFT is being transferred to.
    public let receiver: AccountId

    /// The serial number for the NFT being transferred.
    public let serial: UInt64

    /// If true then the transfer is expected to be an approved allowance and the
    /// `sender` is expected to be the owner. The default is false.
    public let isApproved: Bool
}
