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

/// Response from ``Transaction.execute``.
///
/// When the client sends a node a transaction of any kind, the node replies with this, which
/// simply says that the transaction passed the pre-check (so the node will submit it to
/// the network).
///
/// To learn the consensus result, the client should later obtain a
/// receipt (free), or can buy a more detailed record (not free).
///
public struct TransactionResponse: Decodable {
    /// The account ID of the node that the transaction was submitted to.
    public let nodeAccountId: AccountId

    /// The client-generated transaction ID of the transaction that was submitted.
    ///
    /// This can be used to lookup the transaction in an explorer.
    ///
    public let transactionId: String
    // TODO: Use `TransactionId` type

    /// The client-generated SHA-384 hash of the transaction that was submitted.
    ///
    /// This can be used to lookup the transaction in an explorer.
    ///
    public let transactionHash: String
    // TODO: Use `TransactionHash` type

    /// Get the receipt of this transaction.
    /// Will wait for consensus.
    /// Will return a `receiptStatus` error for a failing receipt.
    public func getReceipt(_ client: Client) async throws -> TransactionReceipt {
        try await TransactionReceiptQuery()
            .transactionId(transactionId)
            .nodeAccountIds([nodeAccountId])
            .validateStatus(true)
            .execute(client)
    }
}
