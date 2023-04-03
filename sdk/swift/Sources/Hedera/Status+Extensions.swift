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
