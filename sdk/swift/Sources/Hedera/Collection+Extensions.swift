extension RandomAccessCollection {
    internal subscript(safe index: Index) -> Element? {
        indices.contains(index) ? self[index] : nil
    }

    internal subscript(safe range: Range<Index>) -> SubSequence? {
        contains(range: range) ? self[range] : nil
    }

    internal func contains(range: Range<Index>) -> Bool {
        // this madness is because a `Range` doesn't actually contain the `upperBound`.
        // So, `0..<self.endIndex` *is* contained by `self` and should return `true` here.
        // but, `indices.contains(range.lowerBound) && indices.contains(range.upperBound)` would return false.
        return range.clamped(to: startIndex..<endIndex) == range
    }
}
