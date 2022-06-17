import Foundation

/// Delete a topic.
///
/// No more transactions or queries on the topic will succeed.
///
/// If an `admin_key` is set, this transaction must be signed by that key.
/// If there is no `admin_key`, this transaction will fail `UNAUTHORIZED`.
///
public final class TopicDeleteTransaction: Transaction {
    /// Create a new `TopicDeleteTransaction` ready for configuration.
    public override init() {}

    /// The topic ID which is being deleted in this transaction.
    public var topicId: TopicId?

    /// Sets the topic ID which is being deleted in this transaction.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case topicId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(topicId, forKey: .topicId)

        try super.encode(to: encoder)
    }
}
