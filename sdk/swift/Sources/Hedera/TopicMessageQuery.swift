import AnyAsyncSequence
import Foundation
import GRPC
import HederaProtobufs

/// Query a stream of Hedera Consensus Service (HCS)
/// messages for an HCS Topic via a specific (possibly open-ended) time range.
public final class TopicMessageQuery: ValidateChecksums, MirrorQuery {
    public typealias Item = TopicMessage
    public typealias Response = [TopicMessage]

    /// Create a new `TopicMessageQuery`.
    public init(
        topicId: TopicId? = nil,
        startTime: Timestamp? = nil,
        endTime: Timestamp? = nil,
        limit: UInt64 = 0
    ) {
        self.topicId = topicId
        self.startTime = startTime
        self.endTime = endTime
        self.limit = limit
    }

    /// The topic ID to retrieve messages for.
    public var topicId: TopicId?

    /// Include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    public var startTime: Timestamp?

    /// Include messages which reached consensus before this time.
    public var endTime: Timestamp?

    /// The maximum number of message to receive before stopping.
    public var limit: UInt64

    /// Sets topic ID to retrieve messages for.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

        return self
    }

    /// Set to include messages which reached consensus on or after this time.
    /// Defaults to the current time.
    @discardableResult
    public func startTime(_ startTime: Timestamp) -> Self {
        self.startTime = startTime

        return self
    }

    /// Set to include messages which reached consensus before this time.
    @discardableResult
    public func endTime(_ endTime: Timestamp) -> Self {
        self.endTime = endTime

        return self
    }

    /// Sets the maximum number of messages to be returned, before closing the subscription.
    /// Defaults to _unlimited_.
    @discardableResult
    public func limit(_ limit: UInt64) -> Self {
        self.limit = limit

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case topicId
        case startTime
        case endTime
        case limit
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try topicId?.validateChecksums(on: ledgerId)
    }

    public func subscribe(_ client: Client, _ timeout: TimeInterval? = nil) -> AnyAsyncSequence<TopicMessage> {
        subscribeInner(client, timeout)
    }

    public func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> [TopicMessage] {
        try await executeInner(client, timeout)
    }
}

extension TopicMessageQuery: ToProtobuf {
    internal typealias Protobuf = Com_Hedera_Mirror_Api_Proto_ConsensusTopicQuery

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            topicId?.toProtobufInto(&proto.topicID)
            startTime?.toProtobufInto(&proto.consensusStartTime)
            endTime?.toProtobufInto(&proto.consensusEndTime)
            proto.limit = limit
        }
    }
}

extension TopicMessageQuery: MirrorRequest {
    internal typealias GrpcItem = TopicMessage.Protobuf

    internal static func collect<S>(_ stream: S) async throws -> Response
    where S: AsyncSequence, Item.Protobuf == S.Element {
        var items: [Item] = []
        for try await proto in stream {
            items.append(try Item.fromProtobuf(proto))
        }

        return items
    }

    internal func connect(channel: GRPCChannel) -> ConnectStream {
        let request = self.toProtobuf()

        return HederaProtobufs.Com_Hedera_Mirror_Api_Proto_ConsensusServiceAsyncClient(channel: channel)
            .subscribeTopic(request)
    }
}
