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
        case requestParse
        case mnemonicParse
        case mnemonicEntropy
        case signatureVerify
        case badEntityId
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
        switch error {
        case HEDERA_ERROR_TIMED_OUT:
            kind = .timedOut

        case HEDERA_ERROR_GRPC_STATUS:
            kind = .grpcStatus(status: hedera_error_grpc_status())

        case HEDERA_ERROR_FROM_PROTOBUF:
            kind = .fromProtobuf

        case HEDERA_ERROR_TRANSACTION_PRE_CHECK_STATUS:
            kind = .transactionPreCheckStatus(status: Status(rawValue: hedera_error_pre_check_status()))

        case HEDERA_ERROR_TRANSACTION_NO_ID_PRE_CHECK_STATUS:
            kind = .transactionNoIdPreCheckStatus(status: Status(rawValue: hedera_error_pre_check_status()))

        case HEDERA_ERROR_QUERY_PRE_CHECK_STATUS:
            kind = .queryPreCheckStatus(status: Status(rawValue: hedera_error_pre_check_status()))

        case HEDERA_ERROR_QUERY_PAYMENT_PRE_CHECK_STATUS:
            kind = .queryPaymentPreCheckStatus(status: Status(rawValue: hedera_error_pre_check_status()))

        case HEDERA_ERROR_QUERY_NO_PAYMENT_PRE_CHECK_STATUS:
            kind = .queryNoPaymentPreCheckStatus(status: Status(rawValue: hedera_error_pre_check_status()))

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
            kind = .receiptStatus(status: Status(rawValue: hedera_error_receipt_status_status()))

        case HEDERA_ERROR_REQUEST_PARSE:
            kind = .requestParse

        case HEDERA_ERROR_MNEMONIC_PARSE:
            kind = .mnemonicParse

        case HEDERA_ERROR_MNEMONIC_ENTROPY:
            kind = .mnemonicEntropy

        case HEDERA_ERROR_SIGNATURE_VERIFY:
            kind = .signatureVerify

        case HEDERA_ERROR_BAD_ENTITY_ID:
            kind = .badEntityId

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
            let message = String(describing: lastErrorMessage())
            fatalError("unknown error code `\(error)`, message: `\(message)`")

            return nil
        }

        description = lastErrorMessage()!
    }

    internal static func fromProtobuf(_ description: String) -> Self {
        Self(kind: .fromProtobuf, description: description)
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
