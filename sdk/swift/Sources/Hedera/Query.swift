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
import GRPC
import HederaProtobufs

/// A query that can be executed on the Hedera network.
public class Query<Response>: ValidateChecksums {
    public typealias Response = Response

    internal init() {}

    private final var payment: PaymentTransaction = PaymentTransaction()

    internal final var nodeAccountIds: [AccountId]? {
        payment.nodeAccountIds
    }

    internal func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        fatalError("Method `Query.toQueryProtobufWith` must be overridden by `\(type(of: self))`")
    }

    internal func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        fatalError("Method `Query.queryExecute` must be overridden by `\(type(of: self))`")
    }

    internal func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        fatalError("Method `Query.makeQueryResponse` must be overridden by `\(type(of: self))`")
    }

    internal func shouldRetry(forResponse response: GrpcResponse) -> Bool {
        false
    }

    internal func shouldRetryPrecheck(forStatus status: Status) -> Bool {
        false
    }

    // reasonable default: the actual cost is expected.
    internal func mapCost(_ cost: Hbar) -> Hbar {
        cost
    }

    /// The transaction ID that this query is for, if this query is about a transaction (eg: TransactionReceiptQuery).
    internal var relatedTransactionId: TransactionId? { nil }

    internal var requiresPayment: Bool { true }

    /// Set the account IDs of the nodes that this query may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client; or, the node account IDs
    /// configured on the query payment transaction (if explicitly provided).
    ///
    @discardableResult
    public final func nodeAccountIds(_ nodeAccountIds: [AccountId]) -> Self {
        self.payment.nodeAccountIds = nodeAccountIds

        return self
    }

    /// Set an explicit payment amount for this query.
    ///
    /// The client will submit exactly this amount for the payment of this query. Hedera
    /// will not return any remainder (over the actual cost for this query).
    ///
    public final func paymentAmount(_ amount: Hbar) -> Self {
        self.payment.amount = amount

        return self
    }

    /// Set the maximum payment allowable for this query.
    ///
    /// When a query is executed without an explicit payment amount set,
    /// the client will first request the cost of the given query from the node it will be
    /// submitted to and attach a payment for that amount from the operator account on the client.
    ///
    /// If the returned value is greater than this value, a [`MaxQueryPaymentExceeded`] error
    /// will be returned.
    ///
    /// Defaults to the maximum payment amount configured on the client.
    ///
    /// Set to `None` to allow unlimited payment amounts.
    ///
    public final func maxPaymentAmount(_ maxAmount: Hbar?) -> Self {
        self.payment.maxAmount = maxAmount

        return self
    }

    /// Sets the duration that the payment transaction is valid for, once finalized and signed.
    ///
    /// Defaults to 120 seconds (or two minutes).
    ///
    public final func paymentTransactionValidDuration(_ validDuration: Duration) -> Self {
        self.payment.transactionValidDuration = validDuration

        return self
    }

    /// Set the maximum transaction fee the payer account is willing to pay for the query
    /// payment transaction.
    ///
    /// Defaults to 1 hbar.
    ///
    public final func maxPaymentTransactionFee(_ maxPaymentTransactionFee: Hbar) -> Self {
        self.payment.maxTransactionFee = maxPaymentTransactionFee

        return self
    }

    /// Set a note or description that should be recorded in the transaction record (maximum length
    /// of 100 characters) for the payment transaction.
    public final func paymentTransactionMemo(_ memo: String) -> Self {
        self.payment.transactionMemo = memo

        return self
    }

    /// Set an explicit transaction ID to use to identify the payment transaction
    /// on this query.
    ///
    /// Overrides payer account defined on this query or on the client.
    public final func paymentTransactionId(_ transactionId: TransactionId) -> Self {
        self.payment.transactionId = transactionId

        return self
    }

    public final func getCost(_ client: Client) async throws -> Hbar {
        try await QueryCost(query: self).execute(client)
    }

    // TODO: paymentSigner

    public final func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> Response {
        if self.requiresPayment {
            // hack: this is a TransactionRecordQuery, which means we need to run the receipt first.
            if let relatedTransactionId = self.relatedTransactionId {
                _ = try? await TransactionReceiptQuery().transactionId(relatedTransactionId).execute(client, timeout)
            }

            if self.payment.amount == nil {
                // should this inherit the timeout?
                // payment is required but none was specified, query the cost
                let cost = try await self.getCost(client)

                if let maxAmount = self.payment.maxAmount {
                    guard cost <= maxAmount else {
                        throw HError.maxQueryPaymentExceeded(queryCost: cost, maxQueryPayment: maxAmount)
                    }
                }

                self.payment.amount = cost
            }

            try payment.freezeWith(client)
        }

        return try await executeAny(client, self, timeout)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try payment.validateChecksums(on: ledgerId)
    }
}

extension Query: Execute {
    internal typealias GrpcRequest = Proto_Query

    internal typealias GrpcResponse = Proto_Response

    internal typealias Context = ()

    internal var explicitTransactionId: TransactionId? {
        payment.transactionId
    }

    internal var requiresTransactionId: Bool {
        self.requiresPayment
    }

    internal func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (Proto_Query, ()) {
        let request = toQueryProtobufWith(
            try .with { proto in
                proto.responseType = .answerOnly

                if requiresPayment {
                    proto.payment = try payment.makeRequest(transactionId, nodeAccountId).0
                }
            })

        return (request, ())
    }

    internal func execute(_ channel: GRPC.GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await queryExecute(channel, request)
    }

    internal func makeResponse(
        _ response: Proto_Response, _ context: (), _ nodeAccountId: AccountId, _ transactionId: TransactionId?
    ) throws -> Response {
        guard let response = response.response else {
            throw HError.fromProtobuf("unexpectly missing `response`")
        }

        return try makeQueryResponse(response)
    }

    internal func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError {
        if let transactionId = relatedTransactionId {
            return HError(
                kind: .queryPreCheckStatus(status: status, transactionId: transactionId),
                description: "query for transaction `\(transactionId)` failed pre-check with status `\(status)`"
            )
        }

        if let transactionId = transactionId {
            return HError(
                kind: .queryPaymentPreCheckStatus(status: status, transactionId: transactionId),
                description:
                    "query with payment transaction `\(transactionId)` failed pre-check with status `\(status)`"
            )
        }

        return HError(
            kind: .queryNoPaymentPreCheckStatus(status: status),
            description: "query with no payment transaction failed pre-check with status `\(status)`"
        )
    }

    internal static func responsePrecheckStatus(_ response: Proto_Response) throws -> Int32 {
        Int32(try response.header().nodeTransactionPrecheckCode.rawValue)
    }
}
