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

internal struct ChunkInfo {
    internal let current: Int
    internal let total: Int
    internal let initialTransactionId: TransactionId
    internal let currentTransactionId: TransactionId
    internal let nodeAccountId: AccountId

    internal static func single(transactionId: TransactionId, nodeAccountId: AccountId) -> Self {
        .initial(total: 1, transactionId: transactionId, nodeAccountId: nodeAccountId)
    }

    internal static func initial(total: Int, transactionId: TransactionId, nodeAccountId: AccountId) -> Self {
        self.init(
            current: 0,
            total: total,
            initialTransactionId: transactionId,
            currentTransactionId: transactionId,
            nodeAccountId: nodeAccountId
        )
    }

    internal func assertSingleTransaction() -> (transactionId: TransactionId, nodeAccountId: AccountId) {
        precondition(self.current == 0 && self.total == 1)

        return (transactionId: self.currentTransactionId, nodeAccountId: self.nodeAccountId)
    }
}
