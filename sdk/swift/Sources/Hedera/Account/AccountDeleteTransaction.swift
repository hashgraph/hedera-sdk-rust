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

/// Mark an account as deleted, moving all its current hbars to another account.
///
/// It will remain in the ledger, marked as deleted, until it expires.
/// Transfers into it a deleted account will fail.
///
public final class AccountDeleteTransaction: Transaction {
    /// Create a new `AccountDeleteTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_CryptoDeleteTransactionBody) throws {
        accountId = data.hasDeleteAccountID ? try .fromProtobuf(data.deleteAccountID) : nil
        transferAccountId = data.hasTransferAccountID ? try .fromProtobuf(data.transferAccountID) : nil

        try super.init(protobuf: proto)
    }

    /// The account ID which will receive all remaining hbars.
    public var transferAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account ID which will receive all remaining hbars.
    @discardableResult
    public func transferAccountId(_ transferAccountId: AccountId) -> Self {
        self.transferAccountId = transferAccountId

        return self
    }

    /// The account ID which should be deleted.
    public var accountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account ID which should be deleted.
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try transferAccountId?.validateChecksums(on: ledgerId)
        try accountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_CryptoServiceAsyncClient(channel: channel).cryptoDelete(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .cryptoDelete(toProtobuf())
    }
}

extension AccountDeleteTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_CryptoDeleteTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            accountId?.toProtobufInto(&proto.deleteAccountID)
            transferAccountId?.toProtobufInto(&proto.transferAccountID)
        }
    }
}

extension AccountDeleteTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .cryptoDelete(toProtobuf())
    }
}
