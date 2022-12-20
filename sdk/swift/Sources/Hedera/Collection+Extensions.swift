extension Collection {
    internal subscript(safe index: Index) -> Element? {
        indices.contains(index) ? self[index] : nil
    }

    internal subscript(safe range: Range<Index>) -> SubSequence? {
        contains(range: range) ? self[range] : nil
    }

    internal func contains(range: Range<Index>) -> Bool {
        indices.contains(range.lowerBound) && indices.contains(range.upperBound)
    }
}
