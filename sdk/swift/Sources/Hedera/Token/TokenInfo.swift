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

import CHedera
import Foundation
import HederaProtobufs

public final class TokenInfo: Codable {
    internal init(
        tokenId: TokenId,
        name: String,
        symbol: String,
        decimals: UInt32,
        totalSupply: UInt64,
        treasuryAccountId: AccountId,
        adminKey: Key?,
        kycKey: Key?,
        freezeKey: Key?,
        wipeKey: Key?,
        supplyKey: Key?,
        feeScheduleKey: Key?,
        defaultFreezeStatus: Bool?,
        defaultKycStatus: Bool?,
        isDeleted: Bool,
        autoRenewAccount: AccountId?,
        autoRenewPeriod: Duration?,
        expirationTime: Timestamp?,
        tokenMemo: String,
        tokenType: TokenType,
        supplyType: TokenSupplyType,
        maxSupply: UInt64,
        customFees: [AnyCustomFee],
        pauseKey: Key?,
        pauseStatus: Bool?,
        ledgerId: LedgerId
    ) {
        self.tokenId = tokenId
        self.name = name
        self.symbol = symbol
        self.decimals = decimals
        self.totalSupply = totalSupply
        self.treasuryAccountId = treasuryAccountId
        self.adminKey = adminKey
        self.kycKey = kycKey
        self.freezeKey = freezeKey
        self.wipeKey = wipeKey
        self.supplyKey = supplyKey
        self.feeScheduleKey = feeScheduleKey
        self.defaultFreezeStatus = defaultFreezeStatus
        self.defaultKycStatus = defaultKycStatus
        self.isDeleted = isDeleted
        self.autoRenewAccount = autoRenewAccount
        self.autoRenewPeriod = autoRenewPeriod
        self.expirationTime = expirationTime
        self.tokenMemo = tokenMemo
        self.tokenType = tokenType
        self.supplyType = supplyType
        self.maxSupply = maxSupply
        self.customFees = customFees
        self.pauseKey = pauseKey
        self.pauseStatus = pauseStatus
        self.ledgerId = ledgerId
    }

    /// The ID of the token for which information is requested.
    public let tokenId: TokenId

    /// Name of token.
    public let name: String

    /// Symbol of token.
    public let symbol: String

    /// The amount of decimal places that this token supports.
    public let decimals: UInt32

    /// Total Supply of token.
    public let totalSupply: UInt64

    /// The ID of the account which is set as Treasury.
    public let treasuryAccountId: AccountId

    /// The key which can perform update/delete operations on the token.
    public let adminKey: Key?

    /// The key which can grant or revoke KYC of an account for the token's transactions.
    public let kycKey: Key?

    /// The key which can freeze or unfreeze an account for token transactions.
    public let freezeKey: Key?

    /// The key which can wipe token balance of an account.
    public let wipeKey: Key?

    /// The key which can change the supply of a token.
    public let supplyKey: Key?

    /// The key which can change the custom fees of the token.
    public let feeScheduleKey: Key?

    /// The default Freeze status (not applicable, frozen or unfrozen)
    public let defaultFreezeStatus: Bool?

    /// The default KYC status (KycNotApplicable or Revoked) of Hedera accounts relative to this token.
    public let defaultKycStatus: Bool?

    /// Specifies whether the token was deleted or not.
    public let isDeleted: Bool

    /// An account which will be automatically charged to renew the token's expiration,
    /// at autoRenewPeriod interval.
    public let autoRenewAccount: AccountId?

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    public let autoRenewPeriod: Duration?

    /// The epoch second at which the token will expire
    public let expirationTime: Timestamp?

    /// The memo associated with the token
    public let tokenMemo: String

    /// The token type.
    public let tokenType: TokenType

    /// The token supply type
    public let supplyType: TokenSupplyType

    /// The Maximum number of tokens that can be in circulation.
    public let maxSupply: UInt64

    /// The custom fees to be assessed during a transfer that transfers units of this token.
    public let customFees: [AnyCustomFee]

    /// The Key which can pause and unpause the Token.
    public let pauseKey: Key?

    /// Specifies whether the token is paused or not.
    public let pauseStatus: Bool?

    /// The ledger ID the response was returned from
    public let ledgerId: LedgerId

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        self.toProtobufBytes()
    }
}

extension TokenInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_TokenInfo

    internal convenience init(fromProtobuf proto: Protobuf) throws {
        let adminKey = proto.hasAdminKey ? proto.adminKey : nil
        let kycKey = proto.hasKycKey ? proto.kycKey : nil
        let freezeKey = proto.hasFreezeKey ? proto.freezeKey : nil
        let wipeKey = proto.hasWipeKey ? proto.wipeKey : nil
        let supplyKey = proto.hasSupplyKey ? proto.supplyKey : nil
        let feeScheduleKey = proto.hasFeeScheduleKey ? proto.feeScheduleKey : nil

        let defaultFreezeStatus: Bool?
        switch proto.defaultFreezeStatus {
        case .freezeNotApplicable:
            defaultFreezeStatus = nil
        case .frozen:
            defaultFreezeStatus = true
        case .unfrozen:
            defaultFreezeStatus = false
        case .UNRECOGNIZED(let value):
            throw HError(
                kind: .fromProtobuf,
                description: "Unrecognized defaultFreezeStatus: `\(value)`"
            )
        }

        let defaultKycStatus: Bool?
        switch proto.defaultKycStatus {
        case .kycNotApplicable:
            defaultKycStatus = nil
        case .granted:
            defaultKycStatus = true
        case .revoked:
            defaultKycStatus = false
        case .UNRECOGNIZED(let value):
            throw HError(
                kind: .fromProtobuf,
                description: "Unrecognized defaultKycStatus: `\(value)`"
            )
        }

        let autoRenewAccount = proto.hasAutoRenewAccount ? proto.autoRenewAccount : nil
        let autoRenewPeriod = proto.hasAutoRenewPeriod ? proto.autoRenewPeriod : nil
        let expirationTime = proto.hasExpiry ? proto.expiry : nil
        let pauseKey = proto.hasPauseKey ? proto.pauseKey : nil

        let pauseStatus: Bool?

        switch proto.pauseStatus {

        case .pauseNotApplicable:
            pauseStatus = nil
        case .paused:
            pauseStatus = true
        case .unpaused:
            pauseStatus = false
        case .UNRECOGNIZED(let value):
            throw HError(
                kind: .fromProtobuf,
                description: "Unrecognized pauseStatus: `\(value)`"
            )

        }

        self.init(
            tokenId: .fromProtobuf(proto.tokenID),
            name: proto.name,
            symbol: proto.symbol,
            decimals: proto.decimals,
            totalSupply: proto.totalSupply,
            treasuryAccountId: try .fromProtobuf(proto.treasury),
            adminKey: try .fromProtobuf(adminKey),
            kycKey: try .fromProtobuf(kycKey),
            freezeKey: try .fromProtobuf(freezeKey),
            wipeKey: try .fromProtobuf(wipeKey),
            supplyKey: try .fromProtobuf(supplyKey),
            feeScheduleKey: try .fromProtobuf(feeScheduleKey),
            defaultFreezeStatus: defaultFreezeStatus,
            defaultKycStatus: defaultKycStatus,
            isDeleted: proto.deleted,
            autoRenewAccount: try .fromProtobuf(autoRenewAccount),
            autoRenewPeriod: .fromProtobuf(autoRenewPeriod),
            expirationTime: .fromProtobuf(expirationTime),
            tokenMemo: proto.memo,
            tokenType: try .fromProtobuf(proto.tokenType),
            supplyType: try .fromProtobuf(proto.supplyType),
            maxSupply: UInt64(proto.maxSupply),
            customFees: try .fromProtobuf(proto.customFees),
            pauseKey: try .fromProtobuf(pauseKey),
            pauseStatus: pauseStatus,
            ledgerId: LedgerId(proto.ledgerID)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.tokenID = tokenId.toProtobuf()
            proto.name = name
            proto.symbol = symbol
            proto.decimals = decimals
            proto.totalSupply = totalSupply
            proto.treasury = treasuryAccountId.toProtobuf()

            adminKey?.toProtobufInto(&proto.adminKey)
            freezeKey?.toProtobufInto(&proto.freezeKey)
            wipeKey?.toProtobufInto(&proto.wipeKey)
            supplyKey?.toProtobufInto(&proto.supplyKey)
            feeScheduleKey?.toProtobufInto(&proto.feeScheduleKey)

            switch defaultFreezeStatus {
            case .none:
                proto.defaultFreezeStatus = .freezeNotApplicable

            case .some(true):
                proto.defaultFreezeStatus = .frozen

            case .some(false):
                proto.defaultFreezeStatus = .unfrozen
            }

            switch defaultKycStatus {
            case .none:
                proto.defaultKycStatus = .kycNotApplicable

            case .some(true):
                proto.defaultKycStatus = .granted

            case .some(false):
                proto.defaultKycStatus = .revoked
            }

            proto.deleted = isDeleted

            autoRenewAccount?.toProtobufInto(&proto.autoRenewAccount)
            autoRenewPeriod?.toProtobufInto(&proto.autoRenewPeriod)
            expirationTime?.toProtobufInto(&proto.expiry)

            proto.memo = tokenMemo
            proto.tokenType = tokenType.toProtobuf()
            proto.supplyType = supplyType.toProtobuf()
            proto.maxSupply = Int64(maxSupply)
            proto.customFees = customFees.toProtobuf()

            pauseKey?.toProtobufInto(&proto.pauseKey)

            switch pauseStatus {
            case .none:
                proto.pauseStatus = .pauseNotApplicable

            case .some(true):
                proto.pauseStatus = .paused

            case .some(false):
                proto.pauseStatus = .unpaused
            }

            proto.ledgerID = ledgerId.bytes
        }
    }
}
