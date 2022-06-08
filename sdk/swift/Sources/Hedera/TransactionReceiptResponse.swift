/// Response from [`TransactionReceiptQuery`][crate::TransactionReceiptQuery].
public final class TransactionReceiptResponse: Decodable {
    /// The receipt of processing the first consensus transaction with the given id.
    public let receipt: TransactionReceipt

    /// The receipts of processing all transactions with the given id, in consensus time order.
    public let duplicateReceipts: [TransactionReceipt]

    /// The receipts (if any) of all child transactions spawned by the transaction with the
    /// given top-level id, in consensus order.
    public let childReceipts: [TransactionReceipt]

    private enum CodingKeys: String, CodingKey {
        case receipt
        case duplicateReceipts
        case childReceipts
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        receipt = try container.decode(TransactionReceipt.self, forKey: .receipt)
        duplicateReceipts = try container.decodeIfPresent([TransactionReceipt].self, forKey: .duplicateReceipts) ?? []
        childReceipts = try container.decodeIfPresent([TransactionReceipt].self, forKey: .childReceipts) ?? []
    }
}
