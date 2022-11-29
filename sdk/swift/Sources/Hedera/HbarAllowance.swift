public struct HbarAllowance: Codable {
    public let ownerAccountId: AccountId
    public let spenderAccountId: AccountId
    public let amount: Hbar
}
