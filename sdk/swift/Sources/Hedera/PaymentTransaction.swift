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

internal final class PaymentTransaction: Codable {
    internal var nodeAccountIds: [AccountId]?
    internal var amount: UInt64?
    internal var maxAmount: UInt64?
    internal var maxTransactionFee: UInt64?
    internal var transactionMemo: String?
    internal var payerAccountId: AccountId?
    internal var transactionId: String?
    internal var transactionValidDuration: TimeInterval?
    // TODO: private var paymentSigners: [OpaquePointer] = [];

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(nodeAccountIds, forKey: .nodeAccountIds)
        try container.encodeIfPresent(amount, forKey: .amount)
        try container.encodeIfPresent(maxAmount, forKey: .maxAmount)
        try container.encodeIfPresent(maxTransactionFee, forKey: .maxTransactionFee)
        try container.encodeIfPresent(transactionMemo, forKey: .transactionMemo)
        try container.encodeIfPresent(payerAccountId, forKey: .payerAccountId)
        try container.encodeIfPresent(transactionId, forKey: .transactionId)
        try container.encodeIfPresent(transactionValidDuration, forKey: .transactionValidDuration)
    }
}
