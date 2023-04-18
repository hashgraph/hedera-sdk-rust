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

/// Unfreezes transfers of the specified token for the account.
public final class TokenUnfreezeTransaction: Transaction {
    /// Create a new `TokenUnfreezeTransaction`.
    public init(
        accountId: AccountId? = nil,
        tokenId: TokenId? = nil
    ) {
        self.accountId = accountId
        self.tokenId = tokenId

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_TokenUnfreezeAccountTransactionBody) throws {
        self.tokenId = data.hasToken ? .fromProtobuf(data.token) : nil
        self.accountId = data.hasAccount ? try .fromProtobuf(data.account) : nil

        try super.init(protobuf: proto)
    }

    /// The account to be unfrozen.
    public var accountId: AccountId? {
        willSet { ensureNotFrozen() }
    }

    /// Sets the account to be unfrozen.
    @discardableResult
    public func accountId(_ accountId: AccountId?) -> Self {
        self.accountId = accountId

        return self
    }

    /// The token for which this account will be unfrozen.
    public var tokenId: TokenId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token for which this account will be unfrozen.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try accountId?.validateChecksums(on: ledgerId)
        try tokenId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_TokenServiceAsyncClient(channel: channel).unfreezeTokenAccount(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .tokenUnfreeze(toProtobuf())
    }
}

extension TokenUnfreezeTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_TokenUnfreezeAccountTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            tokenId?.toProtobufInto(&proto.token)
            accountId?.toProtobufInto(&proto.account)
        }
    }
}

extension TokenUnfreezeTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .tokenUnfreeze(toProtobuf())
    }
}
