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

/// At consensus, updates a token type's fee schedule to the given list of custom fees.
public final class TokenFeeScheduleUpdateTransaction: Transaction {
    /// Create a new `TokenFeeScheduleUpdateTransaction`.
    public init(
        tokenId: TokenId? = nil,
        customFees: [AnyCustomFee] = []
    ) {
        self.tokenId = tokenId
        self.customFees = customFees

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_TokenFeeScheduleUpdateTransactionBody) throws {
        self.tokenId = data.hasTokenID ? .fromProtobuf(data.tokenID) : nil
        self.customFees = try .fromProtobuf(data.customFees)

        try super.init(protobuf: proto)

    }

    /// The token whose fee schedule is to be updated.
    public var tokenId: TokenId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token whose fee schedule is to be updated.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The new custom fees to be assessed during a transfer.
    public var customFees: [AnyCustomFee] {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new custom fees to be assessed during a transfer.
    @discardableResult
    public func customFees(_ customFees: [AnyCustomFee]) -> Self {
        self.customFees = customFees

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)
        try customFees.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_TokenServiceAsyncClient(channel: channel).updateTokenFeeSchedule(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .tokenFeeScheduleUpdate(toProtobuf())
    }
}

extension TokenFeeScheduleUpdateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_TokenFeeScheduleUpdateTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            tokenId?.toProtobufInto(&proto.tokenID)
            proto.customFees = customFees.toProtobuf()
        }
    }
}

extension TokenFeeScheduleUpdateTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .tokenFeeScheduleUpdate(toProtobuf())
    }
}
