public struct TokenNftAllowance: Codable {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public var serials: [UInt64]
    public let approvedForAll: Bool?
    public let delegatingSpenderAccountId: AccountId?
}

public struct NftRemoveAllowance: Encodable {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public var serials: [UInt64]
}
