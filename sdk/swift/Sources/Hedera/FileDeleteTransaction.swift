/// Delete the given file.
///
/// After deletion, it will be marked as deleted and will have no contents.
/// Information about it will continue to exist until it expires.
///
public final class FileDeleteTransaction: Transaction {
    /// Create a new `FileDeleteTransaction` ready for configuration.
    public override init() {}

    /// The file to delete. It will be marked as deleted until it expires.
    /// Then it will disappear.
    public private(set) var fileId: FileId?

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
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .fileDelete)

        try data.encodeIfPresent(fileId, forKey: .fileId)

        try super.encode(to: encoder)
    }
}
