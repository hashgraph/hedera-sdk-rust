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

private func hexVal(_ ch: UInt8) -> UInt8? {
    // this would be a very clean function if swift had a way of doing ascii-charcter literals, but it can't.
    let ascii0: UInt8 = 0x30
    let ascii9: UInt8 = ascii0 + 9
    let asciiUppercaseA: UInt8 = 0x41
    let asciiUppercaseF: UInt8 = 0x46
    let asciiLowercaseA: UInt8 = asciiUppercaseA | 0x20
    let asciiLowercaseF: UInt8 = asciiUppercaseF | 0x20
    switch ch {
    case ascii0...ascii9:
        return ch - ascii0
    case asciiUppercaseA...asciiUppercaseF:
        return ch - asciiUppercaseA + 10
    case asciiLowercaseA...asciiLowercaseF:
        return ch - asciiLowercaseA + 10
    default:
        return nil
    }
}

internal extension Data {
    private static let hexAlphabet = Array("0123456789abcdef".unicodeScalars)
     func hexStringEncoded() -> String {
        String(reduce(into: "".unicodeScalars) { result, value in
            result.append(Self.hexAlphabet[Int(value / 0x10)])
            result.append(Self.hexAlphabet[Int(value % 0x10)])
        })
    }

     init?(hexEncoded: String) {
        let chars = Array(hexEncoded.utf8)
        // note: hex check is done character by character
        let count = chars.count
        guard count % 2 == 0 else { return nil }
        var arr: [UInt8] = Array()
        arr.reserveCapacity(count / 2)

        for idx in stride(from: 0, to: hexEncoded.count, by: 2) {
            guard let hi = hexVal(UInt8(chars[idx])) else { return nil }
            guard let lo = hexVal(UInt8(chars[idx + 1])) else { return nil }
            arr.append(hi << 4 | lo)
        }


        self.init(arr)
    }
}