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

/// A query that can be executed on the Hedera network.
public class Query<Response: Decodable>: Request {
    public typealias Response = Response

    internal init() {}

    private var payment: PaymentTransaction = PaymentTransaction()

    /// Set the account IDs of the nodes that this query may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client; or, the node account IDs
    /// configured on the query payment transaction (if explicitly provided).
    ///
    @discardableResult
    public func nodeAccountIds(_ nodeAccountIds: [AccountId]) -> Self {
        self.payment.nodeAccountIds = nodeAccountIds

        return self
    }

    /// Set an explicit payment amount for this query.
    ///
    /// The client will submit exactly this amount for the payment of this query. Hedera
    /// will not return any remainder (over the actual cost for this query).
    ///
    public func paymentAmount(_ amount: Hbar) -> Self {
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
    public func maxPaymentAmount(_ maxAmount: Hbar?) -> Self {
        self.payment.maxAmount = maxAmount

        return self
    }

    /// Sets the duration that the payment transaction is valid for, once finalized and signed.
    ///
    /// Defaults to 120 seconds (or two minutes).
    ///
    public func paymentTransactionValidDuration(_ validDuration: Duration) -> Self {
        self.payment.transactionValidDuration = validDuration

        return self
    }

    /// Set the maximum transaction fee the payer account is willing to pay for the query
    /// payment transaction.
    ///
    /// Defaults to 1 hbar.
    ///
    public func maxPaymentTransactionFee(_ maxPaymentTransactionFee: Hbar) -> Self {
        self.payment.maxTransactionFee = maxPaymentTransactionFee

        return self
    }

    /// Set a note or description that should be recorded in the transaction record (maximum length
    /// of 100 characters) for the payment transaction.
    public func paymentTransactionMemo(_ memo: String) -> Self {
        self.payment.transactionMemo = memo

        return self
    }

    /// Sets the account that will be paying for this query.
    public func payerAccountId(_ payerAccountId: AccountId) -> Self {
        self.payment.payerAccountId = payerAccountId

        return self
    }

    /// Set an explicit transaction ID to use to identify the payment transaction
    /// on this query.
    ///
    /// Overrides payer account defined on this query or on the client.
    public func paymentTransactionId(_ transactionId: TransactionId) -> Self {
        self.payment.transactionId = transactionId

        return self
    }

    public func getCost(_ client: Client) async throws -> Hbar {

        try await QueryCost(query: self).execute(client)
    }

    // TODO: paymentSigner

    private enum CodingKeys: String, CodingKey {
        case type = "$type"
        case payment
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        let typeName = String(describing: type(of: self))
        let requestName = typeName.prefix(1).lowercased() + typeName.dropFirst().dropLast(5)

        try container.encode(requestName, forKey: .type)
        try container.encode(payment, forKey: .payment)
    }

    public func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> Response {
        try await executeInternal(client, timeout)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try payment.validateChecksums(on: ledgerId)
    }
}
