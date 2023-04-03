import Foundation
import HederaProtobufs

public struct Duration: Sendable {
    public let seconds: UInt64

    public init(seconds: UInt64) {
        self.seconds = seconds
    }

    public static func days(_ days: UInt64) -> Self {
        .hours(days * 24)
    }

    public static func hours(_ hours: UInt64) -> Self {
        .minutes(hours * 60)
    }

    public static func minutes(_ minutes: UInt64) -> Self {
        .seconds(minutes * 60)
    }

    public static func seconds(_ seconds: UInt64) -> Self {
        Self(seconds: seconds)
    }
}

extension Duration: ProtobufCodable {
    internal typealias Protobuf = Proto_Duration

    internal init(protobuf proto: Protobuf) {
        seconds = UInt64(proto.seconds)
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in proto.seconds = Int64(seconds) }
    }
}
