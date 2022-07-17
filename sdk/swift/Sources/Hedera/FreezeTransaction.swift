import Foundation

/// Set the freezing period in which the platform will stop creating
/// events and accepting transactions.
public class FreezeTransaction: Transaction {
    /// Create a new `FreezeTransaction`.
    public init(
        startTime: Date? = nil,
        fileId: FileId? = nil,
        fileHash: Data? = nil,
        freezeType: FreezeType = .unknown
    ) {
        self.startTime = startTime
        self.fileId = fileId
        self.fileHash = fileHash
        self.freezeType = freezeType
    }

    /// The start time.
    public var startTime: Date?

    /// Sets the start time.
    @discardableResult
    public func startTime(_ startTime: Date) -> Self {
        self.startTime = startTime

        return self
    }

    /// The file ID.
    public var fileId: FileId?

    /// Sets the file ID.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The file hash.
    public var fileHash: Data?

    /// Sets the file hash.
    @discardableResult
    public func fileHash(_ fileHash: Data) -> Self {
        self.fileHash = fileHash

        return self
    }

    /// The freeze type.
    public var freezeType: FreezeType

    /// Sets the freeze type.
    @discardableResult
    public func freezeType(_ freezeType: FreezeType) -> Self {
        self.freezeType = freezeType

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case startTime
        case fileId
        case fileHash
        case freezeType
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(startTime, forKey: .startTime)
        try container.encodeIfPresent(fileId, forKey: .fileId)
        try container.encodeIfPresent(fileHash, forKey: .fileHash)
        try container.encode(freezeType, forKey: .freezeType)

        try super.encode(to: encoder)
    }
}
