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

/// Unpauses a previously paused token.
public final class TokenUnpauseTransaction: Transaction {
    /// Create a new `TokenUnpauseTransaction`.
    public init(
        tokenId: TokenId? = nil
    ) {
        self.tokenId = tokenId

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_TokenUnpauseTransactionBody) throws {
        self.tokenId = data.hasToken ? .fromProtobuf(data.token) : nil

        try super.init(protobuf: proto)
    }

    /// The token to be paused.
    public var tokenId: TokenId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token to be paused.
    @discardableResult
    public func tokenId(_ tokenId: TokenId?) -> Self {
        self.tokenId = tokenId

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_TokenServiceAsyncClient(channel: channel).unpauseToken(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .tokenUnpause(toProtobuf())
    }
}

extension TokenUnpauseTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_TokenUnpauseTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            tokenId?.toProtobufInto(&proto.token)
        }
    }
}

extension TokenUnpauseTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .tokenUnpause(toProtobuf())
    }
}
