public struct HbarAllowance: Codable, ValidateChecksums {
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public let amount: Hbar

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try ownerAccountId.validateChecksums(on: ledgerId)
        try spenderAccountId.validateChecksums(on: ledgerId)
    }
}
