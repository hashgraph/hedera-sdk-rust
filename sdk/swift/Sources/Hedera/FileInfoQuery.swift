/// Get all the information about a file.
public final class FileInfoQuery: Query<FileInfo> {
    /// Create a new `FileInfoQuery`.
    public init(
        fileId: FileId? = nil
    ) {
        self.fileId = fileId
    }

    /// The file ID for which information is requested.
    public var fileId: FileId?

    /// Sets the file ID for which information is requested.
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
