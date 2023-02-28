import Foundation

public struct Duration: Codable {
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

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()

        seconds = try container.decode(UInt64.self)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(self.seconds)
    }
}
