public struct TokenAllowance: Codable, ValidateChecksums {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public let amount: UInt64

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId.validateChecksums(on: ledgerId)
        try ownerAccountId.validateChecksums(on: ledgerId)
        try spenderAccountId.validateChecksums(on: ledgerId)
    }
}
