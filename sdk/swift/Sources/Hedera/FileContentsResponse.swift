import Foundation

/// Response from `FileContentsQuery`.
public struct FileContentsResponse: Codable {
    /// The file ID of the file whose contents are being returned.
    public let fileId: FileId

    /// The bytes contained in the file.
    public let contents: Data

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        fileId = try container.decode(FileId.self, forKey: .fileId)

        let contentsB64 = try container.decode(String.self, forKey: .contents)
        contents = Data(base64Encoded: contentsB64)!
    }
}
