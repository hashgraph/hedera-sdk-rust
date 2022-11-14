extension Status: LosslessStringConvertible, CustomStringConvertible, ExpressibleByStringLiteral {
    public init?(_ description: String) {
        guard let value = Self.nameMap[description] else {
            return nil
        }

        self.init(rawValue: value)
    }

    public init(stringLiteral value: StringLiteralType) {
        self.init(rawValue: Self.nameMap[value]!)
    }

    public var description: String {
        Self.nameMap[rawValue] ?? "UNRECOGNIZED \(rawValue)"
    }
}

extension Status: Codable {
    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        let description = try container.decode(String.self)
        self.init(description)!
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        try container.encode(self.description)
    }
}
