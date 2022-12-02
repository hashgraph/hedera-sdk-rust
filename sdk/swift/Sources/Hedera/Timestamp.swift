import CHedera
import Foundation
import HederaProtobufs

private let nanosPerSecond: UInt64 = 1_000_000_000

private let timeZoneUTC: TimeZone = TimeZone(abbreviation: "UTC")!

private let unixEpoch: Date = Calendar.current.date(from: DateComponents(timeZone: timeZoneUTC, year: 1970))!

/// UNIX timestamp with nanosecond precision
public struct Timestamp: Codable, Equatable, CustomStringConvertible {
    public let seconds: UInt64
    public let subSecondNanos: UInt32

    internal init(seconds: UInt64, subSecondNanos: UInt32) {
        self.seconds = seconds + UInt64(subSecondNanos) / nanosPerSecond
        self.subSecondNanos = subSecondNanos % UInt32(nanosPerSecond)
    }

    public init(fromUnixTimestampNanos nanos: UInt64) {
        self.seconds = nanos / nanosPerSecond
        self.subSecondNanos = UInt32(nanos % nanosPerSecond)
    }

    /// Convert from a ``Date`` to a `Timestamp`
    ///
    /// `Date` is stored as ``Double`` seconds, so, it may not have full precision.
    public init(from date: Date) {
        let components = Calendar.current.dateComponents([.second, .nanosecond], from: unixEpoch, to: date)

        seconds = UInt64(components.second!)
        subSecondNanos = UInt32(components.nanosecond!)
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()

        self.init(fromUnixTimestampNanos: try container.decode(UInt64.self))
    }

    // note(sr): these have the same abi lol, no "unsafe" here.
    internal init(fromCHedera timestamp: HederaTimestamp) {
        seconds = timestamp.secs
        subSecondNanos = timestamp.nanos
    }

    // todo: what do on overflow?
    public var unixTimestampNanos: UInt64 {
        seconds * nanosPerSecond + UInt64(subSecondNanos)
    }

    internal func toCHederaTimestamp() -> HederaTimestamp {
        HederaTimestamp(secs: seconds, nanos: subSecondNanos)
    }

    /// Convert from a `Timestamp` to a `Date`
    ///
    /// ``Date`` is stored as ``Double`` seconds, so, it may not have full precision.
    public func toDate() -> Date {
        let components = DateComponents(timeZone: timeZoneUTC, second: Int(seconds), nanosecond: Int(subSecondNanos))

        return Calendar.current.date(byAdding: components, to: unixEpoch)!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(self.unixTimestampNanos)
    }

    public var description: String {
        String(describing: seconds) + String(format: "%09d", subSecondNanos)
    }
}

extension Timestamp: ProtobufCodable {
    internal typealias Protobuf = Proto_Timestamp

    internal init(fromProtobuf proto: Protobuf) {
        self.init(seconds: UInt64(proto.seconds), subSecondNanos: UInt32(proto.nanos))
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.seconds = Int64(seconds)
            proto.nanos = Int32(subSecondNanos)
        }
    }
}
