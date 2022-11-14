/// A custom transfer fee that was assessed during the handling of a ``TransferTransaction``.
public struct AssessedCustomFee: Equatable, Codable {
    /// The amount of currency charged to each payer.
    public let amount: Int64

    /// The currency `amount` is charged in, if `None` the fee is in HBar.
    public let tokenId: TokenId?

    /// The account that receives the fees that were charged.
    public let feeCollectorAccountId: AccountId?

    /// A list of all accounts that were charged this fee.
    public let payerAccountIdList: [AccountId]

    // todo: fromBytes
    // todo: toBytes
}
