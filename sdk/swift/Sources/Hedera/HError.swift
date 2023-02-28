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

private enum HederaErrorDetails {
    case grpcStatus(Int32)
    case statusTransactionId(status: Status, transactionId: TransactionId)
    case statusNoTransactionId(status: Status)
    case maxQueryPaymentExceeded(maxQueryPayment: Hbar, queryCost: Hbar)
    case badEntityId(HError.BadEntityId)

    static func last() -> (error: Self?, message: String?) {
        let message = lastErrorMessage()

        let error = hedera_last_error_details()

        switch error.tag {
        case HEDERA_ERROR_DETAILS_NONE:
            return (nil, message)
        case HEDERA_ERROR_DETAILS_ERROR_GRPC_STATUS:
            return (.grpcStatus(error.error_grpc_status), message!)
        case HEDERA_ERROR_DETAILS_ERROR_STATUS_TRANSACTION_ID:
            let error = error.ERROR_STATUS_TRANSACTION_ID
            return (
                .statusTransactionId(
                    status: Status(rawValue: error.status),
                    transactionId: .init(unsafeFromCHedera: error.transaction_id)
                ),
                message
            )
        case HEDERA_ERROR_DETAILS_ERROR_STATUS_NO_TRANSACTION_ID:
            let error = error.ERROR_STATUS_NO_TRANSACTION_ID
            return (.statusNoTransactionId(status: Status(rawValue: error.status)), message)

        case HEDERA_ERROR_DETAILS_ERROR_MAX_QUERY_PAYMENT_EXCEEDED:
            let error = error.ERROR_MAX_QUERY_PAYMENT_EXCEEDED
            return (
                .maxQueryPaymentExceeded(
                    maxQueryPayment: .fromTinybars(error.max_query_payment), queryCost: .fromTinybars(error.query_cost)),
                message!
            )
        case HEDERA_ERROR_DETAILS_ERROR_BAD_ENTITY_ID:
            return (.badEntityId(.fromCHedera(error.ERROR_BAD_ENTITY_ID)), message)
        default:
            fatalError("Unknown error details variant: \(error.tag), message: \(String(describing: message))")
        }
    }
}

/// Represents any possible error from a fallible function in the Hedera SDK.
public struct HError: Error, CustomStringConvertible {
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

        fileprivate static func fromCHedera(_ error: HederaErrorBadEntityId_Body) -> Self {
            Self(
                shard: error.shard,
                realm: error.realm,
                num: error.num,
                presentChecksum: Checksum(bytes: error.present_checksum),
                expectedChecksum: Checksum(bytes: error.expected_checksum)
            )
        }

        public var description: String {
            "entity ID \(shard).\(realm).\(num)-\(presentChecksum) was incorrect"
        }
    }

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
        case maxQueryPaymentExceeded
        case nodeAccountUnknown
        case responseStatusUnrecognized
        case signature
        case receiptStatus(status: Status, transactionId: TransactionId?)
        case requestParse
        case mnemonicParse
        case mnemonicEntropy
        case signatureVerify
        /// An entity ID had an invalid checksum
        case badEntityId(BadEntityId)
        case cannotToStringWithChecksum
        case cannotPerformTaskWithoutLedgerId
        case wrongKeyType
        case freezeUnsetNodeAccountIds
    }

    public let description: String
    public let kind: ErrorKind

    internal init(kind: ErrorKind, description: String) {
        self.kind = kind
        self.description = description
    }

    // swiftlint:disable cyclomatic_complexity function_body_length
    internal init?(_ error: HederaError) {
        // this consumes the error, so we have to get the message first if it exists
        let (errorInfo, message) = HederaErrorDetails.last()

        switch error {
        case HEDERA_ERROR_TIMED_OUT:
            kind = .timedOut

        case HEDERA_ERROR_GRPC_STATUS:
            guard case .grpcStatus(let status) = errorInfo else {
                fatalError("Unexpected \(String(describing: errorInfo))")
            }

            kind = .grpcStatus(status: status)

        case HEDERA_ERROR_FROM_PROTOBUF:
            kind = .fromProtobuf

        case HEDERA_ERROR_TRANSACTION_PRE_CHECK_STATUS:
            guard case .statusTransactionId(let status, let transactionId) = errorInfo else {
                fatalError("Unexpected \(String(describing: errorInfo))")
            }

            kind = .transactionPreCheckStatus(status: status, transactionId: transactionId)

        case HEDERA_ERROR_TRANSACTION_NO_ID_PRE_CHECK_STATUS:
            guard case .statusNoTransactionId(let status) = errorInfo else {
                fatalError("Unexpected \(String(describing: errorInfo))")
            }

            kind = .transactionNoIdPreCheckStatus(status: status)

        case HEDERA_ERROR_QUERY_PRE_CHECK_STATUS:
            guard case .statusTransactionId(let status, let transactionId) = errorInfo else {
                fatalError("Unexpected \(String(describing: errorInfo))")
            }

            kind = .queryPreCheckStatus(status: status, transactionId: transactionId)

        case HEDERA_ERROR_QUERY_PAYMENT_PRE_CHECK_STATUS:
            guard case .statusTransactionId(let status, let transactionId) = errorInfo else {
                fatalError("Unexpected \(String(describing: errorInfo))")
            }

            kind = .queryPaymentPreCheckStatus(status: status, transactionId: transactionId)

        case HEDERA_ERROR_QUERY_NO_PAYMENT_PRE_CHECK_STATUS:
            guard case .statusNoTransactionId(let status) = errorInfo else {
                fatalError("Unexpected \(String(describing: errorInfo))")
            }

            kind = .queryNoPaymentPreCheckStatus(status: status)

        case HEDERA_ERROR_BASIC_PARSE:
            kind = .basicParse

        case HEDERA_ERROR_KEY_PARSE:
            kind = .keyParse

        case HEDERA_ERROR_KEY_DERIVE:
            kind = .keyDerive

        case HEDERA_ERROR_NO_PAYER_ACCOUNT_OR_TRANSACTION_ID:
            kind = .noPayerAccountOrTransactionId

        case HEDERA_ERROR_MAX_QUERY_PAYMENT_EXCEEDED:
            kind = .maxQueryPaymentExceeded

        case HEDERA_ERROR_NODE_ACCOUNT_UNKNOWN:
            kind = .nodeAccountUnknown

        case HEDERA_ERROR_RESPONSE_STATUS_UNRECOGNIZED:
            kind = .responseStatusUnrecognized

        case HEDERA_ERROR_SIGNATURE:
            kind = .signature

        case HEDERA_ERROR_RECEIPT_STATUS:
            let status: Status
            let transactionId: TransactionId?
            switch errorInfo {
            case .statusTransactionId(let statusInner, let transactionIdInner):
                status = statusInner
                transactionId = transactionIdInner
            case .statusNoTransactionId(let statusInner):
                status = statusInner
                transactionId = nil
            default:
                fatalError("Unexpected \(String(describing: errorInfo))")

            }
            kind = .receiptStatus(status: status, transactionId: transactionId)

        case HEDERA_ERROR_REQUEST_PARSE:
            kind = .requestParse

        case HEDERA_ERROR_MNEMONIC_PARSE:
            kind = .mnemonicParse

        case HEDERA_ERROR_MNEMONIC_ENTROPY:
            kind = .mnemonicEntropy

        case HEDERA_ERROR_SIGNATURE_VERIFY:
            kind = .signatureVerify

        case HEDERA_ERROR_BAD_ENTITY_ID:
            guard case .badEntityId(let field) = errorInfo else {
                fatalError("Unexpected \(String(describing: errorInfo))")
            }

            kind = .badEntityId(field)

        case HEDERA_ERROR_CANNOT_TO_STRING_WITH_CHECKSUM:
            kind = .cannotToStringWithChecksum

        case HEDERA_ERROR_CANNOT_PERFORM_TASK_WITHOUT_LEDGER_ID:
            kind = .cannotPerformTaskWithoutLedgerId

        case HEDERA_ERROR_WRONG_KEY_TYPE:
            kind = .wrongKeyType

        case HEDERA_ERROR_FREEZE_UNSET_NODE_ACCOUNT_IDS:
            kind = .freezeUnsetNodeAccountIds

        case HEDERA_ERROR_OK:
            return nil

        default:
            let message = String(describing: message)
            fatalError("unknown error code `\(error)`, message: `\(message)`")

            return nil
        }

        description = message!
    }

    internal static func throwing(error: HederaError) throws {
        if let err = Self(error) {
            throw err
        }
    }

    internal static func badEntityId(
        shard: UInt64, realm: UInt64, num: UInt64, presentChecksum: Checksum, expectedChecksum: Checksum
    ) -> Self {
        let err = BadEntityId(
            shard: shard, realm: realm, num: num, presentChecksum: presentChecksum, expectedChecksum: expectedChecksum)
        return Self(kind: .badEntityId(err), description: err.description)
    }

    // swiftlint:enable cyclomatic_complexity function_body_length
}

extension HError: LocalizedError {
    public var errorDescription: String? {
        description
    }
}
