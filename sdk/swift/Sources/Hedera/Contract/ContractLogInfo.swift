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
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension ContractLogInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_ContractLoginfo

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            contractId: try .fromProtobuf(proto.contractID),
            bloom: proto.bloom,
            topics: proto.topic,
            data: proto.data
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.contractID = contractId.toProtobuf()
            proto.bloom = bloom
            proto.topic = topics
            proto.data = data
        }
    }
}
