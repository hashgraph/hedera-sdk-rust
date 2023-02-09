internal struct MnemonicWordList: ExpressibleByStringLiteral {
    init(stringLiteral value: StringLiteralType) {
        backingData = value
        words = value.split { $0.isNewline }
        isSorted = words.isSorted()
    }

    private let words: [Substring]
    private let backingData: String
    private let isSorted: Bool

    internal func indexOf<S: StringProtocol>(word: S) -> Int? {
        // todo: binary search if sorted
        words.firstIndex { $0 == word }
    }

    internal func contains<S: StringProtocol>(word: S) -> Bool {
        words.contains { $0 == word }
    }

    internal subscript(index: Int) -> Substring? {
        words[safe: index]
    }
}

extension Array where Element: Comparable {
    func isSorted() -> Bool {
        // empty and mono-element arrays are sorted, just,
        // by nature of there being no (other) elements.
        if self.count < 2 {
            return true
        }

        for i in 1..<count {
            let lhs = self[i - 1]
            let rhs = self[i]
            if lhs > rhs {
                return false
            }
        }

        return true
    }
}
