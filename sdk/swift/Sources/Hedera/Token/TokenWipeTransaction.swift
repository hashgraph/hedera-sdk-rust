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

/// Wipes the provided amount of tokens from the specified account.
public final class TokenWipeTransaction: Transaction {
    /// Create a new `TokenWipeTransaction`.
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

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_TokenWipeAccountTransactionBody) throws {
        self.tokenId = data.hasToken ? .fromProtobuf(data.token) : nil
        self.accountId = data.hasAccount ? try .fromProtobuf(data.account) : nil
        self.amount = data.amount
        self.serials = data.serialNumbers.map(UInt64.init)

        try super.init(protobuf: proto)
    }

    /// The token for which to wipe tokens.
    public var tokenId: TokenId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token for which to wipe tokens.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The account to be wiped.
    public var accountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account to be wiped.
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId

        return self
    }

    /// The amount of a fungible token to wipe from the specified account.
    public var amount: UInt64 = 0 {
        willSet {
            ensureNotFrozen()
        }
    }

    //// Sets the amount of a fungible token to wipe from the specified account.
    @discardableResult
    public func amount(_ amount: UInt64) -> Self {
        self.amount = amount

        return self
    }

    /// The serial numbers of a non-fungible token to wipe from the specified account.
    public var serials: [UInt64] = [] {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the serial numbers of a non-fungible token to wipe from the specified account.
    @discardableResult
    public func serials(_ serials: [UInt64]) -> Self {
        self.serials = serials

        return self
    }

    /// Add a serial number of a non-fungible token to wipe from the specified account
    @discardableResult
    public func addSerial(_ serial: UInt64) -> Self {
        self.serials.append(serial)

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_TokenServiceAsyncClient(channel: channel).wipeTokenAccount(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .tokenWipe(toProtobuf())
    }
}

extension TokenWipeTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_TokenWipeAccountTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            tokenId?.toProtobufInto(&proto.token)
            accountId?.toProtobufInto(&proto.account)
            proto.amount = amount
            proto.serialNumbers = serials.map(Int64.init(bitPattern:))

        }
    }
}

extension TokenWipeTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .tokenWipe(toProtobuf())
    }
}
