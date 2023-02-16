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

// every single bit of this file is all required and can't be moved into a different file :/
// swiftlint:disable file_length type_body_length

import Foundation

/// Create a new token.
public final class TokenCreateTransaction: Transaction {
    /// Create a new `TokenCreateTransaction`.
    public init(
        name: String = "",
        symbol: String = "",
        decimals: UInt32 = 0,
        initialSupply: UInt64 = 0,
        treasuryAccountId: AccountId? = nil,
        adminKey: Key? = nil,
        kycKey: Key? = nil,
        freezeKey: Key? = nil,
        wipeKey: Key? = nil,
        supplyKey: Key? = nil,
        freezeDefault: Bool = false,
        expirationTime: Timestamp? = nil,
        autoRenewAccountId: AccountId? = nil,
        autoRenewPeriod: Duration? = nil,
        tokenMemo: String = "",
        tokenType: TokenType = .fungibleCommon,
        tokenSupplyType: TokenSupplyType = .infinite,
        maxSupply: UInt64 = 0,
        feeScheduleKey: Key? = nil,
        customFees: [AnyCustomFee] = [],
        pauseKey: Key? = nil
    ) {
        self.name = name
        self.symbol = symbol
        self.decimals = decimals
        self.initialSupply = initialSupply
        self.treasuryAccountId = treasuryAccountId
        self.adminKey = adminKey
        self.kycKey = kycKey
        self.freezeKey = freezeKey
        self.wipeKey = wipeKey
        self.supplyKey = supplyKey
        self.freezeDefault = freezeDefault
        self.expirationTime = expirationTime
        self.autoRenewAccountId = autoRenewAccountId
        self.autoRenewPeriod = autoRenewPeriod
        self.tokenMemo = tokenMemo
        self.tokenType = tokenType
        self.tokenSupplyType = tokenSupplyType
        self.maxSupply = maxSupply
        self.feeScheduleKey = feeScheduleKey
        self.customFees = customFees
        self.pauseKey = pauseKey

        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        name = try container.decodeIfPresent(.name) ?? ""
        symbol = try container.decodeIfPresent(.symbol) ?? ""
        decimals = try container.decodeIfPresent(.decimals) ?? 0
        initialSupply = try container.decodeIfPresent(.initialSupply) ?? 0
        treasuryAccountId = try container.decodeIfPresent(.treasuryAccountId)
        adminKey = try container.decodeIfPresent(.adminKey)
        kycKey = try container.decodeIfPresent(.kycKey)
        freezeKey = try container.decodeIfPresent(.freezeKey)
        wipeKey = try container.decodeIfPresent(.wipeKey)
        supplyKey = try container.decodeIfPresent(.supplyKey)
        freezeDefault = try container.decodeIfPresent(.freezeDefault) ?? false
        expirationTime = try container.decodeIfPresent(.expirationTime)
        autoRenewAccountId = try container.decodeIfPresent(.autoRenewAccountId)
        autoRenewPeriod = try container.decodeIfPresent(.autoRenewPeriod)
        tokenMemo = try container.decodeIfPresent(.tokenMemo) ?? ""
        tokenType = try container.decodeIfPresent(.tokenType) ?? .fungibleCommon
        tokenSupplyType = try container.decodeIfPresent(.tokenSupplyType) ?? .infinite
        maxSupply = try container.decodeIfPresent(.maxSupply) ?? 0
        feeScheduleKey = try container.decodeIfPresent(.feeScheduleKey)
        customFees = try container.decodeIfPresent(.customFees) ?? []
        pauseKey = try container.decodeIfPresent(.pauseKey)

        try super.init(from: decoder)
    }

    /// The publicly visible name of the token.
    public var name: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the publicly visible name of the token.
    @discardableResult
    public func name(_ name: String) -> Self {
        self.name = name

        return self
    }

    /// The publicly visible token symbol.
    public var symbol: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the publicly visible token symbol.
    @discardableResult
    public func symbol(_ symbol: String) -> Self {
        self.symbol = symbol

        return self
    }

    /// The number of decimal places a fungible token is divisible by.
    public var decimals: UInt32 {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the number of decimal places a fungible token is divisible by.
    @discardableResult
    public func decimals(_ decimals: UInt32) -> Self {
        self.decimals = decimals

        return self
    }

    /// The initial supply of fungible tokens to to mint to the treasury account.
    public var initialSupply: UInt64 {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the initial supply of fungible tokens to to mint to the treasury account.
    @discardableResult
    public func initialSupply(_ initialSupply: UInt64) -> Self {
        self.initialSupply = initialSupply

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

    /// The default freeze status (frozen or unfrozen) of Hedera accounts relative to this token. If
    /// true, an account must be unfrozen before it can receive the token.
    public var freezeDefault: Bool {
        willSet { ensureNotFrozen() }
    }

    /// Sets the default freeze status (frozen or unfrozen) of Hedera accounts relative to this token.
    @discardableResult
    public func freezeDefault(_ freezeDefault: Bool) -> Self {
        self.freezeDefault = freezeDefault

        return self
    }

    /// The time at which the token should expire.
    public var expirationTime: Timestamp? {
        willSet { ensureNotFrozen() }
    }

    /// Sets the time at which the token should expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// An account which will be automatically charged to renew the token's expiration, at
    /// `autoRenewPeriod` interval.
    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account which will be automatically charged to renew the token's expiration, at
    /// `autoRenewPeriod` interval.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The interval at which the auto-renew account will be charged to extend the token's expiry.
    public var autoRenewPeriod: Duration? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the interval at which the auto-renew account will be charged to extend the token's expiry.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The memo associated with the token.
    public var tokenMemo: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the memo associated with the token.
    @discardableResult
    public func tokenMemo(_ tokenMemo: String) -> Self {
        self.tokenMemo = tokenMemo

        return self
    }

    /// The token type. Defaults to FungibleCommon.
    public var tokenType: TokenType {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token type.
    @discardableResult
    public func tokenType(_ tokenType: TokenType) -> Self {
        self.tokenType = tokenType

        return self
    }

    /// The token supply type. Defaults to Infinite.
    public var tokenSupplyType: TokenSupplyType {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the token supply type.
    @discardableResult
    public func tokenSupplyType(_ tokenSupplyType: TokenSupplyType) -> Self {
        self.tokenSupplyType = tokenSupplyType

        return self
    }

    /// The maximum number of tokens that can be in circulation.
    public var maxSupply: UInt64 {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the maximum number of tokens that can be in circulation.
    @discardableResult
    public func maxSupply(_ maxSupply: UInt64) -> Self {
        self.maxSupply = maxSupply

        return self
    }

    /// The key which can change the token's custom fee schedule.
    public var feeScheduleKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key which can change the token's custom fee schedule.
    @discardableResult
    public func feeScheduleKey(_ feeScheduleKey: Key) -> Self {
        self.feeScheduleKey = feeScheduleKey

        return self
    }

    /// The custom fees to be assessed during a transfer.
    public var customFees: [AnyCustomFee] {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the custom fees to be assessed during a transfer.
    @discardableResult
    public func customFees(_ customFees: [AnyCustomFee]) -> Self {
        self.customFees = customFees

        return self
    }

    /// The key which can pause and unpause the token.
    public var pauseKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the key which can pause and unpause the token.
    @discardableResult
    public func pauseKey(_ pauseKey: Key) -> Self {
        self.pauseKey = pauseKey

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case name
        case symbol
        case decimals
        case initialSupply
        case treasuryAccountId
        case adminKey
        case kycKey
        case freezeKey
        case wipeKey
        case supplyKey
        case freezeDefault
        case expirationTime
        case autoRenewAccountId
        case autoRenewPeriod
        case tokenMemo
        case tokenType
        case tokenSupplyType
        case maxSupply
        case feeScheduleKey
        case customFees
        case pauseKey
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(name, forKey: .name)
        try container.encode(symbol, forKey: .symbol)
        try container.encode(decimals, forKey: .decimals)
        try container.encode(initialSupply, forKey: .initialSupply)
        try container.encodeIfPresent(treasuryAccountId, forKey: .treasuryAccountId)
        try container.encodeIfPresent(adminKey, forKey: .adminKey)
        try container.encodeIfPresent(kycKey, forKey: .kycKey)
        try container.encodeIfPresent(freezeKey, forKey: .freezeKey)
        try container.encodeIfPresent(wipeKey, forKey: .wipeKey)
        try container.encodeIfPresent(supplyKey, forKey: .supplyKey)
        try container.encode(freezeDefault, forKey: .freezeDefault)
        try container.encodeIfPresent(expirationTime, forKey: .expirationTime)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try container.encode(tokenMemo, forKey: .tokenMemo)
        try container.encode(tokenType, forKey: .tokenType)
        try container.encode(tokenSupplyType, forKey: .tokenSupplyType)
        try container.encode(maxSupply, forKey: .maxSupply)
        try container.encodeIfPresent(feeScheduleKey, forKey: .feeScheduleKey)
        try container.encode(customFees, forKey: .customFees)
        try container.encodeIfPresent(pauseKey, forKey: .pauseKey)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try treasuryAccountId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try customFees.validateChecksums(on: ledgerId)

        try super.validateChecksums(on: ledgerId)
    }
}
