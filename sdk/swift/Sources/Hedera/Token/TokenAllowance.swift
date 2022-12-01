public struct TokenAllowance: Codable {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public let amount: UInt64
}
