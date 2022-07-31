/// Get the record of a transaction, given its transaction ID.
///
public final class TransactionRecordQuery: Query<TransactionRecord> {
    /// The ID of the transaction for which the record is being requested.
    // TODO: TransactionId
    public var transactionId: String?

    /// Set the ID of the transaction for which the record is being requested.
    @discardableResult
    public func transactionId(_ transactionId: String) -> Self {
        self.transactionId = transactionId

        return self
    }

    /// Whether records of processing duplicate transactions should be returned.
    public var includeDuplicates: Bool = false

    /// Sets whether records of processing duplicate transactions should be returned.
    @discardableResult
    public func includeDuplicates(_ includeDuplicates: Bool) -> Self {
        self.includeDuplicates = includeDuplicates

        return self
    }

    /// Whether the response should include the records of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    public var includeChildren: Bool = false

    /// Sets whether the response should include the records of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    @discardableResult
    public func includeChildren(_ includeChildren: Bool) -> Self {
        self.includeChildren = includeChildren

        return self
    }

    /// Whether the record status should be validated.
    public var validateStatus: Bool = false

    /// Sets whether the record status should be validated.
    @discardableResult
    public func validateStatus(_ validateStatus: Bool) -> Self {
        self.validateStatus = validateStatus

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case transactionId
        case includeChildren
        case includeDuplicates
        case validateStatus
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(transactionId, forKey: .transactionId)
        try container.encode(includeDuplicates, forKey: .includeDuplicates)
        try container.encode(includeChildren, forKey: .includeChildren)
        try container.encode(validateStatus, forKey: .validateStatus)

        try super.encode(to: encoder)
    }
}
