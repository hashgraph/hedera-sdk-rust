import Foundation

/// Modify the metadata and/or the contents of a file.
///
/// If a field is not set in the transaction body, the
/// corresponding file attribute will be unchanged.
///
public final class FileUpdateTransaction: Transaction {
    /// Create a new `FileUpdateTransaction` ready for configuration.
    public override init() {}

    /// The file ID which is being updated in this transaction.
    public var fileId: FileId?

    /// Sets the file ID which is being updated in this transaction.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The memo associated with the file.
    public var fileMemo: String = ""

    /// Sets the memo associated with the file.
    @discardableResult
    public func fileMemo(_ fileMemo: String) -> Self {
        self.fileMemo = fileMemo

        return self
    }

    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    public var keys: [Key] = []

    /// Sets the keys for this file.
    ///
    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    ///
    @discardableResult
    public func keys(_ keys: [Key]) -> Self {
        self.keys = keys

        return self
    }

    /// The bytes that are to be the contents of the file.
    public var contents: Data = Data()

    /// Sets the bytes that are to be the contents of the file.
    @discardableResult
    public func contents(_ contents: Data) -> Self {
        self.contents = contents

        return self
    }

    /// The time at which this file should expire.
    public var expiresAt: Date?

    /// Sets the time at which this file should expire.
    @discardableResult
    public func expiresAt(_ expiresAt: Date) -> Self {
        self.expiresAt = expiresAt

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
        case fileMemo
        case keys
        case contents
        case expiresAt
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(fileId, forKey: .fileId)
        try container.encode(fileMemo, forKey: .fileMemo)
        try container.encode(keys, forKey: .keys)
        try container.encode(contents.base64EncodedString(), forKey: .contents)
        try container.encodeIfPresent(expiresAt?.unixTimestampNanos, forKey: .expiresAt)

        try super.encode(to: encoder)
    }
}
