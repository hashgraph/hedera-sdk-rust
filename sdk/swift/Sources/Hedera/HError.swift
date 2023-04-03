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

/// Represents any possible error from a fallible function in the Hedera SDK.
public struct HError: Error, CustomStringConvertible {
    // https://developer.apple.com/documentation/swift/error#2845903
    public enum ErrorKind: Equatable {
        case timedOut
        case grpcStatus(status: Int32)
        case fromProtobuf
        case transactionPreCheckStatus(status: Status, transactionId: TransactionId)
        case transactionNoIdPreCheckStatus(status: Status)
        case queryPreCheckStatus(status: Status, transactionId: TransactionId)
        case queryPaymentPreCheckStatus(status: Status, transactionId: TransactionId)
        case queryNoPaymentPreCheckStatus(status: Status)
        case basicParse
        case keyParse
        case keyDerive
        case noPayerAccountOrTransactionId
        case maxQueryPaymentExceeded(queryCost: Hbar, maxQueryPayment: Hbar)
        case nodeAccountUnknown
        case responseStatusUnrecognized
        case receiptStatus(status: Status, transactionId: TransactionId?)
        case mnemonicParse(reason: MnemonicParse, mnemonic: Mnemonic)
        case mnemonicEntropy(MnemonicEntropy)
        case signatureVerify
        /// An entity ID had an invalid checksum
        case badEntityId(BadEntityId)
        case cannotCreateChecksum
        case freezeUnsetNodeAccountIds
    }

    public let description: String
    public let kind: ErrorKind

    internal init(kind: ErrorKind, description: String) {
        self.kind = kind
        self.description = description
    }

    internal init?(_ error: HederaError) {
        // this consumes the error, so we have to get the message first if it exists
        let message = lastErrorMessage()

        switch error {
        case HEDERA_ERROR_KEY_PARSE:
            kind = .keyParse

        case HEDERA_ERROR_KEY_DERIVE:
            kind = .keyDerive

        case HEDERA_ERROR_SIGNATURE_VERIFY:
            kind = .signatureVerify

        case HEDERA_ERROR_OK:
            return nil

        default:
            let message = String(describing: message)
            fatalError("unknown error code `\(error)`, message: `\(message)`")

            return nil
        }

        description = message!
    }

    internal static func fromProtobuf(_ description: String) -> Self {
        Self(kind: .fromProtobuf, description: description)
    }

    internal static func throwing(error: HederaError) throws {
        if let err = Self(error) {
            throw err
        }
    }

    internal static func mnemonicParse(_ error: MnemonicParse, _ mnemonic: Mnemonic) -> Self {
        Self(
            kind: .mnemonicParse(reason: error, mnemonic: mnemonic),
            description: String(describing: error)
        )
    }

    internal static func mnemonicEntropy(_ error: MnemonicEntropy) -> Self {
        Self(
            kind: .mnemonicEntropy(error),
            description: String(describing: error)
        )
    }

    internal static func badEntityId(
        shard: UInt64, realm: UInt64, num: UInt64, presentChecksum: Checksum, expectedChecksum: Checksum
    ) -> Self {
        let err = BadEntityId(
            shard: shard, realm: realm, num: num, presentChecksum: presentChecksum, expectedChecksum: expectedChecksum)
        return Self(kind: .badEntityId(err), description: err.description)
    }

    internal static func basicParse(_ description: String) -> Self {
        Self(kind: .basicParse, description: description)
    }

    internal static func keyParse(_ description: String) -> Self {
        Self(kind: .keyParse, description: "failed to parse a key: \(description)")
    }

    internal static let cannotCreateChecksum: Self = Self(
        kind: .cannotCreateChecksum,
        description: "an entity ID with an alias or evmAddress cannot have a checksum"
    )

    internal static let noPayerAccountOrTransactionId: Self = Self(
        kind: .noPayerAccountOrTransactionId,
        description:
            "client must be configured with a payer account or requests must be given an explicit transaction id"
    )

    internal static func maxQueryPaymentExceeded(queryCost: Hbar, maxQueryPayment: Hbar) -> Self {
        let kind = ErrorKind.maxQueryPaymentExceeded(queryCost: queryCost, maxQueryPayment: maxQueryPayment)

        let description =
            "cost of (queryCost) without explicit payment is greater than the maximum allowed payment of \(maxQueryPayment)"

        return Self(kind: kind, description: description)
    }

    internal static func timedOut(_ description: String) -> Self {
        Self(
            kind: .timedOut,
            description:
                "Failed to complete request within the maximum time allowed; most recent attempt failed with: \(description)"
        )
    }

    // swiftlint:enable function_body_length
}

extension HError: LocalizedError {
    public var errorDescription: String? {
        description
    }
}

extension HError {
    /// An entity ID had an invalid checksum
    public struct BadEntityId: Equatable, CustomStringConvertible {
        /// The entity ID's shard.
        public let shard: UInt64
        /// The entity ID's realm.
        public let realm: UInt64
        /// The entity ID's num.
        public let num: UInt64
        /// The (invalid) checksum that was present on the entity ID.
        public let presentChecksum: Checksum
        /// The checksum that should've been present on the entity ID.
        public let expectedChecksum: Checksum

        public var description: String {
            "entity ID \(shard).\(realm).\(num)-\(presentChecksum) was incorrect"
        }
    }
}

extension HError {
    public enum MnemonicParse: Equatable, CustomStringConvertible {
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
                return
                    "checksum mismatch: expected `0x\(String(expected, radix: 16))`, found `0x\(String(actual, radix: 16))`"
            }
        }
    }

    public enum MnemonicEntropy: Equatable, CustomStringConvertible {
        case badLength(expected: Int, actual: Int)
        case checksumMismatch(expected: UInt8, actual: UInt8)
        case legacyWithPassphrase

        public var description: String {
            switch self {
            case .badLength(let expected, let actual):
                return "bad length: expected `\(expected)` words, found \(actual) words"
            case .checksumMismatch(let expected, let actual):
                return
                    "checksum mismatch: expected `0x\(String(expected, radix: 16))`, found `0x\(String(actual, radix: 16))`"
            case .legacyWithPassphrase:
                return "used a passphrase with a legacy mnemonic"
            }
        }
    }
}

extension HError.ErrorKind: Sendable {}
extension HError.MnemonicEntropy: Sendable {}
extension HError.MnemonicParse: Sendable {}
extension HError.BadEntityId: Sendable {}
