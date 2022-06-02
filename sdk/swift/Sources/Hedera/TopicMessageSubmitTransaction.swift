import Foundation

/// Submit a message for consensus.
///
/// Valid and authorized messages on valid topics will be ordered by the consensus service, gossipped to the
/// mirror net, and published (in order) to all subscribers (from the mirror net) on this topic.
///
/// The `submitKey` (if any) must sign this transaction.
///
/// On success, the resulting `TransactionReceipt` contains the topic's updated `topicSequenceNumber` and
/// `topicRunningHash`.
///
public final class TopicMessageSubmitTransaction: Transaction {
    /// Create a new `TopicMessageSubmitTransaction` ready for configuration.
    public override init() {}

    /// The topic ID to submit this message to.
    public private(set) var topicId: TopicId?

    /// Sets the topic ID to submit this message to.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

        return self
    }

    /// Message to be submitted.
    /// Max size of the Transaction (including signatures) is 6KiB.
    public private(set) var message: Data = Data()

    /// Sets the message to be submitted.
    @discardableResult
    public func message(_ message: Data) -> Self {
        self.message = message

        return self
    }

    /// The `TransactionId` of the first chunk.
    ///
    /// Should get copied to every subsequent chunk in a fragmented message.
    // TODO: TransactionId
    public private(set) var initialTransactionId: String?

    /// Sets the `TransactionId` of the first chunk.
    @discardableResult
    public func initialTransactionId(_ initialTransactionId: String) -> Self {
        self.initialTransactionId = initialTransactionId

        return self
    }

    /// The total number of chunks in the message.
    /// Defaults to 1.
    public private(set) var chunkTotal: Int = 1

    /// Sets the total number of chunks in the message.
    @discardableResult
    public func chunkTotal(_ chunkTotal: Int) -> Self {
        self.chunkTotal = chunkTotal

        return self
    }

    /// The sequence number (from 1 to total) of the current chunk in the message.
    /// Defaults to 1.
    public private(set) var chunkNumber: Int = 1

    /// Sets the sequence number (from 1 to total) of the current chunk in the message.
    @discardableResult
    public func chunkNumber(_ chunkNumber: Int) -> Self {
        self.chunkNumber = chunkNumber

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case topicId
        case message
        case initialTransactionId
        case chunkTotal
        case chunkNumber
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .topicMessageSubmit)

        try data.encode(topicId, forKey: .topicId)
        try data.encodeIfPresent(message.base64EncodedString(), forKey: .message)
        try data.encodeIfPresent(initialTransactionId, forKey: .initialTransactionId)
        try data.encode(chunkTotal, forKey: .chunkTotal)
        try data.encode(chunkNumber, forKey: .chunkNumber)

        try super.encode(to: encoder)
    }
}
