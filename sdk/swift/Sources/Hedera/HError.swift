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

private func lastErrorMessage() -> String? {
    guard let descriptionBytes = hedera_error_message() else {
        return nil
    }

    return String(hString: descriptionBytes)
}

public enum MnemonicParseError: Equatable, CustomStringConvertible {
    /// The mnemonic has an unexpected length.
    case badLength(Int)
    /// The mnemonic contains words that are not in the wordlist.
    case unknownWords([Int])
    /// The checksum for the mnemonic isn't as expected
    case checksumMismatch(expected: UInt8, actual: UInt8)

    public var description: String {
        switch self {
        case .badLength(let length):
            return "bad length: expected `12` or `24` words, found `\(length)`"
        case .unknownWords(let words):
            return "unknown words at indecies: `\(words)`"
        case .checksumMismatch(let expected, let actual):
            return "checksum mismatch: expected `0x\([expected].toHexString())`, found `0x\([actual].toHexString())`"
        }
    }
}

public enum MnemonicEntropyError: Equatable, CustomStringConvertible {
    case badLength(expected: Int, actual: Int)
    case checksumMismatch(expected: UInt8, actual: UInt8)
    case legacyWithPassphrase

    public var description: String {
        switch self {
        case .badLength(let expected, let actual):
            return "bad length: expected `\(expected)` words, found \(actual) words"
        case .checksumMismatch(let expected, let actual):
            return "checksum mismatch: expected `0x\([expected].toHexString())`, found `0x\([actual].toHexString())`"
        case .legacyWithPassphrase:
            return "used a passphrase with a legacy mnemonic"
        }
    }
}

/// Represents any possible error from a fallible function in the Hedera SDK.
public struct HError: Error, Equatable, CustomStringConvertible {
    // https://developer.apple.com/documentation/swift/error#2845903
    public enum ErrorKind: Equatable {
        indirect case timedOut(source: HError?)
        case grpcStatus(status: Int32)
        case fromProtobuf
        // TODO: add TransactionId
        case transactionPreCheckStatus(status: Status)
        case transactionNoIdPreCheckStatus(status: Status)
        case queryPreCheckStatus(status: Status)
        case queryPaymentPreCheckStatus(status: Status)
        case queryNoPaymentPreCheckStatus(status: Status)
        case basicParse
        case keyParse
        case keyDerive
        case noPayerAccountOrTransactionId
        case maxQueryPaymentExceeded
        case nodeAccountUnknown
        case responseStatusUnrecognized
        case signature
        case receiptStatus(status: Status)
        case mnemonicParse(reason: MnemonicParseError, mnemonic: Mnemonic)
        case mnemonicEntropy(MnemonicEntropyError)
        case signatureVerify
        case badEntityId
        case cannotToStringWithChecksum
        case cannotPerformTaskWithoutLedgerId
        case wrongKeyType
        case freezeUnsetNodeAccountIds
    }

    public let description: String
    public let kind: ErrorKind

    internal static let timedOut = Self.timedOut(source: nil)
    internal static func timedOut(source: Self?) -> Self {
        Self(kind: .timedOut(source: source), description: "Operation timed out")
    }

    internal init(kind: ErrorKind, description: String) {
        self.kind = kind
        self.description = description
    }

    internal static func mnemonicParse(_ reason: MnemonicParseError, _ mnemonic: Mnemonic) -> Self {
        Self(
            kind: .mnemonicParse(reason: reason, mnemonic: mnemonic),
            description: "failed to parse a mnemonic: \(reason)"
        )
    }

    internal static func mnemonicEntropy(_ reason: MnemonicEntropyError) -> Self {
        Self(
            kind: .mnemonicEntropy(reason),
            description: "failed to convert a mnemonic to entropy: \(reason)"
        )
    }

    // swiftlint:disable cyclomatic_complexity function_body_length
    internal init?(_ error: HederaError) {
        switch error {
        case HEDERA_ERROR_KEY_PARSE:
            kind = .keyParse

        case HEDERA_ERROR_KEY_DERIVE:
            kind = .keyDerive

        case HEDERA_ERROR_SIGNATURE_VERIFY:
            kind = .signatureVerify

        case HEDERA_ERROR_WRONG_KEY_TYPE:
            kind = .wrongKeyType

        case HEDERA_ERROR_OK:
            return nil

        default:
            let message = String(describing: lastErrorMessage())
            fatalError("unknown error code `\(error)`, message: `\(message)`")

            return nil
        }

        description = lastErrorMessage()!
    }

    internal static func fromProtobuf(_ description: String) -> Self {
        Self(kind: .fromProtobuf, description: description)
    }

    internal static func basicParse(_ description: String) -> Self {
        Self(kind: .basicParse, description: description)
    }

    internal static func throwing(error: HederaError) throws {
        if let err = Self(error) {
            throw err
        }
    }

    // swiftlint:enable cyclomatic_complexity function_body_length
}

extension HError: LocalizedError {
    public var errorDescription: String? {
        description
    }
}
