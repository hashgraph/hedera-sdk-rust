/// Retrieve the latest state of a topic.
public final class TopicInfoQuery: Query<TopicInfo> {
    /// Create a new `TopicInfoQuery`.
    public init(
        topicId: TopicId? = nil
    ) {
        self.topicId = topicId
    }

    /// The topic ID for which information is requested.
    public var topicId: TopicId?

    /// Sets the topic ID for which information is requested.
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

        try container.encodeIfPresent(topicId, forKey: .topicId)

        try super.encode(to: encoder)
    }
}
