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

import CHedera
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

    private static let hexAlphabet = Array("0123456789abcdef".unicodeScalars)

    // (sr): swift compiler wins the "useless acl vs explicit acl debate
    internal func hexStringEncoded() -> String {
        String(
            reduce(into: "".unicodeScalars) { result, value in
                result.append(Self.hexAlphabet[Int(value / 0x10)])
                result.append(Self.hexAlphabet[Int(value % 0x10)])
            })
    }

    // (sr): swift compiler wins the "useless acl vs explicit acl debate
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

    internal static func base64Encoded(_ description: String) throws -> Self {
        guard let tmp = Self(base64Encoded: description) else {
            throw HError(kind: .basicParse, description: "Invalid base64 Data")
        }

        return tmp
    }
}

extension Data {
    internal func withUnsafeTypedBytes<R>(_ body: (UnsafeBufferPointer<UInt8>) throws -> R) rethrows -> R {
        try self.withUnsafeBytes { pointer in
            try body(pointer.bindMemory(to: UInt8.self))
        }
    }
}

extension Data.Deallocator {
    // safety: `hedera_bytes_free` needs to be called so...
    // perf: might as well enable use of the no copy constructor.
    internal static let unsafeCHederaBytesFree: Data.Deallocator = .custom { (buf, size) in
        hedera_bytes_free(buf.bindMemory(to: UInt8.self, capacity: size), size)
    }
}
