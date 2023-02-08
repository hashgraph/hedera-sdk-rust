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
import HederaProtobufs

/// Response from `AccountInfoQuery`.
public final class AccountInfo: Codable {
    internal init(
        accountId: AccountId,
        contractAccountId: String,
        isDeleted: Bool,
        proxyAccountId: AccountId?,
        proxyReceived: Hbar,
        key: Key,
        balance: Hbar,
        sendRecordThreshold: Hbar,
        receiveRecordThreshold: Hbar,
        isReceiverSignatureRequired: Bool,
        expirationTime: Timestamp?,
        autoRenewPeriod: Duration?,
        accountMemo: String,
        ownedNfts: UInt64,
        maxAutomaticTokenAssociations: UInt32,
        aliasKey: PublicKey?,
        ethereumNonce: UInt64,
        ledgerId: LedgerId,
        staking: StakingInfo?
    ) {
        self.accountId = accountId
        self.contractAccountId = contractAccountId
        self.isDeleted = isDeleted
        self.proxyAccountId = proxyAccountId
        self.proxyReceived = proxyReceived
        self.key = key
        self.balance = balance
        self.sendRecordThreshold = sendRecordThreshold
        self.receiveRecordThreshold = receiveRecordThreshold
        self.isReceiverSignatureRequired = isReceiverSignatureRequired
        self.expirationTime = expirationTime
        self.autoRenewPeriod = autoRenewPeriod
        self.accountMemo = accountMemo
        self.ownedNfts = ownedNfts
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.aliasKey = aliasKey
        self.ethereumNonce = ethereumNonce
        self.ledgerId = ledgerId
        self.staking = staking
    }

    /// The account that is being referenced.
    public let accountId: AccountId

    /// The Contract Account ID comprising of both the contract instance and the cryptocurrency
    /// account owned by the contract instance, in the format used by Solidity.
    public let contractAccountId: String

    /// If true, then this account has been deleted, it will disappear when it expires, and all
    /// transactions for it will fail except the transaction to extend its expiration date.
    public let isDeleted: Bool

    /// The Account ID of the account to which this is proxy staked.
    ///
    /// If `proxy_account_id` is `None`, an invalid account, or an account that isn't a node,
    /// then this account is automatically proxy staked to a node chosen by the network,
    /// but without earning payments.
    ///
    /// If the `proxy_account_id` account refuses to accept proxy staking, or if it is not currently
    /// running a node, then it will behave as if `proxy_account_id` is `None`.
    // @available(*, deprecated)
    public let proxyAccountId: AccountId?

    /// The total number of HBARs proxy staked to this account.
    public let proxyReceived: Hbar

    /// The key for the account, which must sign in order to transfer out, or to modify the
    /// account in any way other than extending its expiration date.
    public let key: Key

    /// Current balance of the referenced account.
    public let balance: Hbar

    /// The threshold amount for which an account record is created (and this account
    /// charged for them) for any send/withdraw transaction.
    // @available(*, deprecated)
    public let sendRecordThreshold: Hbar

    /// The threshold amount for which an account record is created
    /// (and this account charged for them) for any transaction above this amount.
    // @available(*, deprecated)
    public let receiveRecordThreshold: Hbar

    /// If true, no transaction can transfer to this account unless signed by
    /// this account's key.
    public let isReceiverSignatureRequired: Bool

    /// The TimeStamp time at which this account is set to expire.
    public let expirationTime: Timestamp?

    /// The duration for expiration time will extend every this many seconds.
    public let autoRenewPeriod: Duration?

    /// The memo associated with the account.
    public let accountMemo: String

    /// The number of NFTs owned by this account
    public let ownedNfts: UInt64

    /// The maximum number of tokens that an Account can be implicitly associated with.
    public let maxAutomaticTokenAssociations: UInt32

    /// The public key which aliases to this account.
    public let aliasKey: PublicKey?

    /// The ethereum transaction nonce associated with this account.
    public let ethereumNonce: UInt64

    /// The ledger ID the response was returned from.
    public let ledgerId: LedgerId

    /// Staking metadata for this account.
    public let staking: StakingInfo?

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        self.toProtobufBytes()
    }
}

extension AccountInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_CryptoGetInfoResponse.AccountInfo

    internal convenience init(fromProtobuf proto: Protobuf) throws {
        let expirationTime = proto.hasExpirationTime ? proto.expirationTime : nil
        let autoRenewPeriod = proto.hasAutoRenewPeriod ? proto.autoRenewPeriod : nil
        let staking = proto.hasStakingInfo ? proto.stakingInfo : nil
        let proxyAccountId = proto.hasProxyAccountID ? proto.proxyAccountID : nil

        self.init(
            accountId: try .fromProtobuf(proto.accountID),
            contractAccountId: proto.contractAccountID,
            isDeleted: proto.deleted,
            proxyAccountId: try .fromProtobuf(proxyAccountId),
            proxyReceived: Hbar.fromTinybars(proto.proxyReceived),
            key: try .fromProtobuf(proto.key),
            balance: .fromTinybars(Int64(proto.balance)),
            sendRecordThreshold: Hbar.fromTinybars(Int64(proto.generateSendRecordThreshold)),
            receiveRecordThreshold: Hbar.fromTinybars(Int64(proto.generateReceiveRecordThreshold)),
            isReceiverSignatureRequired: proto.receiverSigRequired,
            expirationTime: .fromProtobuf(expirationTime),
            autoRenewPeriod: .fromProtobuf(autoRenewPeriod),
            accountMemo: proto.memo,
            ownedNfts: UInt64(proto.ownedNfts),
            maxAutomaticTokenAssociations: UInt32(proto.maxAutomaticTokenAssociations),
            aliasKey: try .fromAliasBytes(proto.alias),
            ethereumNonce: UInt64(proto.ethereumNonce),
            ledgerId: .fromBytes(proto.ledgerID),
            staking: try .fromProtobuf(staking)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.accountID = accountId.toProtobuf()
            proto.contractAccountID = contractAccountId
            proto.deleted = isDeleted
            proxyAccountId?.toProtobufInto(&proto.proxyAccountID)
            proto.proxyReceived = proxyReceived.toTinybars()
            proto.key = key.toProtobuf()
            proto.balance = UInt64(balance.toTinybars())
            proto.generateSendRecordThreshold = UInt64(sendRecordThreshold.toTinybars())
            proto.generateReceiveRecordThreshold = UInt64(receiveRecordThreshold.toTinybars())
            proto.receiverSigRequired = isReceiverSignatureRequired
            expirationTime?.toProtobufInto(&proto.expirationTime)
            autoRenewPeriod?.toProtobufInto(&proto.autoRenewPeriod)
            proto.memo = accountMemo
            proto.ownedNfts = Int64(ownedNfts)
            proto.maxAutomaticTokenAssociations = Int32(maxAutomaticTokenAssociations)
            proto.alias = aliasKey?.toProtobufBytes() ?? Data()
            proto.ethereumNonce = Int64(ethereumNonce)
            proto.ledgerID = ledgerId.bytes
            staking?.toProtobufInto(&proto.stakingInfo)
        }
    }
}
