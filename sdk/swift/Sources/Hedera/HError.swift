import CHedera
import Foundation

/// Represents any possible error from a fallible function in the Hedera SDK.
public struct HError: Error, CustomStringConvertible {
    // https://developer.apple.com/documentation/swift/error#2845903
    public enum ErrorKind {
        case timedOut
        case grpcStatus(status: Int32)
        case fromProtobuf
        // TODO: add TransactionId
        // TODO: use Status enum
        case transactionPreCheckStatus(status: Int32)
        case transactionNoIdPreCheckStatus(status: Int32)
        case queryPreCheckStatus(status: Int32)
        case queryPaymentPreCheckStatus(status: Int32)
        case queryNoPaymentPreCheckStatus(status: Int32)
        case basicParse
        case keyParse
        case noPayerAccountOrTransactionId
        case maxQueryPaymentExceeded
        case nodeAccountUnknown
        case responseStatusUnrecognized
        case signature
        // TODO: enum Status
        case receiptStatus(status: String)
        case requestParse
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
            kind = .transactionPreCheckStatus(status: hedera_error_pre_check_status())

        case HEDERA_ERROR_TRANSACTION_NO_ID_PRE_CHECK_STATUS:
            kind = .transactionNoIdPreCheckStatus(status: hedera_error_pre_check_status())

        case HEDERA_ERROR_QUERY_PRE_CHECK_STATUS:
            kind = .queryPreCheckStatus(status: hedera_error_pre_check_status())

        case HEDERA_ERROR_QUERY_PAYMENT_PRE_CHECK_STATUS:
            kind = .queryPaymentPreCheckStatus(status: hedera_error_pre_check_status())

        case HEDERA_ERROR_QUERY_NO_PAYMENT_PRE_CHECK_STATUS:
            kind = .queryNoPaymentPreCheckStatus(status: hedera_error_pre_check_status())

        case HEDERA_ERROR_BASIC_PARSE:
            kind = .basicParse

        case HEDERA_ERROR_KEY_PARSE:
            kind = .keyParse

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
            // TODO: get receipt status
            kind = .receiptStatus(status: "?")

        case HEDERA_ERROR_REQUEST_PARSE:
            kind = .requestParse

        default:
            // HEDERA_ERROR_OK
            return nil
        }

        let descriptionBytes = hedera_error_message()
        description = String(validatingUTF8: descriptionBytes!)!
    }
    // swiftlint:enable cyclomatic_complexity function_body_length
}

extension HError: LocalizedError {
    public var errorDescription: String? {
        description
    }
}
