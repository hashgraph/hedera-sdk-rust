/// Get the receipt of a transaction, given its transaction ID.
///
/// Once a transaction reaches consensus, then information about whether it succeeded or failed
/// will be available until the end of the receipt period.
///
public class TransactionReceiptQuery: Query<TransactionReceiptResponse> {
    /// The ID of the transaction for which the receipt is being requested.
    // TODO: TransactionId
    public private(set) var transactionId: String?;

    /// Set the ID of the transaction for which the receipt is being requested.
    @discardableResult
    public func transactionId(_ transactionId: String) -> Self {
        self.transactionId = transactionId

        return self
    }

    /// Whether receipts of processing duplicate transactions should be returned.
    public private(set) var includeDuplicates: Bool = false;

    /// Sets whether receipts of processing duplicate transactions should be returned.
    @discardableResult
    public func includeDuplicates(_ includeDuplicates: Bool) -> Self {
        self.includeDuplicates = includeDuplicates

        return self
    }

    /// Whether the response should include the receipts of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    public private(set) var includeChildren: Bool = false;

    /// Sets whether the response should include the receipts of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    @discardableResult
    public func includeChildren(_ includeChildren: Bool) -> Self {
        self.includeChildren = includeChildren

        return self
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        var transactionReceipt = container.nestedContainer(
            keyedBy: TransactionReceiptKeys.self, forKey: .transactionReceipt)

        try transactionReceipt.encode(transactionId, forKey: .transactionId)
        try transactionReceipt.encode(includeDuplicates, forKey: .includeDuplicates)
        try transactionReceipt.encode(includeChildren, forKey: .includeChildren)

        try super.encode(to: encoder)
    }
}

private enum CodingKeys: String, CodingKey {
    case transactionReceipt
}

private enum TransactionReceiptKeys: String, CodingKey {
    case transactionId
    case includeChildren
    case includeDuplicates
}
