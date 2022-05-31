/// Response from [`TransactionReceiptQuery`][crate::TransactionReceiptQuery].
public final class TransactionReceiptResponse: Decodable {
    /// The receipt of processing the first consensus transaction with the given id.
    public let reciept: TransactionReceipt

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
        let container = try decoder.container(keyedBy: AnyQueryResponseCodingKeys.self)
        let data = try container.nestedContainer(keyedBy: CodingKeys.self, forKey: .transactionReceipt)

        reciept = try data.decode(TransactionReceipt.self, forKey: .receipt)
        duplicateReceipts = try data.decodeIfPresent([TransactionReceipt].self, forKey: .duplicateReceipts) ?? []
        childReceipts = try data.decodeIfPresent([TransactionReceipt].self, forKey: .childReceipts) ?? []
    }
}
