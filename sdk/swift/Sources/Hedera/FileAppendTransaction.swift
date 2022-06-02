import Foundation

/// Append the given contents to the end of the specified file.
public final class FileAppendTransaction: Transaction {
    /// Create a new `FileAppendTransaction` ready for configuration.
    public override init() {}

    /// The file to which the bytes will be appended.
    public private(set) var fileId: FileId?

    /// Sets the file to which the bytes will be appended.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The bytes that will be appended to the end of the specified file.
    public private(set) var contents: Data = Data()

    /// Sets the bytes that will be appended to the end of the specified file.
    @discardableResult
    public func contents(_ contents: Data) -> Self {
        self.contents = contents

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
        case contents
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: AnyTransactionCodingKeys.self)
        var data = container.nestedContainer(keyedBy: CodingKeys.self, forKey: .fileAppend)

        try data.encodeIfPresent(fileId, forKey: .fileId)
        try data.encode(contents.base64EncodedString(), forKey: .contents)

        try super.encode(to: encoder)
    }
}
