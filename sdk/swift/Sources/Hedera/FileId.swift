import CHedera

/// The unique identifier for a file on Hedera.
public final class FileId: LosslessStringConvertible, Codable {
    /// The shard number (non-negative).
    public let shard: UInt64

    /// The realm number (non-negative).
    public let realm: UInt64

    public let num: UInt64

    public init(num: UInt64, shard: UInt64 = 0, realm: UInt64 = 0) {
        self.num = num
        self.shard = shard
        self.realm = realm
    }

    public convenience init?(_ description: String) {
        var fileId = HederaFileId()
        let err = hedera_file_id_from_string(description, &fileId)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        self.init(num: fileId.num, shard: fileId.shard, realm: fileId.realm)
    }

    public convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }

    public var description: String {
        "\(shard).\(realm).\(num)"
    }
}
