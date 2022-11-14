internal struct NameMap: ExpressibleByDictionaryLiteral {
    internal typealias Key = Int32

    internal typealias Value = String

    internal init(dictionaryLiteral elements: (Key, Value)...) {
        var left: [Int32] = []
        var right: [String] = []
        for element in elements {
            left.append(element.0)
            right.append(element.1)
        }

        self.left = left
        self.right = right
    }

    private let left: [Int32]
    private let right: [String]

    internal subscript(value: Int32) -> String? {
        self.left.firstIndex(of: value).map { right[$0] }
    }

    internal subscript(name: String) -> Int32? {
        self.right.firstIndex(of: name).map { left[$0] }
    }
}
