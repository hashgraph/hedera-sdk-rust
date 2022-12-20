import Foundation

public struct EvmAddress: CustomStringConvertible, LosslessStringConvertible, ExpressibleByStringLiteral, Hashable {
    internal let data: Data

    internal init(_ data: Data) throws {
        guard data.count == 20 else {
            throw HError(kind: .basicParse, description: "expected evm address to have 20 bytes, it had \(data.count)")
        }

        self.data = data
    }

    internal init<S: StringProtocol>(parsing description: S) throws {
        guard let description = description.stripPrefix("0x") else {
            throw HError(kind: .basicParse, description: "expected evm address to have `0x` prefix")
        }

        guard let bytes = Data(hexEncoded: description) else {
            // todo: better error message
            throw HError(kind: .basicParse, description: "invalid evm address")
        }

        try self.init(bytes)
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }

    public static func fromString(_ description: String) throws -> Self {
        try Self(parsing: description)
    }

    public static func fromBytes(_ data: Data) throws -> Self {
        try Self(data)
    }

    public var description: String {
        "0x\(data.hexStringEncoded())"
    }

    public func toString() -> String {
        description
    }

    public func toBytes() -> Data {
        data
    }
}

extension EvmAddress: Codable {
    public init(from decoder: Decoder) throws {
        try self.init(parsing: decoder.singleValueContainer().decode(String.self))
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }
}
