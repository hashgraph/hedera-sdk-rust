import Foundation

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

    internal func splitFirst() -> (first: Element, rest: SubSequence)? {
        first.map { ($0, dropFirst(1)) }
    }

    internal func splitLast() -> (last: Element, rest: SubSequence)? {
        last.map { ($0, dropLast(1)) }
    }

    internal func splitAt(_ middle: Index) -> (SubSequence, SubSequence)? {
        guard self.indices.contains(middle) else {
            return nil
        }

        return (self[..<middle], self[middle...])
    }
}

internal enum Ordering: Comparable {
    case less
    case equal
    case greater
}

extension RandomAccessCollection where Element: Comparable {
    /// Searches for an element equal to the given element.
    ///
    /// If a match is found, the index of that match is returned.
    /// If multiple matches are found, the index of any matches may be returned.
    ///
    /// > Note: time complexity: O(log n)
    ///
    /// > Important: The collection must be sorted.
    internal func binarySearch(by predicate: (Element) throws -> Ordering) rethrows -> Index? {
        var size = count
        var left = startIndex
        var right = endIndex

        while left < right {
            let mid = index(left, offsetBy: size / 2)

            switch try predicate(self[mid]) {
            case .less: left = index(after: mid)
            case .equal: return mid
            case .greater: right = mid
            }

            size = distance(from: left, to: right)
        }

        return nil
    }
}
