public struct TokenNftAllowance: Codable, ValidateChecksums {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public var serials: [UInt64]
    public let approvedForAll: Bool?
    public let delegatingSpenderAccountId: AccountId?

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId.validateChecksums(on: ledgerId)
        try ownerAccountId.validateChecksums(on: ledgerId)
        try spenderAccountId.validateChecksums(on: ledgerId)
        try delegatingSpenderAccountId?.validateChecksums(on: ledgerId)
    }
}

public struct NftRemoveAllowance: Encodable, ValidateChecksums {
    public let tokenId: TokenId
    public let ownerAccountId: AccountId
    public var serials: [UInt64]

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId.validateChecksums(on: ledgerId)
        try ownerAccountId.validateChecksums(on: ledgerId)
    }
}
