import Foundation

/// Response from `TokenNftInfoQuery`.
public final class TokenNftInfo: Codable {
    /// The ID of the NFT.
    public let nftId: NftId

    /// The current owner of the NFT.
    public let accountId: AccountId

    /// Effective consensus timestamp at which the NFT was minted.
    public let creationTime: Date

    /// The unique metadata of the NFT.
    public let metadata: Data

    /// If an allowance is granted for the NFT, its corresponding spender account.
    public let spenderAccountId: AccountId?
}
