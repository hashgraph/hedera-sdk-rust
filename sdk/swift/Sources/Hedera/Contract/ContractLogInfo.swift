import Foundation
import HederaProtobufs

/// The log information for an event returned by a smart contract function call.
/// One function call may return several such events.
public struct ContractLogInfo: Equatable {
    /// Address of the contract that emitted the event.
    public let contractId: ContractId

    /// Bloom filter for this log.
    public let bloom: Data

    /// A list of topics this log is relevent to.
    public let topics: [Data]

    /// The log's data payload.
    public let data: Data

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension ContractLogInfo: Codable {
    private enum CodingKeys: CodingKey {
        case contractId
        case bloom
        case topics
        case data
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        contractId = try container.decode(.contractId)
        bloom = try Data(base64Encoded: container.decode(String.self, forKey: .bloom))!
        topics = try container.decode([String].self, forKey: .topics).map { Data(base64Encoded: $0)! }
        data = try Data(base64Encoded: container.decode(String.self, forKey: .data))!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(contractId, forKey: .contractId)
        try container.encode(bloom.base64EncodedString(), forKey: .bloom)
        try container.encode(topics.map { $0.base64EncodedString() }, forKey: .topics)
        try container.encode(data.base64EncodedString(), forKey: .data)
    }
}

extension ContractLogInfo: TryProtobufCodable {
    typealias Protobuf = Proto_ContractLoginfo

    init(fromProtobuf proto: Protobuf) throws {
        self.init(
            contractId: try .fromProtobuf(proto.contractID),
            bloom: proto.bloom,
            topics: proto.topic,
            data: proto.data
        )
    }

    func toProtobuf() -> Protobuf {
        .with { proto in
            proto.contractID = contractId.toProtobuf()
            proto.bloom = bloom
            proto.topic = topics
            proto.data = data
        }
    }
}
