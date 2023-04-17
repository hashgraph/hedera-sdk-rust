/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

import Foundation

private func hexVal(_ char: UInt8) -> UInt8? {
    // this would be a very clean function if swift had a way of doing ascii-charcter literals, but it can't.
    let ascii0: UInt8 = 0x30
    let ascii9: UInt8 = ascii0 + 9
    let asciiUppercaseA: UInt8 = 0x41
    let asciiUppercaseF: UInt8 = 0x46
    let asciiLowercaseA: UInt8 = asciiUppercaseA | 0x20
    let asciiLowercaseF: UInt8 = asciiUppercaseF | 0x20
    switch char {
    case ascii0...ascii9:
        return char - ascii0
    case asciiUppercaseA...asciiUppercaseF:
        return char - asciiUppercaseA + 10
    case asciiLowercaseA...asciiLowercaseF:
        return char - asciiLowercaseA + 10
    default:
        return nil
    }
}

extension Data {
    // this copies
    internal func safeSubdata(in range: Range<Self.Index>) -> Data? {
        return contains(range: range) ? self.subdata(in: range) : nil
    }

    internal func hexStringEncoded() -> String {
        String(
            reduce(into: "".unicodeScalars) { result, value in
                result.append(Self.hexAlphabet[Int(value / 0x10)])
                result.append(Self.hexAlphabet[Int(value % 0x10)])
            })
    }

    internal init?<S: StringProtocol>(hexEncoded: S) {
        let chars = Array(hexEncoded.utf8)
        // note: hex check is done character by character
        let count = chars.count

        guard count % 2 == 0 else {
            return nil
        }

        var arr: [UInt8] = Array()
        arr.reserveCapacity(count / 2)

        for idx in stride(from: 0, to: hexEncoded.count, by: 2) {
            // swiftlint complains about the length of these if they're less than 4 characters
            // that'd be fine and all, but `low` is still only 3 characters.
            guard let highNibble = hexVal(UInt8(chars[idx])), let lowNibble = hexVal(UInt8(chars[idx + 1])) else {
                return nil
            }

            arr.append(highNibble << 4 | lowNibble)
        }

        self.init(arr)
    }

    private static let hexAlphabet = Array("0123456789abcdef".unicodeScalars)
}

extension Data {
    internal func withUnsafeTypedBytes<R>(_ body: (UnsafeBufferPointer<UInt8>) throws -> R) rethrows -> R {
        try self.withUnsafeBytes { pointer in
            try body(pointer.bindMemory(to: UInt8.self))
        }
    }

    internal mutating func withUnsafeMutableTypedBytes<R>(_ body: (UnsafeMutableBufferPointer<UInt8>) throws -> R)
        rethrows -> R
    {
        try self.withUnsafeMutableBytes { pointer in
            try body(pointer.bindMemory(to: UInt8.self))
        }
    }
}

extension Data {
    internal static func randomData(withLength length: Int) -> Self {
        Self((0..<length).map { _ in UInt8.random(in: 0...0xff) })
    }
}

extension Data {
    internal func split(at middle: Index) -> (SubSequence, SubSequence)? {
        guard let index = index(startIndex, offsetBy: middle, limitedBy: endIndex) else {
            return nil
        }

        // note: neither of these operations can cause issues because `startIndex <= index <= endIndex`
        return (self[..<index], self[index...])
    }

    /// Slice this data using *sane* ranges
    ///
    /// Example:
    /// ```swift
    /// // gives the equivalent of Data([2, 3])
    /// let tmp = Data([1, 2, 3])[slicing: 1..<3]!
    /// // gives the equivalent of Data([2])
    /// let tmp2 = tmp[slicing: 0..<1]!
    /// precondition(tmp2[slicing: 1..<2] == nil)
    /// ```
    internal subscript(slicing range: Range<Index>) -> SubSequence? {
        guard let startIndex = index(startIndex, offsetBy: range.lowerBound, limitedBy: endIndex) else {
            return nil
        }

        guard let endIndex = index(startIndex, offsetBy: range.count, limitedBy: endIndex) else {
            return nil
        }

        return self[startIndex..<endIndex]
    }

    /// Slice this data using *sane* ranges
    internal subscript(slicing range: PartialRangeFrom<Index>) -> SubSequence? {
        guard let startIndex = index(startIndex, offsetBy: range.lowerBound, limitedBy: endIndex) else {
            return nil
        }

        return self[startIndex...]
    }

    /// Slice this data using *sane* ranges
    internal subscript(slicing range: PartialRangeUpTo<Index>) -> SubSequence? {
        guard let endIndex = index(startIndex, offsetBy: range.upperBound, limitedBy: endIndex) else {
            return nil
        }

        return self[..<endIndex]
    }

    internal subscript(at index: Index) -> Element? {
        guard let index = self.index(startIndex, offsetBy: index, limitedBy: endIndex) else {
            return nil
        }

        return self[index]
    }
}
