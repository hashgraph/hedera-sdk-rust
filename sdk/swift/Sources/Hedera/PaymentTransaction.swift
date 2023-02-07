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
    internal var amount: Hbar? {
        willSet {
            ensureNotFrozen(fieldName: "amount")
        }
    }
    internal var maxAmount: Hbar? {
        willSet {
            ensureNotFrozen(fieldName: "maxAmount")
        }
    }

    private enum CodingKeys: CodingKey {
        case amount
        case maxAmount

    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(amount, forKey: .amount)
        try container.encodeIfPresent(maxAmount, forKey: .maxAmount)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try super.validateChecksums(on: ledgerId)
    }

    internal override func execute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_CryptoServiceAsyncClient(channel: channel).cryptoTransfer(request)
    }

    internal override func toTransactionDataProtobuf(_ nodeAccountId: AccountId, _ transactionId: TransactionId)
        -> Proto_TransactionBody.OneOf_Data
    {
        let amount = amount ?? Hbar.zero
        return .cryptoTransfer(
            .with { proto in
                proto.transfers = .with { transfers in
                    transfers.accountAmounts = [
                        .with { transfer in
                            transfer.accountID = transactionId.accountId.toProtobuf()
                            transfer.amount = amount.toTinybars()
                        },
                        .with { transfer in
                            transfer.accountID = nodeAccountId.toProtobuf()
                            transfer.amount = -amount.toTinybars()
                        },
                    ]
                }
            }
        )
    }
}
