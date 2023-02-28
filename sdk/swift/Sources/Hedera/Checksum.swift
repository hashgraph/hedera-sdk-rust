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

public struct Checksum: LosslessStringConvertible, Hashable {
    internal let data: String

    public init?<S: StringProtocol>(_ description: S) {
        guard description.allSatisfy({ $0.isASCII && $0.isLowercase && $0.isLetter }), description.count == 5 else {
            return nil
        }

        self.data = String(description)
    }

    internal init<S: StringProtocol>(parsing description: S) throws {
        guard let tmp = Self(description) else {
            throw HError(kind: .basicParse, description: "Invalid checksum string \(description)")
        }

        self = tmp
    }

    internal init?(data: Data) {
        guard data.count == 5 else {
            return nil
        }

        let str = String(data: data, encoding: .ascii)!
        // fixme: check for ascii-alphanumeric

        self.data = str
    }

    // swift doesn't have any other way to do "fixed length array"
    // swiftlint:disable:next large_tuple
    internal init(bytes: (UInt8, UInt8, UInt8, UInt8, UInt8)) {
        // swiftlint:disable:next identifier_name
        let (a, b, c, d, e) = bytes
        // fixme: check for ascii-alphanumeric
        self.data = String(data: Data([a, b, c, d, e]), encoding: .ascii)!
    }

    public var description: String {
        data
    }

    internal static func generate<E: EntityId>(for entity: E, on ledgerId: LedgerId) -> Self {
        // todo: fix these
        // swiftlint:disable identifier_name
        // 3 digits in base 26
        let p3 = 26 * 26 * 26
        // 5 digits in base 26
        let p5 = 26 * 26 * 26 * 26 * 26

        // min prime greater than a million. Used for the final permutation.
        let m = 1_000_003

        // Sum s of digit values weights them by powers of W. Should be coprime to P5.
        let w = 31
        // W to the 6th power
        let w6 = w * w * w * w * w * w

        // don't need the six 0 bytes.
        let h = ledgerId.bytes

        let d = entity.description.map { char -> Int in
            if char == "." {
                return 10
            } else {
                return char.wholeNumberValue!
            }
        }

        // Weighted sum of all positions (mod P3)
        var s = 0
        // Sum of even positions (mod 11)
        var s0 = 0
        // Sum of odd positions (mod 11)
        var s1 = 0

        for (index, digit) in d.enumerated() {
            s = (w * s + digit) % p3
            if index.isOdd {
                s1 += digit
            } else {
                s0 += digit
            }
        }

        s0 = s0 % 11
        s1 = s1 % 11

        // instead of six 0 bytes, we compute this in two steps
        var sh = h.reduce(0) { (result, value) in (w * result + Int(value)) % p5 }
        // `(w * result + Int(0)) % p5` applied 6 times...
        // `(w * result + Int(0)) % p5 = (w * result) % p5` because 0 is the additive identity
        // then expanding out the full expression:
        // `((w * ((w * ((w * ((w * ((w * ((w * result) % p5)) % p5)) % p5)) % p5)) % p5)) % p5)`
        // ... and using the fact that `((x % y) * z) % y = (x * z) % y`
        // we get:
        sh = (sh * w6) % p5

        // original expression:
        // var c = ((((((entityIdString.count % 5) * 11 + s0) * 11 + s1) * p3 + s + sh) % p5) * m) % p5
        // but `((x % y) * z) % y = ((x * z) % y) % y = (x * z) % y`
        // checksum as a single number
        var c = (((((d.count % 5) * 11 + s0) * 11 + s1) * p3 + s + sh) * m) % p5

        var output: [UInt8] = [0, 0, 0, 0, 0]

        for i in (0..<5).reversed() {
            output[i] = UInt8(0x61 + c % 26)
            c /= 26
        }

        // thanks swift, for not having fixed length arrays
        return Checksum(bytes: (output[0], output[1], output[2], output[3], output[4]))

        // swiftlint:endable identifier_name
    }
}
