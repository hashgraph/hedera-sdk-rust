import CHedera
import Foundation

/// Represents any possible error from a fallible function in the Hedera SDK.
public struct HError: Error, CustomStringConvertible {
    // https://developer.apple.com/documentation/swift/error#2845903
    public enum ErrorKind {
        case timedOut
        case grpcStatus(code: Int32)
        case fromProtobuf
        // TODO: add transactionId
        case preCheckStatus(code: Int32)
        case basicParse
        case keyParse
        case noPayerAccountOrTransactionId
        case maxAttemptsExceeded
        case maxQueryPaymentExceeded
        case nodeAccountUnknown
        case responseStatusUnrecognized
        case signature
        case requestParse
    }

    public let description: String
    public let kind: ErrorKind

    internal init?(_ error: HederaError) {
        switch error {
        case HEDERA_ERROR_TIMED_OUT:
            kind = .timedOut

        case HEDERA_ERROR_GRPC_STATUS:
            kind = .grpcStatus(code: hedera_error_grpc_status())

        case HEDERA_ERROR_FROM_PROTOBUF:
            kind = .fromProtobuf

        case HEDERA_ERROR_PRE_CHECK_STATUS:
            kind = .preCheckStatus(code: hedera_error_pre_check_status())

        case HEDERA_ERROR_BASIC_PARSE:
            kind = .basicParse

        case HEDERA_ERROR_KEY_PARSE:
            kind = .keyParse

        case HEDERA_ERROR_NO_PAYER_ACCOUNT_OR_TRANSACTION_ID:
            kind = .noPayerAccountOrTransactionId

        case HEDERA_ERROR_MAX_ATTEMPTS_EXCEEDED:
            kind = .maxAttemptsExceeded

        case HEDERA_ERROR_MAX_QUERY_PAYMENT_EXCEEDED:
            kind = .maxQueryPaymentExceeded

        case HEDERA_ERROR_NODE_ACCOUNT_UNKNOWN:
            kind = .nodeAccountUnknown

        case HEDERA_ERROR_RESPONSE_STATUS_UNRECOGNIZED:
            kind = .responseStatusUnrecognized

        case HEDERA_ERROR_SIGNATURE:
            kind = .signature

        case HEDERA_ERROR_REQUEST_PARSE:
            kind = .requestParse

        default:
            // HEDERA_ERROR_OK
            return nil
        }

        let descriptionBytes = hedera_error_message()
        description = String(validatingUTF8: descriptionBytes!)!
    }
}

extension HError: LocalizedError {
    public var errorDescription: String? {
        description
    }
}
