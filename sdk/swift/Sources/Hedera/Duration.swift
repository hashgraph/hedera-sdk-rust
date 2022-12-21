import Foundation
import HederaProtobufs

public struct Duration: Codable {
    public let seconds: UInt64

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()

        seconds = try container.decode(UInt64.self)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(seconds)
    }
}

extension Duration: ProtobufCodable {
    typealias Protobuf = Proto_Duration

    init(fromProtobuf proto: Protobuf) {
        seconds = UInt64(proto.seconds)
    }

    func toProtobuf() -> Protobuf {
        .with { proto in proto.seconds = Int64(seconds) }
    }
}
