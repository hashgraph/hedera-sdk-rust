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

public final class ContractInfo: Codable {
    internal init(
        contractId: ContractId,
        accountId: AccountId,
        contractAccountId: String,
        adminKey: Key?,
        expirationTime: Timestamp?,
        autoRenewPeriod: Duration?,
        storage: UInt64,
        contractMemo: String,
        balance: Hbar,
        isDeleted: Bool,
        autoRenewAccountId: AccountId?,
        maxAutomaticTokenAssociations: UInt32,
        ledgerId: LedgerId,
        stakingInfo: StakingInfo
    ) {
        self.contractId = contractId
        self.accountId = accountId
        self.contractAccountId = contractAccountId
        self.adminKey = adminKey
        self.expirationTime = expirationTime
        self.autoRenewPeriod = autoRenewPeriod
        self.storage = storage
        self.contractMemo = contractMemo
        self.balance = balance
        self.isDeleted = isDeleted
        self.autoRenewAccountId = autoRenewAccountId
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.ledgerId = ledgerId
        self.stakingInfo = stakingInfo
    }

    /// ID of the contract instance, in the format used by transactions.
    public let contractId: ContractId

    /// ID of the cryptocurrency account owned by the contract instance,
    /// in the format used in transactions.
    public let accountId: AccountId

    /// ID of both the contract instance and the cryptocurrency account owned by the contract
    /// instance, in the format used by Solidity.
    public let contractAccountId: String

    /// The admin key of the contract instance.
    public let adminKey: Key?

    /// The current time at which this contract instance (and its account) is set to expire.
    public let expirationTime: Timestamp?

    /// The auto renew period for this contract instance.
    public let autoRenewPeriod: Duration?

    /// Number of bytes of storage being used by this instance.
    public let storage: UInt64

    /// The memo associated with the contract.
    public let contractMemo: String

    /// The current balance, in tinybars.
    public let balance: Hbar

    /// Whether the contract has been deleted.
    public let isDeleted: Bool

    /// ID of the an account to charge for auto-renewal of this contract.
    public let autoRenewAccountId: AccountId?

    /// The maximum number of tokens that a contract can be implicitly associated with.
    public let maxAutomaticTokenAssociations: UInt32

    public let ledgerId: LedgerId

    /// Staking metadata for this contract.
    public let stakingInfo: StakingInfo

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension ContractInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_ContractGetInfoResponse.ContractInfo

    internal convenience init(fromProtobuf proto: Protobuf) throws {
        let adminKey = proto.hasAdminKey ? proto.adminKey : nil
        let expirationTime = proto.hasExpirationTime ? proto.expirationTime : nil
        let autoRenewPeriod = proto.hasAutoRenewPeriod ? proto.autoRenewPeriod : nil
        let autoRenewAccountId = proto.hasAutoRenewAccountID ? proto.autoRenewAccountID : nil

        self.init(
            contractId: try .fromProtobuf(proto.contractID),
            accountId: try .fromProtobuf(proto.accountID),
            contractAccountId: proto.contractAccountID,
            adminKey: try .fromProtobuf(adminKey),
            expirationTime: .fromProtobuf(expirationTime),
            autoRenewPeriod: .fromProtobuf(autoRenewPeriod),
            storage: UInt64(proto.storage),
            contractMemo: proto.memo,
            balance: .fromTinybars(Int64(proto.balance)),
            isDeleted: proto.deleted,
            autoRenewAccountId: try .fromProtobuf(autoRenewAccountId),
            maxAutomaticTokenAssociations: UInt32(proto.maxAutomaticTokenAssociations),
            ledgerId: .fromBytes(proto.ledgerID),
            stakingInfo: try .fromProtobuf(proto.stakingInfo)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.contractID = contractId.toProtobuf()
            proto.accountID = accountId.toProtobuf()
            proto.contractAccountID = contractAccountId
            adminKey?.toProtobufInto(&proto.adminKey)
            expirationTime?.toProtobufInto(&proto.expirationTime)
            autoRenewPeriod?.toProtobufInto(&proto.autoRenewPeriod)
            proto.storage = Int64(storage)
            proto.memo = contractMemo
            proto.balance = UInt64(balance.toTinybars())
            proto.deleted = isDeleted
            autoRenewAccountId?.toProtobufInto(&proto.autoRenewAccountID)
            proto.maxAutomaticTokenAssociations = Int32(maxAutomaticTokenAssociations)
            proto.ledgerID = ledgerId.bytes
            proto.stakingInfo = stakingInfo.toProtobuf()
        }
    }
}
