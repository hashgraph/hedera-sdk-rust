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

/// Burns tokens from the token's treasury account.
public final class TokenBurnTransaction: Transaction {
    /// Create a new `TokenBurnTransaction`.
    public init(
        tokenId: TokenId? = nil,
        amount: UInt64 = 0,
        serials: [UInt64] = []
    ) {
        self.tokenId = tokenId
        self.amount = amount
        self.serials = serials

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_TokenBurnTransactionBody) throws {
        self.tokenId = data.hasToken ? .fromProtobuf(data.token) : nil
        self.amount = data.amount
        self.serials = data.serialNumbers.map(UInt64.init)

        try super.init(protobuf: proto)

    }

    /// The token for which to burn tokens.
    public var tokenId: TokenId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token for which to burn tokens.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The amount of a fungible token to burn from the treasury account.
    public var amount: UInt64 {
        willSet {
            ensureNotFrozen()
        }
    }

    //// Sets the amount of a fungible token to burn from the treasury account.
    @discardableResult
    public func amount(_ amount: UInt64) -> Self {
        self.amount = amount

        return self
    }

    /// The serial numbers of a non-fungible token to burn from the treasury account.
    public var serials: [UInt64] {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the serial numbers of a non-fungible token to burn from the treasury account.
    @discardableResult
    public func setSerials(_ serials: [UInt64]) -> Self {
        self.serials = serials

        return self
    }

    /// Add a serial number to the list of serial numbers.
    @discardableResult
    public func addSerial(_ serial: UInt64) -> Self {
        serials.append(serial)

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)

        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_TokenServiceAsyncClient(channel: channel).burnToken(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .tokenBurn(toProtobuf())
    }
}

extension TokenBurnTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_TokenBurnTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            tokenId?.toProtobufInto(&proto.token)
            proto.amount = amount
            proto.serialNumbers = serials.map(Int64.init(bitPattern:))
        }
    }
}

extension TokenBurnTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .tokenBurn(toProtobuf())
    }
}
