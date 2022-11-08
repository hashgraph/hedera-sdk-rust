import Foundation

public struct Duration: Codable {
    public let seconds: UInt64

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()

        seconds = try container.decode(UInt64.self)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(self.seconds)
    }
}
