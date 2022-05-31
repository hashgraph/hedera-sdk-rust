/// Response from [`TransactionReceiptQuery`][crate::TransactionReceiptQuery].
public final class TransactionReceiptResponse: Decodable {
    /// The receipt of processing the first consensus transaction with the given id.
    public let reciept: TransactionReceipt;

    /// The receipts of processing all transactions with the given id, in consensus time order.
    public let duplicateReceipts: [TransactionReceipt];

    /// The receipts (if any) of all child transactions spawned by the transaction with the
    /// given top-level id, in consensus order.
    public let childReceipts: [TransactionReceipt];

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        let transactionReceipt = try container.nestedContainer(keyedBy: TransactionReceiptKeys.self, forKey: .transactionReceipt)

        reciept = try transactionReceipt.decode(TransactionReceipt.self, forKey: .receipt)
        duplicateReceipts = try transactionReceipt.decodeIfPresent([TransactionReceipt].self, forKey: .duplicateReceipts) ?? []
        childReceipts = try transactionReceipt.decodeIfPresent([TransactionReceipt].self, forKey: .childReceipts) ?? []
    }
}

private enum CodingKeys: String, CodingKey {
    case transactionReceipt
}

private enum TransactionReceiptKeys: String, CodingKey {
    case receipt
    case duplicateReceipts
    case childReceipts
}
