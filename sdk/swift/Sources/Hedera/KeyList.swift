public struct KeyList: ExpressibleByArrayLiteral, Equatable {
    public typealias ArrayLiteralElement = Key

    public var keys: [Key]
    public var threshold: Int?

    public init(arrayLiteral elements: Key...) {
        self.init(keys: Array(elements))
    }

    internal init(keys: [Key], threshold: Int? = nil) {
        self.keys = keys
        self.threshold = threshold
    }
}

extension KeyList: Collection, RandomAccessCollection {
    public typealias Index = Int
    public typealias Element = Key

    public subscript(_ position: Int) -> Key {
        get {
            self.keys[position]
        }

        set(value) {
            self.keys[position] = value
        }
    }

    // i is *the* identifier name to use here.
    // swiftlint:disable:next identifier_name
    public func index(after i: Int) -> Int {
        self.keys.index(after: i)
    }

    public var startIndex: Int { keys.startIndex }
    public var endIndex: Int { keys.endIndex }
}

extension KeyList: Codable {
    private enum CodingKeys: CodingKey {
        case keys
        case threshold
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.init(
            keys: try container.decodeIfPresent(.keys) ?? [],
            threshold: try container.decodeIfPresent(.threshold)
        )
    }
}
