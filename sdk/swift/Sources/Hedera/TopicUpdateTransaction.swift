import Foundation

/// Change properties for the given topic.
///
/// Any null field is ignored (left unchanged).
///
public final class TopicUpdateTransaction: Transaction {
    /// Create a new `TopicUpdateTransaction` ready for configuration.
    public override init() {}

    /// The topic ID which is being updated in this transaction.
    public private(set) var topicId: TopicId?

    /// Sets the topic ID which is being updated in this transaction.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

        return self
    }

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    public private(set) var expiresAt: Date?

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    @discardableResult
    public func expiresAt(_ expiresAt: Date) -> Self {
        self.expiresAt = expiresAt

        return self
    }

    /// Short publicly visible memo about the topic. No guarantee of uniqueness.
    public private(set) var topicMemo: String = ""

    /// Sets the short publicly visible memo about the topic.
    @discardableResult
    public func topicMemo(_ topicMemo: String) -> Self {
        self.topicMemo = topicMemo

        return self
    }

    /// Access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    public private(set) var adminKey: Key?

    /// Sets the access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    @discardableResult
    public func adminKey(_ adminKey: Key) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// Access control for `TopicMessageSubmitTransaction`.
    public private(set) var submitKey: Key?

    /// Sets the access control for `TopicMessageSubmitTransaction`.
    @discardableResult
    public func submitKey(_ submitKey: Key) -> Self {
        self.submitKey = submitKey

        return self
    }

    /// The initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time, if
    /// the `autoRenewAccountId` is configured.
    public private(set) var autoRenewPeriod: TimeInterval?

    /// Sets the initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: TimeInterval) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// Account to be used at the topic's expiration time to extend the life of the topic.
    public private(set) var autoRenewAccountId: AccountId?

    /// Sets the account to be used at the topic's expiration time to extend the life of the topic.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case topicId
        case expiresAt
        case topicMemo
        case adminKey
        case submitKey
        case autoRenewPeriod
        case autoRenewAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .topicUpdate)

        try data.encode(topicId, forKey: .topicId)
        try data.encodeIfPresent(expiresAt?.unixTimestampNanos, forKey: .expiresAt)
        try data.encodeIfPresent(topicMemo, forKey: .topicMemo)
        try data.encodeIfPresent(adminKey, forKey: .adminKey)
        try data.encodeIfPresent(submitKey, forKey: .submitKey)
        try data.encodeIfPresent(autoRenewPeriod?.wholeSeconds, forKey: .autoRenewPeriod)
        try data.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)

        try super.encode(to: encoder)
    }
}
