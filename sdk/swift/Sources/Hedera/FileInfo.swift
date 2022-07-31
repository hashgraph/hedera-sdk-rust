import Foundation

// TODO: keys
/// Response from `FileInfoQuery`.
public final class FileInfo: Codable {
    /// The file ID of the file for which information is requested.
    public let fileId: FileId

    /// Number of bytes in contents.
    public let size: UInt64

    /// Current time which this account is set to expire.
    public let expirationTime: TimeInterval?

    /// True if deleted but not yet expired.
    public let isDeleted: Bool

    /// Memo associated with the file.
    public let fileMemo: String
}
