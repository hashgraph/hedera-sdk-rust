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

import GRPC
import HederaProtobufs

/// Get the record of a transaction, given its transaction ID.
///
public final class TransactionRecordQuery: Query<TransactionRecord> {
    /// The ID of the transaction for which the record is being requested.
    public var transactionId: TransactionId?

    /// Set the ID of the transaction for which the record is being requested.
    @discardableResult
    public func transactionId(_ transactionId: TransactionId) -> Self {
        self.transactionId = transactionId

        return self
    }

    /// Whether records of processing duplicate transactions should be returned.
    public var includeDuplicates: Bool = false

    /// Sets whether records of processing duplicate transactions should be returned.
    @discardableResult
    public func includeDuplicates(_ includeDuplicates: Bool) -> Self {
        self.includeDuplicates = includeDuplicates

        return self
    }

    /// Whether the response should include the records of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    public var includeChildren: Bool = false

    /// Sets whether the response should include the records of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    @discardableResult
    public func includeChildren(_ includeChildren: Bool) -> Self {
        self.includeChildren = includeChildren

        return self
    }

    /// Whether the record status should be validated.
    public var validateStatus: Bool = false

    /// Sets whether the record status should be validated.
    @discardableResult
    public func validateStatus(_ validateStatus: Bool) -> Self {
        self.validateStatus = validateStatus

        return self
    }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.transactionGetRecord = .with { proto in
                proto.header = header
                proto.includeDuplicates = includeDuplicates
                proto.includeChildRecords = includeChildren
                transactionId?.toProtobufInto(&proto.transactionID)
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_CryptoServiceAsyncClient(channel: channel).getTxRecordByTxID(request)
    }

    internal override func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        guard case .transactionGetRecord(let proto) = response else {
            throw HError.fromProtobuf("unexpected \(response) received, expected `transactionGetRecord`")
        }

        let record = try Response.fromProtobuf(proto)

        let status = record.receipt.status

        if validateStatus && status != .success {
            throw HError(
                kind: .receiptStatus(status: status, transactionId: transactionId),
                description:
                    "receipt for transaction `\(String(describing: transactionId))` failed with status `\(status)"
            )
        }

        return record
    }

    internal override func shouldRetryPrecheck(forStatus status: Status) -> Bool {
        switch status {
        case .receiptNotFound, .recordNotFound: return true
        default: return false
        }
    }

    internal override var relatedTransactionId: TransactionId? { transactionId }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try transactionId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
