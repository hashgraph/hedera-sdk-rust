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

// Somehow nobody has made an RlpEncoder/Decoder that

import Foundation

internal enum Rlp {}

// swiftlint:disable nesting
extension Rlp {
    internal enum DecoderError: Error, CustomStringConvertible {
        /// Input was non-canonical (eg, a value like `8100` was found).
        case nonCanonical

        /// Found a value when we expected a list.
        case expectedList

        /// Found a list when we expected a value.
        case expectedValue

        /// Rlp data ran out of buffer.
        case tooShort

        /// a number value was too big (eg, a size > Int.maxValue)
        case valueTooLarge

        /// Expected a list of `expected` length, but it was actually `actual`
        case incorrectListCount(expected: Int, actual: Int)

        internal var description: String {
            let start = "Rlp decoding error: "
            let rest: String
            switch self {
            case .nonCanonical:
                rest = "Input was non canonical"
            case .expectedList:
                rest = "expected a list but found a value"
            case .expectedValue:
                rest = "expected a value but found a list"
            case .tooShort:
                rest = "input was shorter than required"
            case .valueTooLarge:
                rest = "an integer value was too big"
            case .incorrectListCount(let expected, let actual):
                rest = "expected a list of size \(expected), but it was actually \(actual) elements long"
            }

            return start + rest
        }
    }

    fileprivate enum Tag {
        fileprivate enum Value {
            /// Parameter value: A value in the range 0x00..<0x80
            case inline(value: UInt8)
            /// Parameter byteCount: The number of bytes that value is made up of, in the range 0..<55
            case short(byteCount: UInt8)
            /// Parameter byteCountSize: the number of bytes in `count`, in the range 0..<8.
            case long(byteCountSize: UInt8)
        }

        fileprivate enum List {
            /// Parameter byteCount: The number of bytes that the list contains (not the number of elements)
            case short(byteCount: UInt8)

            /// Parameter byteCountSize: the number of bytes in `count`, in the range 0..<8.
            case long(byteCountSize: UInt8)
        }

        case value(Value)
        case list(List)

        fileprivate static let shortMaxByteCount: Int = 55
    }

    internal struct List {
        /// Raw is Rlp encoded data inside the list, the elements of the list may be values, or lists themselves.
        ///
        /// >Note: `raw` contains the remaining elements of `self`
        fileprivate var raw: Data
    }

    // todo: make a `@resultBuilder`?
    internal struct Encoder {
        private struct List {
            let offset: Int
        }

        private var raw: Data
        private var lists: [List]
    }
}

// swiftlint:enable nesting

extension Rlp.Tag: RawRepresentable {
    internal init(rawValue: UInt8) {
        let value = rawValue

        switch value {
        case 0..<0x80:
            self = .value(.inline(value: value))
        case 0x80..<0xb8:
            self = .value(.short(byteCount: value - 0x80))
        // this range starts at 0xb8, but the value starts at 1.
        case 0xb8..<0xc0:
            self = .value(.long(byteCountSize: value - 0xb8 + 1))
        case 0xc0..<0xf8:
            self = .list(.short(byteCount: value - 0xc0))
        // 0xf8...0xff
        // this range starts at 0xf8, but the value starts at 1.
        default:
            self = .list(.long(byteCountSize: value - 0xf8 + 1))
        }
    }

    internal var rawValue: UInt8 {
        switch self {
        case .value(.inline(let value)):
            assert(value < 0x80)
            return value
        case .value(.short(let byteCount)):
            assert(byteCount <= 55)
            return 0x80 + byteCount

        case .value(.long(let byteCountSize)):
            assert((1..<8).contains(byteCountSize))
            return 0xb8 - 1 + byteCountSize

        case .list(.short(byteCount: let byteCount)):
            assert(byteCount <= 55)
            return 0xc0 + byteCount

        case .list(.long(let byteCountSize)):
            assert((1..<8).contains(byteCountSize))
            return 0xf8 - 1 + byteCountSize
        }
    }
}

internal struct AnyRlp {
    internal init(raw: Data) {
        self.raw = raw
    }

    private let raw: Data
}

internal protocol RlpDecodable {
    init(rlp: AnyRlp) throws
}

internal protocol RlpEncodable {
    func encode(to encoder: inout Rlp.Encoder)
}

extension RlpEncodable {
    internal func rlpEncoded() -> Data {
        var encoder = Rlp.Encoder()
        encode(to: &encoder)
        return encoder.output
    }
}

extension Data: RlpDecodable, RlpEncodable {
    internal init(rlp: AnyRlp) throws {
        self.init(try rlp.makeRawValue())
    }

    internal func encode(to encoder: inout Rlp.Encoder) {
        encoder.appendRawValue(self)
    }
}

extension Array: RlpDecodable where Element: RlpDecodable {
    internal init(rlp: AnyRlp) throws {
        var rlpList = try rlp.makeRawList()

        var arr: Self = []

        while let element = try rlpList.nextValue(Element.self) {
            arr.append(element)
        }

        self = arr
    }
}

extension Array: RlpEncodable where Element: RlpEncodable {
    internal func encode(to encoder: inout Rlp.Encoder) {
        encoder.startList()

        for element in self {
            encoder.append(element)
        }

        encoder.endList()
    }
}

extension Rlp.List {
    private func peek() -> AnyRlp? {
        if raw.isEmpty {
            return nil
        }

        return AnyRlp(raw: raw)
    }

    internal var isEmpty: Bool { raw.isEmpty }

    /// Returns the number of elements remaining
    ///
    /// This function is O(n) because we have to scan through the list to find the number of elements.
    internal func count() throws -> Int {
        var list = Self(raw: raw)
        var count = 0

        while try list.next() != nil {
            count += 1
        }

        return count
    }

    internal mutating func next() throws -> AnyRlp? {
        guard let rlp = self.peek() else {
            return nil
        }

        do {
            let consumed = try rlp.count()
            raw = raw[slicing: consumed...]!
        } catch {
            // ensure we only throw an error once.
            raw = Data()
            throw error
        }

        return rlp
    }

    internal mutating func nextRawList() throws -> Self? {
        guard let rlp = try self.next() else {
            return nil
        }

        return try rlp.makeRawList()
    }

    internal mutating func nextList<V>(_: V.Type) throws -> [V]? where V: RlpDecodable {
        guard let rlp = try self.next() else {
            return nil
        }

        return try .init(rlp: rlp)
    }

    internal mutating func nextValue<V>(_: V.Type) throws -> V? where V: RlpDecodable {
        guard let rlp = try self.next() else {
            return nil
        }

        return try .init(rlp: rlp)
    }
}

extension AnyRlp {
    private typealias Tag = Rlp.Tag

    internal var isList: Bool {
        // there are two ranges for lists: 0xc0..<0xf7 and 0xf7...0xff,
        // those two ranges are right next to each other, so, we can combine them.
        // And since those go _through_ the end of the range for a UInt8, we can do `$0 >= 0xc0`
        raw.first.map { $0 >= 0xc0 } ?? false
    }

    private var tag: Tag? {
        guard let value = raw.first else {
            return nil
        }

        return Tag(rawValue: value)
    }

    /// Returns the number of bytes this value requires.
    fileprivate func counts() throws -> (header: Int, data: Int) {
        guard let tag = self.tag else {
            throw Rlp.DecoderError.tooShort
        }

        let raw = raw[slicing: 1...]!

        switch tag {
        case .value(.inline): return (0, 1)
        // special case because we need to canonical check
        case .value(.short(1)):
            guard let value = raw[at: 0] else {
                throw Rlp.DecoderError.tooShort
            }

            // value should be .inline
            guard value >= 0x80 else {
                throw Rlp.DecoderError.nonCanonical
            }

            return (1, 1)

        case .value(.short(byteCount: let count)), .list(.short(byteCount: let count)):
            return (1, Int(count))

        case .value(.long(byteCountSize: let byteCountSize)), .list(.long(byteCountSize: let byteCountSize)):
            let byteCountSize = Int(byteCountSize)
            guard let (countData, rest) = raw.split(at: byteCountSize) else {
                throw Rlp.DecoderError.tooShort
            }

            let byteCount = try Self.decodeIntCount(bytes: countData)

            guard rest.count >= byteCount else {
                throw Rlp.DecoderError.tooShort
            }

            return (byteCountSize + 1, byteCount)
        }
    }

    fileprivate func count() throws -> Int {
        let (header, data) = try counts()

        return header + data
    }

    // Decode the number of bytes in a list/value, this if for `.long` tags.
    private static func decodeIntCount(bytes: Data) throws -> Int {
        do {
            // we're decoding a `.long`, an empty count would be canonically expressed via `.short(byteCount: 0)`
            // likewise a 0 count would be expressed as an empty count.
            guard let first = bytes.first, first != 0 else {
                throw Rlp.DecoderError.nonCanonical
            }
        }

        if bytes.isEmpty {
            throw Rlp.DecoderError.nonCanonical
        }

        guard Int.bitWidth >= bytes.count * 8 else {
            throw Rlp.DecoderError.valueTooLarge
        }

        let value: Int

        if bytes.count * 8 == Int.bitWidth {
            // avoid a concat if we have the exact right amount of bytes.
            value = Int(bigEndianBytes: Data(bytes))!
        } else {
            // we need to fill the start bytes with zeros to pad out the size...
            let repeatTimes = Int.bitWidth / 8 - bytes.count
            let start = Data(repeating: 0, count: repeatTimes)
            value = Int(bigEndianBytes: start + bytes)!
        }

        // should be .short(value)
        guard value >= 56 else {
            throw Rlp.DecoderError.nonCanonical
        }

        return value
    }

    internal func makeRawValue() throws -> Data {
        guard case .value = tag else {
            throw Rlp.DecoderError.expectedValue
        }

        // data starts after the header ends, so
        let (headerCount, dataCount) = try counts()

        return raw[slicing: headerCount...]![slicing: ..<dataCount]!
    }

    internal func makeRawList() throws -> Rlp.List {
        guard case .list = tag else {
            throw Rlp.DecoderError.expectedList
        }

        let (headerCount, dataCount) = try counts()

        return Rlp.List(raw: raw[slicing: headerCount...]![slicing: ..<dataCount]!)
    }
}

extension Rlp.Encoder {
    internal init(buffer: Data = Data()) {
        self.raw = buffer
        self.lists = []
    }

    fileprivate mutating func appendRawValue(_ value: Data) {
        switch value.count {
        case 0:
            let tag = Rlp.Tag.value(.short(byteCount: 0))
            raw.append(tag.rawValue)
        case 1 where value.first! < 0x80:
            let tag = Rlp.Tag.value(.inline(value: value.first!))
            raw.append(tag.rawValue)

        case 1..<56:
            let tag = Rlp.Tag.value(.short(byteCount: UInt8(exactly: value.count)!))
            raw.append(tag.rawValue)
            raw.append(contentsOf: value)

        default:
            precondition(Int.bitWidth <= 64)
            // skip the bytes that start with 0 (because they waste space / are non canonical)
            let count = value.count.bigEndianBytes.drop { $0 == 0 }
            let tag = Rlp.Tag.value(.long(byteCountSize: UInt8(exactly: count.count)!))

            raw.append(tag.rawValue)
            raw.append(count)
            raw.append(value)
        }
    }

    // todo: be more efficient
    internal mutating func append<V: RlpEncodable>(_ value: V) {
        let count = self.lists.count
        value.encode(to: &self)
        precondition(self.lists.count == count, "appending an item must leave the number of lists the same")
    }

    internal mutating func appendRaw<D: Sequence>(contentsOf seq: D) where D.Element == UInt8 {
        self.raw.append(contentsOf: seq)
    }

    internal mutating func appendRaw(contentsOf array: [UInt8]) {
        self.raw.append(contentsOf: array)
    }

    internal mutating func appendRaw(_ data: Data) {
        self.raw.append(data)
    }

    internal mutating func startList() {
        self.lists.append(List(offset: raw.endIndex))
        // have to put something here, so.
        self.raw.append(0xff)
    }

    internal mutating func endList() {
        guard let list = lists.popLast() else {
            fatalError("Attempted to pop a list but there were no lists on the stack")
        }

        // we'll have always appended a single byte for the tag, but the tag doesn't count
        let listByteCount = raw.distance(from: list.offset, to: raw.endIndex) - 1
        switch listByteCount {
        case 0...Rlp.Tag.shortMaxByteCount:
            raw[list.offset] = Rlp.Tag.list(.short(byteCount: UInt8(listByteCount))).rawValue
        default:
            // we need to encode the tag and insert the length right after, unfortunately it's impossible to actually predict the label size.
            precondition(Int.bitWidth <= 64)

            // skip the bytes that start with 0 (because they waste space / are non canonical)
            let count = listByteCount.bigEndianBytes.drop { $0 == 0 }
            let tag = Rlp.Tag.list(.long(byteCountSize: UInt8(exactly: count.count)!))

            raw[list.offset] = tag.rawValue
            raw.insert(contentsOf: count, at: raw.index(after: list.offset))
        }
    }

    internal var output: Data {
        precondition(self.lists.isEmpty, "Cannot provide output with unfinished lists")

        return Data(raw)
    }
}
