import Foundation
import HederaProtobufs

// fixme: chunking
/// Topic message records.
public struct TopicMessage {
    /// The consensus timestamp of the message.
    public let consensusTimestamp: Timestamp

    /// The content of the message.
    public let contents: Data

    /// The new running hash of the topic that received the message.
    public let runningHash: Data

    /// Version of the SHA-384 digest used to update the running hash.
    public let runningHashVersion: UInt64

    /// The sequence number of the message relative to all other messages
    /// for the same topic.
    public let sequenceNumber: UInt64

    /// The `TransactionId` of the first chunk, gets copied to every subsequent chunk in
    /// a fragmented message.
    public let initialTransactionId: TransactionId?

    /// The sequence number (from 1 to total) of the current chunk in the message.
    public let chunkNumber: UInt32

    /// The total number of chunks in the message.
    public let chunkTotal: UInt32
}

extension TopicMessage: TryFromProtobuf {
    internal typealias Protobuf = Com_Hedera_Mirror_Api_Proto_ConsensusTopicResponse

    internal init(protobuf proto: Protobuf) throws {
        let initialTransactionId: TransactionId?
        let chunkNumber: UInt32
        let chunkTotal: UInt32

        if proto.hasChunkInfo {
            initialTransactionId = try .fromProtobuf(proto.chunkInfo.initialTransactionID)
            chunkNumber = UInt32(proto.chunkInfo.number)
            chunkTotal = UInt32(proto.chunkInfo.total)
        } else {
            initialTransactionId = nil
            chunkNumber = 1
            chunkTotal = 1
        }

        self.init(
            consensusTimestamp: .fromProtobuf(proto.consensusTimestamp),
            contents: proto.message,
            runningHash: proto.runningHash,
            runningHashVersion: proto.runningHashVersion,
            sequenceNumber: proto.sequenceNumber,
            initialTransactionId: initialTransactionId,
            chunkNumber: chunkNumber,
            chunkTotal: chunkTotal
        )
    }
}
