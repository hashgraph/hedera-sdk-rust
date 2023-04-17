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
import NumberKit

///  `BIP-39` 24-word mnemonic phrase compatible with the Android and iOS mobile wallets.
public struct Mnemonic: Equatable {
    // *sigh*.
    // We need this to ensure the generate functions will actually give the right results.
    internal static func fromEntropyForTesting(entropy: Data) -> Self {
        Self(kind: .v2v3(.fromEntropy(entropy)))
    }

    private init(kind: Mnemonic.Kind) {
        self.kind = kind
    }

    private enum Kind: Equatable {
        // swiftlint:disable:next identifier_name
        case v1(MnemonicV1Data)
        case v2v3(MnemonicV2V3Data)
    }

    private let kind: Kind

    /// Returns `true` if `self` is a legacy mnemonic.
    public var isLegacy: Bool {
        if case .v1 = kind {
            return true
        }

        return false
    }

    public var words: [String] {
        switch kind {
        case .v1(let data):
            return data.words
        case .v2v3(let data):
            return data.words
        }
    }

    public static func fromString(_ description: String) throws -> Self {
        try Self(parsing: description)
    }

    fileprivate init(parsing description: String) throws {
        self = try .fromWords(words: description.split(separator: " ").map(String.init))
    }

    public static func fromWords(words: [String]) throws -> Self {
        if words.count == 22 {
            return Self(kind: .v1(MnemonicV1Data(words: words)))
        }

        let mnemonic = Self(kind: .v2v3(MnemonicV2V3Data(words: words)))

        guard words.count == 12 || words.count == 24 else {
            throw HError.mnemonicParse(.badLength(words.count), mnemonic)
        }

        var wordIndecies: [UInt16] = []
        var unknownWords: [Int] = []

        for (offset, word) in words.enumerated() {
            switch bip39WordList.indexOf(word: word) {
            case .some(let index):
                wordIndecies.append(UInt16(index))
            case nil:
                unknownWords.append(offset)
            }
        }

        guard unknownWords.isEmpty else {
            throw HError.mnemonicParse(.unknownWords(unknownWords), mnemonic)
        }

        let (entropy, actualChecksum) = inceciesToEntropyAndChecksum(wordIndecies)

        var expectedChecksum = checksum(entropy)
        expectedChecksum = words.count == 12 ? (expectedChecksum & 0xf0) : expectedChecksum

        guard expectedChecksum == actualChecksum else {
            throw HError.mnemonicParse(.checksumMismatch(expected: expectedChecksum, actual: actualChecksum), mnemonic)
        }

        return mnemonic
    }

    public static func generate12() -> Self {
        Self(kind: .v2v3(.generate12()))
    }

    public static func generate24() -> Self {
        Self(kind: .v2v3(.generate24()))
    }

    public func toLegacyPrivateKey() throws -> PrivateKey {
        let entropy: Foundation.Data
        switch kind {
        case .v1(let mnemonic):
            entropy = try mnemonic.toEntropy()
        case .v2v3(let mnemonic):
            entropy = try mnemonic.toLegacyEntropy()
        }

        return try .fromBytes(entropy)
    }

    public func toPrivateKey(passphrase: String = "") throws -> PrivateKey {
        switch kind {
        case .v1 where !passphrase.isEmpty:
            throw HError.mnemonicEntropy(.legacyWithPassphrase)
        case .v1(let mnemonic):
            let entropy = try mnemonic.toEntropy()
            // failure here is `unreachable`
            // swiftlint:disable:next force_try
            return try! PrivateKey.fromBytes(entropy)

        // known unfixable bug: `PrivateKey.fromMnemonic` can be called with a legacy private key.
        case .v2v3:
            return PrivateKey.fromMnemonic(self, "")
        }
    }

    public func toString() -> String {
        String(describing: self)
    }

    internal func toSeed<S: StringProtocol>(passphrase: S) -> Data {
        var salt = "mnemonic"
        salt += passphrase

        return
            Pkcs5.pbkdf2(
                variant: .sha2(.sha512),
                password: String(describing: self).data(using: .utf8)!,
                salt: salt.data(using: .utf8)!,
                rounds: 2048,
                keySize: 64
            )

    }
}

extension Mnemonic: LosslessStringConvertible {
    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public var description: String {
        self.words.joined(separator: " ")
    }
}

extension Mnemonic: ExpressibleByStringLiteral {
    public typealias StringLiteralType = String

    public init(stringLiteral value: String) {
        // we do want to fatalError here.
        // swiftlint:disable:next force_try
        try! self.init(parsing: value)
    }
}

private struct MnemonicV1Data: Equatable {
    let words: [String]

    private static func convertRadix(_ nums: [Int32], from fromRadix: Int32, to toRadix: Int32, outputLength: Int)
        -> [Int32]
    {
        var buf = BigInt(0)
        let fromRadix = BigInt(fromRadix)

        for num in nums {
            buf *= fromRadix
            buf += BigInt(num)
        }

        var out: [Int32] = Array(repeating: 0, count: outputLength)

        let toRadix = BigInt(toRadix)

        for index in (0..<out.count).reversed() {
            let remainder: BigInt
            (buf, remainder) = buf.quotientAndRemainder(dividingBy: toRadix)
            out[index] = Int32(remainder.intValue!)
        }

        return out
    }

    private static func crc8<C>(_ data: C) -> UInt8 where C: Collection, C.Element == UInt8 {
        var crc: UInt8 = 0xff
        for value in data.dropLast(1) {
            crc ^= value
            for _ in 0..<8 {
                crc = (crc >> 1) ^ ((crc & 1) == 0 ? 0 : 0xb2)
            }
        }

        return crc ^ 0xff
    }

    func toEntropy() throws -> Data {
        let indecies: [Int32] = words.map { word in
            legacyWordList.indexOf(word: word).map(Int32.init) ?? -1
        }

        var data = Self.convertRadix(indecies, from: 4096, to: 256, outputLength: 33).map(
            UInt8.init(truncatingIfNeeded:)
        )

        precondition(data.count == 33)

        let crc = data.popLast()!

        for index in 0..<data.count {
            data[index] ^= crc
        }

        let crc2 = Self.crc8(data)

        guard crc == crc2 else {
            throw HError.mnemonicEntropy(.checksumMismatch(expected: crc2, actual: crc))
        }

        return Data(data)
    }
}

extension Mnemonic: Sendable {}

private struct MnemonicV2V3Data: Equatable {
    let words: [String]

    fileprivate static func fromEntropy(_ entropyIn: Data) -> Self {
        assert(entropyIn.count == 16 || entropyIn.count == 32, "Invalid entropy length")

        let checksum = checksum(entropyIn)

        let entropy: Data =
            entropyIn + [entropyIn.count == 16 ? (checksum & 0xf0) : checksum]

        var buffer: UInt32 = 0
        var offset: UInt8 = 0

        var words: [String] = []

        for byte in entropy {
            buffer = (buffer << 8) | UInt32(byte)
            offset += 8
            if offset >= 11 {
                let index = Int(buffer >> (offset - 11) & 0x7ff)
                words.append(String(bip39WordList[index]!))
                offset -= 11
            }
        }

        return Self(words: words)
    }

    fileprivate static func generate12() -> Self {
        fromEntropy(.randomData(withLength: 16))
    }

    fileprivate static func generate24() -> Self {
        fromEntropy(.randomData(withLength: 32))
    }

    fileprivate func toLegacyEntropy() throws -> Data {
        // error here where we'll have more context than `PrivateKey.fromBytes`.
        guard words.count == 24 else {
            throw HError.mnemonicEntropy(.badLength(expected: 24, actual: words.count))
        }

        // technically, this code all works for 12 words, but I'm going to pretend it doesn't.
        let (entropy, actualChecksum) = wordsToEntropyAndChecksum(words)

        var expectedChecksum = checksum(entropy)
        expectedChecksum = words.count == 12 ? (expectedChecksum & 0xf0) : expectedChecksum

        guard expectedChecksum == actualChecksum else {
            throw HError.mnemonicEntropy(.checksumMismatch(expected: expectedChecksum, actual: actualChecksum))
        }

        return entropy
    }
}

private func checksum(_ data: Data) -> UInt8 {
    Crypto.Sha2.sha256(data)[0]
}

private func wordsToEntropyAndChecksum(_ words: [String]) -> (entropy: Data, checksum: UInt8) {
    inceciesToEntropyAndChecksum(words.map { UInt16(bip39WordList.indexOf(word: $0)!) })
}

private func inceciesToEntropyAndChecksum(_ indecies: [UInt16]) -> (entropy: Data, checksum: UInt8) {
    precondition(indecies.count == 12 || indecies.count == 24)

    var output: Data = Data()
    var buf: UInt32 = 0
    var offset: UInt8 = 0

    for index in indecies {
        precondition(index <= 0x7ff)

        buf = (buf << 11) | UInt32(index)
        offset += 11
        while offset >= 8 {
            // we want to truncate.
            let byte = UInt8(truncatingIfNeeded: buf >> (offset - 8))
            output.append(byte)
            offset -= 8
        }
    }

    if offset != 0 {
        output.append(UInt8(truncatingIfNeeded: buf << offset))
    }

    var checksum = output.popLast()!
    checksum = indecies.count == 12 ? (checksum & 0xf0) : checksum

    return (output, checksum)
}
