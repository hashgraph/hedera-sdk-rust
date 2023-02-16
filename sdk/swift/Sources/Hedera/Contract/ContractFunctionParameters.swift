/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

// can't get around the file length?
// swiftlint:disable file_length

import Foundation
import NumberKit

private struct Argument {
    fileprivate let typeName: String
    fileprivate let value: Data
    fileprivate let dynamic: Bool

    fileprivate static func typeName<I: FixedWidthInteger & UnsignedInteger>(for: I.Type, _ bits: Int) -> String {
        "uint\(bits)"
    }

    fileprivate static func typeName<I: FixedWidthInteger & SignedInteger>(for: I.Type, _ bits: Int) -> String {
        "int\(bits)"
    }

    fileprivate static func intTypeName<I: SignedInteger>(for: I.Type, _ bits: Int, signed: Bool) -> String {
        (signed ? "int" : "uint") + String(describing: bits)
    }

    fileprivate static func array(_ arr: [Data], _ typeName: String) -> Self {
        precondition(arr.allSatisfy { $0.count == 32 })

        return Self(
            typeName: "\(typeName)[]",
            value: Int64(arr.count).toDataWithPadding32() + arr.joined(),
            dynamic: true
        )
    }

    fileprivate static func array(_ arr: [Self], _ typeName: String) -> Self {
        precondition(arr.allSatisfy { !$0.dynamic && $0.typeName == typeName })
        return array(arr.map { $0.value }, typeName)
    }

    fileprivate static func array<I: FixedWidthInteger & UnsignedInteger>(_ arr: [I], _ bits: Int) -> Self {
        array(arr.map { int($0, bits) }, typeName(for: I.self, bits))
    }

    fileprivate static func array<I: FixedWidthInteger & SignedInteger>(_ arr: [I], _ bits: Int) -> Self {
        array(arr.map { int($0, bits) }, typeName(for: I.self, bits))
    }

    fileprivate static func array(_ arr: [BigInt], _ bits: Int, signed: Bool) -> Self {
        array(
            arr.map { int($0, bits, signed: signed) }, intTypeName(for: BigInt.self, bits, signed: signed))
    }

    fileprivate static func dynArray(_ arr: [Data], _ typeName: String) -> Self {

        let offsetsLen = arr.count

        var data = Data()

        data += UInt64(arr.count).toDataWithPadding32()

        var currentOffset = offsetsLen * 32

        for item in arr {
            data += UInt64(currentOffset).toDataWithPadding32()
            currentOffset += item.count
        }

        data += arr.joined()

        return Self(typeName: "\(typeName)[]", value: data, dynamic: true)
    }

    fileprivate static func int<I: FixedWidthInteger & UnsignedInteger>(_ num: I, _ bits: Int) -> Self {
        let offset = (I.bitWidth - bits) / 8

        let value = leftPad32Bytes(num.bigEndianBytes.suffix(from: offset), negative: false)

        return Self(typeName: "uint\(bits)", value: value, dynamic: false)
    }

    fileprivate static func int<I: FixedWidthInteger & SignedInteger>(_ num: I, _ bits: Int) -> Self {
        let offset = (I.bitWidth - bits) / 8

        let value = leftPad32Bytes(num.bigEndianBytes.suffix(from: offset), negative: num < 0)

        return Self(typeName: "int\(bits)", value: value, dynamic: false)
    }

    fileprivate static func int(_ num: BigInt, _ bits: Int, signed: Bool) -> Self {
        precondition(signed || !num.isNegative)
        let typeName = (signed ? "int" : "uint") + String(describing: bits)

        let offset = max(num.bitWidth - bits, 0) / 8

        let value = leftPad32Bytes(num.toBigEndianBytes().suffix(from: offset), negative: signed && num.isNegative)

        return Self(typeName: typeName, value: value, dynamic: false)
    }

    fileprivate static func fixedBytes(_ bytes: Data, _ expectedCount: Int) -> Self {
        precondition(expectedCount <= 32)

        let typeName = "bytes\(expectedCount)"

        precondition(
            bytes.count == expectedCount, "expected \(typeName), but was provided a value of bytes\(bytes.count)")

        return Self(typeName: typeName, value: rightPad32Bytes(bytes), dynamic: false)
    }

    fileprivate static func bytes(_ bytes: Data) -> Self {
        Self(
            typeName: "bytes",
            value: Int64(bytes.count).toDataWithPadding32() + rightPad32Bytes(bytes),
            dynamic: true
        )
    }

    fileprivate static func string(_ value: String) -> Self {
        let value = value.data(using: .utf8)!

        return Self(
            typeName: "string",
            value: Int64(value.count).toDataWithPadding32() + rightPad32Bytes(value),
            dynamic: true
        )
    }

    fileprivate static func address<S: StringProtocol>(_ value: S) -> Self {
        return Self(
            typeName: "address",
            // we intentionally want to fatal error if this happens.
            // swiftlint:disable:next force_try
            value: leftPad32Bytes(try! decodeAddress(from: value).data, negative: false),
            dynamic: false
        )
    }
}

// swiftlint:disable:next type_body_length
public final class ContractFunctionParameters {
    private var args: [Argument]

    public init() {
        args = []
    }

    @discardableResult
    private func add(_ arg: Argument) -> Self {
        self.args.append(arg)

        return self
    }

    /// Add a solidity `string`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addString(_ value: String) -> Self {
        add(.string(value))
    }

    /// Add a solidity `string[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addStringArray(_ values: [String]) -> Self {
        add(.dynArray(values.map { $0.data(using: .utf8)! }, "string"))
    }

    /// Add a solidity `bytes`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addBytes(_ value: Data) -> Self {
        add(.bytes(value))
    }

    /// Add a solidity `bytes[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addBytesArray(_ values: [Data]) -> Self {
        add(.dynArray(values, "bytes"))
    }

    /// Add a solidity `bytes32`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addBytes32(_ value: Data) -> Self {
        add(.fixedBytes(value, 32))
    }

    /// Add a solidity `bytes32[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addBytes32Array(_ value: [Data]) -> Self {
        add(.array(value.map { Argument.fixedBytes($0, 32).value }, "bytes32"))
    }

    /// Add a solidity `bool`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addBool(_ value: Bool) -> Self {
        addUint8(value ? 1 : 0)
    }

    /// Add a solidity `uint8`.
    ///
    /// Solidity `uint8`s are stored in memory with 32 bytes, a value of this type is quite wasteful.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint8(_ num: UInt8) -> Self {
        add(.int(num, 8))
    }

    /// Add a solidity `int8`.
    ///
    /// Solidity `int8`s are stored in memory with 32 bytes, a value of this type is quite wasteful.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt8(_ num: Int8) -> Self {
        add(.int(num, 8))
    }

    /// Add a solidity `uint16`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint16(_ num: UInt16) -> Self {
        add(.int(num, 16))
    }

    /// Add a solidity `int16`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt16(_ num: Int16) -> Self {
        add(.int(num, 16))
    }

    /// Add a solidity `uint24`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint24(_ num: UInt32) -> Self {
        add(.int(num, 24))
    }

    /// Add a solidity `int24`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt24(_ num: Int32) -> Self {
        add(.int(num, 24))
    }

    /// Add a solidity `uint32`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint32(_ num: UInt32) -> Self {
        add(.int(num, 32))
    }

    /// Add a solidity `int32`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt32(_ num: Int32) -> Self {
        add(.int(num, 32))
    }

    /// Add a solidity `uint40`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint40(_ num: UInt64) -> Self {
        add(.int(num, 40))
    }

    /// Add a solidity `int40`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt40(_ num: Int64) -> Self {
        add(.int(num, 40))
    }

    /// Add a solidity `uint48`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint48(_ num: UInt64) -> Self {
        add(.int(num, 48))
    }

    /// Add a solidity `int48`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt48(_ num: Int64) -> Self {
        add(.int(num, 48))
    }

    /// Add a solidity `uint56`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint56(_ num: UInt64) -> Self {
        add(.int(num, 56))
    }

    /// Add a solidity `int56`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt56(_ num: Int64) -> Self {
        add(.int(num, 56))
    }

    /// Add a solidity `uint64`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint64(_ num: UInt64) -> Self {
        add(.int(num, 64))
    }

    /// Add a solidity `int64`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt64(_ num: Int64) -> Self {
        add(.int(num, 64))
    }

    /// Add a solidity `uint72`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint72(_ num: BigInt) -> Self {
        add(.int(num, 72, signed: false))
    }

    /// Add a solidity `int72`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt72(_ num: BigInt) -> Self {
        add(.int(num, 72, signed: true))
    }

    /// Add a solidity `uint80`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint80(_ num: BigInt) -> Self {
        add(.int(num, 80, signed: false))
    }

    /// Add a solidity `int80`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt80(_ num: BigInt) -> Self {
        add(.int(num, 80, signed: true))
    }

    /// Add a solidity `uint88`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint88(_ num: BigInt) -> Self {
        add(.int(num, 88, signed: false))
    }

    /// Add a solidity `int88`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt88(_ num: BigInt) -> Self {
        add(.int(num, 88, signed: true))
    }

    /// Add a solidity `uint96`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint96(_ num: BigInt) -> Self {
        add(.int(num, 96, signed: false))
    }

    /// Add a solidity `int96`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt96(_ num: BigInt) -> Self {
        add(.int(num, 96, signed: true))
    }

    /// Add a solidity `uint104`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint104(_ num: BigInt) -> Self {
        add(.int(num, 104, signed: false))
    }

    /// Add a solidity `int104`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt104(_ num: BigInt) -> Self {
        add(.int(num, 104, signed: true))
    }

    /// Add a solidity `uint112`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint112(_ num: BigInt) -> Self {
        add(.int(num, 112, signed: false))
    }

    /// Add a solidity `int112`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt112(_ num: BigInt) -> Self {
        add(.int(num, 112, signed: true))
    }

    /// Add a solidity `uint128`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint128(_ num: BigInt) -> Self {
        add(.int(num, 128, signed: false))
    }

    /// Add a solidity `int128`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt128(_ num: BigInt) -> Self {
        add(.int(num, 128, signed: true))
    }

    /// Add a solidity `uint136`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint136(_ num: BigInt) -> Self {
        add(.int(num, 136, signed: false))
    }

    /// Add a solidity `int136`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt136(_ num: BigInt) -> Self {
        add(.int(num, 136, signed: true))
    }

    /// Add a solidity `uint144`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint144(_ num: BigInt) -> Self {
        add(.int(num, 144, signed: false))
    }

    /// Add a solidity `int144`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt144(_ num: BigInt) -> Self {
        add(.int(num, 144, signed: true))
    }

    /// Add a solidity `uint152`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint152(_ num: BigInt) -> Self {
        add(.int(num, 152, signed: false))
    }

    /// Add a solidity `int152`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt152(_ num: BigInt) -> Self {
        add(.int(num, 152, signed: true))
    }

    /// Add a solidity `uint160`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint160(_ num: BigInt) -> Self {
        add(.int(num, 160, signed: false))
    }

    /// Add a solidity `int160`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt160(_ num: BigInt) -> Self {
        add(.int(num, 160, signed: true))
    }

    /// Add a solidity `uint168`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint168(_ num: BigInt) -> Self {
        add(.int(num, 168, signed: false))
    }

    /// Add a solidity `int168`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt168(_ num: BigInt) -> Self {
        add(.int(num, 168, signed: true))
    }

    /// Add a solidity `uint176`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint176(_ num: BigInt) -> Self {
        add(.int(num, 176, signed: false))
    }

    /// Add a solidity `int176`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt176(_ num: BigInt) -> Self {
        add(.int(num, 176, signed: true))
    }

    /// Add a solidity `uint184`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint184(_ num: BigInt) -> Self {
        add(.int(num, 184, signed: false))
    }

    /// Add a solidity `int184`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt184(_ num: BigInt) -> Self {
        add(.int(num, 184, signed: true))
    }

    /// Add a solidity `uint192`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint192(_ num: BigInt) -> Self {
        add(.int(num, 192, signed: false))
    }

    /// Add a solidity `int192`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt192(_ num: BigInt) -> Self {
        add(.int(num, 192, signed: true))
    }

    /// Add a solidity `uint200`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint200(_ num: BigInt) -> Self {
        add(.int(num, 200, signed: false))
    }

    /// Add a solidity `int200`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt200(_ num: BigInt) -> Self {
        add(.int(num, 200, signed: true))
    }

    /// Add a solidity `uint208`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint208(_ num: BigInt) -> Self {
        add(.int(num, 208, signed: false))
    }

    /// Add a solidity `int208`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt208(_ num: BigInt) -> Self {
        add(.int(num, 208, signed: true))
    }

    /// Add a solidity `uint216`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint216(_ num: BigInt) -> Self {
        add(.int(num, 216, signed: false))
    }

    /// Add a solidity `int216`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt216(_ num: BigInt) -> Self {
        add(.int(num, 216, signed: true))
    }

    /// Add a solidity `uint224`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint224(_ num: BigInt) -> Self {
        add(.int(num, 224, signed: false))
    }

    /// Add a solidity `int224`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt224(_ num: BigInt) -> Self {
        add(.int(num, 224, signed: true))
    }

    /// Add a solidity `uint232`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint232(_ num: BigInt) -> Self {
        add(.int(num, 232, signed: false))
    }

    /// Add a solidity `int232`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt232(_ num: BigInt) -> Self {
        add(.int(num, 232, signed: true))
    }

    /// Add a solidity `uint240`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint240(_ num: BigInt) -> Self {
        add(.int(num, 240, signed: false))
    }

    /// Add a solidity `int240`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt240(_ num: BigInt) -> Self {
        add(.int(num, 240, signed: true))
    }

    /// Add a solidity `uint248`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint248(_ num: BigInt) -> Self {
        add(.int(num, 248, signed: false))
    }

    /// Add a solidity `int248`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt248(_ num: BigInt) -> Self {
        add(.int(num, 248, signed: true))
    }

    /// Add a solidity `uint256`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint256(_ num: BigInt) -> Self {
        add(.int(num, 256, signed: false))
    }

    /// Add a solidity `int256`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt256(_ num: BigInt) -> Self {
        add(.int(num, 256, signed: true))
    }

    /// Add a solidity `uint8[]`.
    ///
    /// Solidity `uint8`s are stored in memory with 32 bytes, a value of this type is quite wasteful.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint8Array(_ nums: [UInt8]) -> Self {
        add(.array(nums, 8))
    }

    /// Add a solidity `int8[]`.
    ///
    /// Solidity `int8`s are stored in memory with 32 bytes, a value of this type is quite wasteful.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt8Array(_ nums: [Int8]) -> Self {
        add(.array(nums, 8))
    }

    /// Add a solidity `uint16[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint16Array(_ nums: [UInt16]) -> Self {
        add(.array(nums, 16))
    }

    /// Add a solidity `int16[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt16Array(_ nums: [Int16]) -> Self {
        add(.array(nums, 16))
    }

    /// Add a solidity `uint24[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint24Array(_ nums: [UInt32]) -> Self {
        add(.array(nums, 24))
    }

    /// Add a solidity `int24[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt24Array(_ nums: [Int32]) -> Self {
        add(.array(nums, 24))
    }

    /// Add a solidity `uint32[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint32Array(_ nums: [UInt32]) -> Self {
        add(.array(nums, 32))
    }

    /// Add a solidity `int32[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt32Array(_ nums: [Int32]) -> Self {
        add(.array(nums, 32))
    }

    /// Add a solidity `uint40[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint40Array(_ nums: [UInt64]) -> Self {
        add(.array(nums, 40))
    }

    /// Add a solidity `int40[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt40Array(_ nums: [Int64]) -> Self {
        add(.array(nums, 40))
    }

    /// Add a solidity `uint48[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint48Array(_ nums: [UInt64]) -> Self {
        add(.array(nums, 48))
    }

    /// Add a solidity `int48[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt48Array(_ nums: [Int64]) -> Self {
        add(.array(nums, 48))
    }

    /// Add a solidity `uint56[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint56Array(_ nums: [UInt64]) -> Self {
        add(.array(nums, 56))
    }

    /// Add a solidity `int56[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt56Array(_ nums: [Int64]) -> Self {
        add(.array(nums, 56))
    }

    /// Add a solidity `uint64[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint64Array(_ nums: [UInt64]) -> Self {
        add(.array(nums, 64))
    }

    /// Add a solidity `int64[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt64Array(_ nums: [Int64]) -> Self {
        add(.array(nums, 64))
    }

    /// Add a solidity `uint72[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint72Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 72, signed: false))
    }

    /// Add a solidity `int72[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt72Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 72, signed: true))
    }

    /// Add a solidity `uint80[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint80Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 80, signed: false))
    }

    /// Add a solidity `int80[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt80Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 80, signed: true))
    }

    /// Add a solidity `uint88[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint88Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 88, signed: false))
    }

    /// Add a solidity `int88[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt88Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 88, signed: true))
    }

    /// Add a solidity `uint96[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint96Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 96, signed: false))
    }

    /// Add a solidity `int96[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt96Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 96, signed: true))
    }

    /// Add a solidity `uint104[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint104Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 104, signed: false))
    }

    /// Add a solidity `int104[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt104Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 104, signed: true))
    }

    /// Add a solidity `uint112[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint112Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 112, signed: false))
    }

    /// Add a solidity `int112[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt112Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 112, signed: true))
    }

    /// Add a solidity `uint128[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint128Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 128, signed: false))
    }

    /// Add a solidity `int128[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt128Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 128, signed: true))
    }

    /// Add a solidity `uint136[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint136Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 136, signed: false))
    }

    /// Add a solidity `int136[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt136Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 136, signed: true))
    }

    /// Add a solidity `uint144[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint144Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 144, signed: false))
    }

    /// Add a solidity `int144[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt144Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 144, signed: true))
    }

    /// Add a solidity `uint152[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint152Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 152, signed: false))
    }

    /// Add a solidity `int152[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt152Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 152, signed: true))
    }

    /// Add a solidity `uint160[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint160Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 160, signed: false))
    }

    /// Add a solidity `int160[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt160Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 160, signed: true))
    }

    /// Add a solidity `uint168[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint168Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 168, signed: false))
    }

    /// Add a solidity `int168[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt168Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 168, signed: true))
    }

    /// Add a solidity `uint176[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint176Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 176, signed: false))
    }

    /// Add a solidity `int176[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt176Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 176, signed: true))
    }

    /// Add a solidity `uint184[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint184Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 184, signed: false))
    }

    /// Add a solidity `int184[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt184Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 184, signed: true))
    }

    /// Add a solidity `uint192[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint192Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 192, signed: false))
    }

    /// Add a solidity `int192[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt192Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 192, signed: true))
    }

    /// Add a solidity `uint200[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint200Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 200, signed: false))
    }

    /// Add a solidity `int200[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt200Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 200, signed: true))
    }

    /// Add a solidity `uint208[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint208Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 208, signed: false))
    }

    /// Add a solidity `int208[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt208Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 208, signed: true))
    }

    /// Add a solidity `uint216[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint216Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 216, signed: false))
    }

    /// Add a solidity `int216[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt216Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 216, signed: true))
    }

    /// Add a solidity `uint224[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint224Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 224, signed: false))
    }

    /// Add a solidity `int224[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt224Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 224, signed: true))
    }

    /// Add a solidity `uint232[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint232Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 232, signed: false))
    }

    /// Add a solidity `int232[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt232Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 232, signed: true))
    }

    /// Add a solidity `uint240[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint240Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 240, signed: false))
    }

    /// Add a solidity `int240[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt240Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 240, signed: true))
    }

    /// Add a solidity `uint248[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint248Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 248, signed: false))
    }

    /// Add a solidity `int248[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt248Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 248, signed: true))
    }

    /// Add a solidity `uint256[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addUint256Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 256, signed: false))
    }

    /// Add a solidity `int256[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addInt256Array(_ nums: [BigInt]) -> Self {
        add(.array(nums, 256, signed: true))
    }

    /// Add a solidity `address`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addAddress<S: StringProtocol>(_ address: S) -> Self {
        add(.address(address))
    }

    /// Add a solidity `address[]`.
    ///
    /// - Returns: `self`
    @discardableResult
    public func addAddressArray<S: StringProtocol>(_ addresses: [S]) -> Self {
        add(.array(addresses.map { .address($0) }, "address"))
    }

    /// Add a solidity `function`.
    ///
    /// - Returns: `self`
    public func addFunction<S: StringProtocol>(_ address: S, _ selector: Data) -> Self {
        precondition(selector.count == 4, "function selectors must be 4 bytes or 8 hex chars")

        return add(
            Argument(
                typeName: "function",
                // we intentionally want to fatal error if this happens.
                // swiftlint:disable:next force_try
                value: rightPad32Bytes(try! decodeAddress(from: address).data + selector),
                dynamic: false
            )
        )
    }

    /// Add a solidity `function`.
    ///
    /// - Returns: `self`
    public func addFunction<S: StringProtocol>(_ address: S, _ selector: ContractFunctionSelector)
        -> Self
    {
        return addFunction(address, selector.finish())
    }

    /// Get the encoding of the currently added parameters as bytes.
    ///
    /// You can continue adding arguments after calling this function.
    ///
    /// - Returns: `self`
    public func toBytes(_ funcName: String? = nil) -> Data {
        var dynamicOffset = args.count * 32

        var staticArgs = Data()
        var dynamicArgs = Data()

        let selector = funcName.map { ContractFunctionSelector($0) }

        for arg in args {
            selector?.addParamType(arg.typeName)

            if arg.dynamic {
                // dynamic arguments supply their offset in value position and append their data at
                // that offset
                staticArgs += UInt64(dynamicOffset).toDataWithPadding32()
                dynamicArgs += arg.value
                dynamicOffset += arg.value.count
            } else {
                staticArgs += arg.value
            }
        }

        if let selector = selector {
            staticArgs.insert(contentsOf: selector.finish(), at: 0)
        }

        return staticArgs + dynamicArgs
    }
}

private func decodeAddress<S: StringProtocol>(from description: S) throws -> EvmAddress {
    let description = description.stripPrefix("0x") ?? description[...]

    guard let bytes = Data(hexEncoded: description) else {
        // todo: better error message
        throw HError(kind: .basicParse, description: "invalid evm address")
    }

    return try EvmAddress(bytes)

}

private func leftPad32Bytes(_ bytes: Data, negative: Bool) -> Data {
    Data(repeating: negative ? 0xff : 0x00, count: 32 - bytes.count) + bytes
}

private func rightPad32Bytes(_ bytes: Data) -> Data {
    if bytes.count % 32 == 0 {
        return bytes
    }

    return bytes + Data(repeating: 0, count: 32 - (bytes.count % 32))
}

extension FixedWidthInteger {
    fileprivate func toDataWithPadding32() -> Data {
        leftPad32Bytes(self.bigEndianBytes, negative: self < 0)
    }
}
