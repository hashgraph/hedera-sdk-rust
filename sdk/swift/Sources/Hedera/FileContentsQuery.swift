import Foundation

/// Get the contents of a file.
public final class FileContentsQuery: Query<FileContentsResponse> {
    /// Create a new `FileContentsQuery` ready for configuration.
    public override init() {}

    /// The file ID for which contents are requested.
    public private(set) var fileId: FileId?

    /// Sets the file ID for which contents are requested.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyQueryCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .fileContents)

        try data.encode(fileId, forKey: .fileId)

        try super.encode(to: encoder)
    }
}
