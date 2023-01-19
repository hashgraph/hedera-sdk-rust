import Foundation

// fixme: chunking
/// Topic message records.
public struct TopicMessage: Codable {
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

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        consensusTimestamp = try container.decodeIfPresent(Timestamp.self, forKey: .consensusTimestamp)!
        contents = Data(base64Encoded: try container.decode(String.self, forKey: .contents))!
        runningHash = Data(base64Encoded: try container.decode(String.self, forKey: .runningHash))!
        runningHashVersion = try container.decode(UInt64.self, forKey: .runningHashVersion)
        sequenceNumber = try container.decode(UInt64.self, forKey: .sequenceNumber)
        initialTransactionId = try container.decode(TransactionId.self, forKey: .initialTransactionId)
        chunkNumber = try container.decode(UInt32.self, forKey: .chunkNumber)
        chunkTotal = try container.decode(UInt32.self, forKey: .chunkTotal)
    }
}
