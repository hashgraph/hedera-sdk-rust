import Foundation

/// Response from `FileContentsQuery`.
public struct FileContentsResponse: Decodable {
    /// The file ID of the file whose contents are being returned.
    public let fileId: FileId

    /// The bytes contained in the file.
    public let contents: Data

    private enum CodingKeys: String, CodingKey {
        case fileId
        case contents
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: AnyQueryResponseCodingKeys.self)
        let data = try container.nestedContainer(keyedBy: CodingKeys.self, forKey: .fileContents)

        fileId = try data.decode(FileId.self, forKey: .fileId)

        let contentsB64 = try data.decode(String.self, forKey: .contents)
        contents = Data(base64Encoded: contentsB64)!
    }
}
