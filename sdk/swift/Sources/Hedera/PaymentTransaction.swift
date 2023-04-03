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

internal final class PaymentTransaction: Transaction {
    internal var amount: Hbar?
    internal var maxAmount: Hbar?

    // TODO: private var paymentSigners: [OpaquePointer] = [];

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        let (transactionId, nodeAccountId) = chunkInfo.assertSingleTransaction()

        let amount = self.amount ?? 0

        return .cryptoTransfer(
            .with { proto in
                proto.transfers = .with { proto in
                    proto.accountAmounts = [
                        .with { proto in
                            proto.accountID = nodeAccountId.toProtobuf()
                            proto.amount = amount.toTinybars()
                            proto.isApproval = false
                        },
                        .with { proto in
                            proto.accountID = transactionId.accountId.toProtobuf()
                            proto.amount = -(amount.toTinybars())
                            proto.isApproval = false
                        },
                    ]
                }
            }
        )
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_CryptoServiceAsyncClient(channel: channel).cryptoTransfer(request)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try super.validateChecksums(on: ledgerId)
    }
}
