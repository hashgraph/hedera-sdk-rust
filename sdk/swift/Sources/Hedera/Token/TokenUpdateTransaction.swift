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
import SwiftProtobuf

/// At consensus, updates an already created token to the given values.
public final class TokenUpdateTransaction: Transaction {
    /// Create a new `TokenUpdateTransaction`.
    public init(
        tokenId: TokenId? = nil,
        tokenName: String = "",
        tokenSymbol: String = "",
        treasuryAccountId: AccountId? = nil,
        adminKey: Key? = nil,
        kycKey: Key? = nil,
        freezeKey: Key? = nil,
        wipeKey: Key? = nil,
        supplyKey: Key? = nil,
        autoRenewAccountId: AccountId? = nil,
        autoRenewPeriod: Duration? = nil,
        expirationTime: Timestamp? = nil,
        tokenMemo: String = "",
        feeScheduleKey: Key? = nil,
        pauseKey: Key? = nil
    ) {
        self.tokenId = tokenId
        self.tokenName = tokenName
        self.tokenSymbol = tokenSymbol
        self.treasuryAccountId = treasuryAccountId
        self.adminKey = adminKey
        self.kycKey = kycKey
        self.freezeKey = freezeKey
        self.wipeKey = wipeKey
        self.supplyKey = supplyKey
        self.autoRenewAccountId = autoRenewAccountId
        self.autoRenewPeriod = autoRenewPeriod
        self.expirationTime = expirationTime
        self.tokenMemo = tokenMemo
        self.feeScheduleKey = feeScheduleKey
        self.pauseKey = pauseKey

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_TokenUpdateTransactionBody) throws {
        self.tokenId = data.hasToken ? .fromProtobuf(data.token) : nil
        self.tokenName = data.name
        self.tokenSymbol = data.symbol
        self.treasuryAccountId = data.hasTreasury ? try .fromProtobuf(data.treasury) : nil
        self.adminKey = data.hasAdminKey ? try .fromProtobuf(data.adminKey) : nil
        self.kycKey = data.hasKycKey ? try .fromProtobuf(data.kycKey) : nil
        self.freezeKey = data.hasFreezeKey ? try .fromProtobuf(data.freezeKey) : nil
        self.wipeKey = data.hasWipeKey ? try .fromProtobuf(data.wipeKey) : nil
        self.supplyKey = data.hasSupplyKey ? try .fromProtobuf(data.supplyKey) : nil
        self.autoRenewAccountId = data.hasAutoRenewAccount ? try .fromProtobuf(data.autoRenewAccount) : nil
        self.autoRenewPeriod = data.hasAutoRenewPeriod ? .fromProtobuf(data.autoRenewPeriod) : nil
        self.expirationTime = data.hasExpiry ? .fromProtobuf(data.expiry) : nil
        self.tokenMemo = data.hasMemo ? data.memo.value : nil ?? ""
        self.feeScheduleKey = data.hasFeeScheduleKey ? try .fromProtobuf(data.feeScheduleKey) : nil
        self.pauseKey = data.hasPauseKey ? try .fromProtobuf(data.pauseKey) : nil

        try super.init(protobuf: proto)
    }

    /// The token to be updated.
    public var tokenId: TokenId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token to be updated.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    /// The publicly visible name of the token.
    public var tokenName: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the publicly visible name of the token.
    @discardableResult
    public func tokenName(_ tokenName: String) -> Self {
        self.tokenName = tokenName

        return self
    }

    /// The publicly visible token symbol.
    public var tokenSymbol: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the publicly visible token symbol.
    @discardableResult
    public func tokenSymbol(_ tokenSymbol: String) -> Self {
        self.tokenSymbol = tokenSymbol

        return self
    }

    /// The account which will act as a treasury for the token.
    public var treasuryAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account which will act as a treasury for the token.
    @discardableResult
    public func treasuryAccountId(_ treasuryAccountId: AccountId) -> Self {
        self.treasuryAccountId = treasuryAccountId

        return self
    }

    /// The key which can perform update/delete operations on the token.
    public var adminKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key which can perform update/delete operations on the token.
    @discardableResult
    public func adminKey(_ adminKey: Key) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The key which can grant or revoke KYC of an account for the token's transactions.
    public var kycKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key which can grant or revoke KYC of an account for the token's transactions.
    @discardableResult
    public func kycKey(_ kycKey: Key) -> Self {
        self.kycKey = kycKey

        return self
    }

    /// The key which can sign to freeze or unfreeze an account for token transactions.
    public var freezeKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key which can sign to freeze or unfreeze an account for token transactions.
    @discardableResult
    public func freezeKey(_ freezeKey: Key) -> Self {
        self.freezeKey = freezeKey

        return self
    }

    /// The key which can wipe the token balance of an account.
    public var wipeKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key which can wipe the token balance of an account.
    @discardableResult
    public func wipeKey(_ wipeKey: Key) -> Self {
        self.wipeKey = wipeKey

        return self
    }

    /// The key which can change the supply of a token.
    public var supplyKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key which can change the supply of a token.
    @discardableResult
    public func supplyKey(_ supplyKey: Key) -> Self {
        self.supplyKey = supplyKey

        return self
    }

    /// The new account which will be automatically charged to renew the token's expiration.
    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new account which will be automatically charged to renew the token's expiration.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The new interval at which the auto renew account will be charged to extend
    /// the token's expiry.
    public var autoRenewPeriod: Duration? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new interval at which the auto renew account will be charged to extend
    /// the token's expiry.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The new time at which the token should expire.
    public var expirationTime: Timestamp? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new time at which the token should expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// The new memo associated with the token (UTF-8 encoding max 100 bytes).
    public var tokenMemo: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new memo associated with the token (UTF-8 encoding max 100 bytes).
    @discardableResult
    public func tokenMemo(_ tokenMemo: String) -> Self {
        self.tokenMemo = tokenMemo

        return self
    }

    @discardableResult
    public func clearMemo() -> Self {
        tokenMemo = ""

        return self
    }

    /// The new key which can change the token's custom fee schedule.
    public var feeScheduleKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new key which can change the token's custom fee schedule.
    @discardableResult
    public func feeScheduleKey(_ feeScheduleKey: Key) -> Self {
        self.feeScheduleKey = feeScheduleKey

        return self
    }

    /// The new key which can pause and unpause the Token.
    public var pauseKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new key which can pause and unpause the Token.
    @discardableResult
    public func pauseKey(_ pauseKey: Key) -> Self {
        self.pauseKey = pauseKey

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)
        try treasuryAccountId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_TokenServiceAsyncClient(channel: channel).updateToken(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .tokenUpdate(toProtobuf())
    }
}

extension TokenUpdateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_TokenUpdateTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            tokenId?.toProtobufInto(&proto.token)
            proto.name = tokenName
            proto.symbol = tokenSymbol
            treasuryAccountId?.toProtobufInto(&proto.treasury)
            adminKey?.toProtobufInto(&proto.adminKey)
            kycKey?.toProtobufInto(&proto.kycKey)
            freezeKey?.toProtobufInto(&proto.freezeKey)
            wipeKey?.toProtobufInto(&proto.wipeKey)
            supplyKey?.toProtobufInto(&proto.supplyKey)
            autoRenewAccountId?.toProtobufInto(&proto.autoRenewAccount)
            autoRenewPeriod?.toProtobufInto(&proto.autoRenewPeriod)
            expirationTime?.toProtobufInto(&proto.expiry)
            proto.memo = Google_Protobuf_StringValue(tokenMemo)
            feeScheduleKey?.toProtobufInto(&proto.feeScheduleKey)
            pauseKey?.toProtobufInto(&proto.pauseKey)
        }
    }
}

extension TokenUpdateTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .tokenUpdate(toProtobuf())
    }
}
