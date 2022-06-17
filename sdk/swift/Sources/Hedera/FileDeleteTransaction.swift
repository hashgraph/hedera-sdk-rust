/// Delete the given file.
///
/// After deletion, it will be marked as deleted and will have no contents.
/// Information about it will continue to exist until it expires.
///
public final class FileDeleteTransaction: Transaction {
    /// Create a new `FileDeleteTransaction`.
    public init(
        fileId: FileId? = nil
    ) {
        self.fileId = fileId
    }

    /// The file to delete. It will be marked as deleted until it expires.
    /// Then it will disappear.
    public var fileId: FileId?

    /// Sets the file to delete. It will be marked as deleted until it expires.
    /// Then it will disappear.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(fileId, forKey: .fileId)

        try super.encode(to: encoder)
    }
}
