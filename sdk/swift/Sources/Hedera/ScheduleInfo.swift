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

/// Response from `ScheduleInfoQuery`.
public final class ScheduleInfo: Codable {
    internal init(
        scheduleId: ScheduleId,
        creatorAccountId: AccountId,
        payerAccountId: AccountId?,
        signatories: [Key],
        adminKey: Key?,
        scheduledTransactionId: TransactionId,
        waitForExpiry: Bool,
        scheduleMemo: String,
        expirationTime: Timestamp?,
        executedAt: Timestamp?,
        deletedAt: Timestamp?,
        ledgerId: LedgerId
    ) {
        self.scheduleId = scheduleId
        self.creatorAccountId = creatorAccountId
        self.payerAccountId = payerAccountId
        self.signatories = signatories
        self.adminKey = adminKey
        self.scheduledTransactionId = scheduledTransactionId
        self.waitForExpiry = waitForExpiry
        self.scheduleMemo = scheduleMemo
        self.expirationTime = expirationTime
        self.executedAt = executedAt
        self.deletedAt = deletedAt
        self.ledgerId = ledgerId
    }

    /// The ID of the schedule for which information is requested.
    public let scheduleId: ScheduleId

    /// The account that created the scheduled transaction.
    public let creatorAccountId: AccountId

    /// The account paying for the execution of the scheduled transaction.
    public let payerAccountId: AccountId?

    /// The signatories that have provided signatures so far for the schedule
    /// transaction.
    public let signatories: [Key]

    /// The key which is able to delete the schedule transaction if set.
    public let adminKey: Key?

    /// The transaction id that will be used in the record of the scheduled transaction (if
    /// it executes).
    public let scheduledTransactionId: TransactionId

    /// When set to true, the transaction will be evaluated for execution at `expiration_time`
    /// instead of when all required signatures are received.
    public let waitForExpiry: Bool

    /// Publicly visible information about the Schedule entity.
    public let scheduleMemo: String

    /// The date and time the schedule transaction will expire
    public let expirationTime: Timestamp?

    /// The time the schedule transaction was executed.
    public let executedAt: Timestamp?

    /// The time the schedule transaction was deleted.
    public let deletedAt: Timestamp?

    public let ledgerId: LedgerId

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension ScheduleInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_ScheduleInfo

    internal convenience init(fromProtobuf proto: Protobuf) throws {
        let adminKey = proto.hasAdminKey ? proto.adminKey : nil
        let expirationTime = proto.hasExpirationTime ? proto.expirationTime : nil

        let deletedAt: Timestamp.Protobuf?
        let executedAt: Timestamp.Protobuf?

        switch proto.data {
        case .none:
            deletedAt = nil
            executedAt = nil

        case .some(.deletionTime(let time)):
            deletedAt = time
            executedAt = nil
        case .some(.executionTime(let time)):
            deletedAt = nil
            executedAt = time
        }

        self.init(
            scheduleId: .fromProtobuf(proto.scheduleID),
            creatorAccountId: try .fromProtobuf(proto.creatorAccountID),
            payerAccountId: try .fromProtobuf(proto.payerAccountID),
            signatories: try .fromProtobuf(proto.signers.keys),
            adminKey: try .fromProtobuf(adminKey),
            scheduledTransactionId: try .fromProtobuf(proto.scheduledTransactionID),
            waitForExpiry: proto.waitForExpiry,
            scheduleMemo: proto.memo,
            expirationTime: .fromProtobuf(expirationTime),
            executedAt: .fromProtobuf(executedAt),
            deletedAt: .fromProtobuf(deletedAt),
            ledgerId: LedgerId.fromBytes(proto.ledgerID)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.scheduleID = scheduleId.toProtobuf()
            proto.creatorAccountID = creatorAccountId.toProtobuf()
            payerAccountId?.toProtobufInto(&proto.payerAccountID)
            proto.signers = KeyList(keys: signatories).toProtobuf()
            adminKey?.toProtobufInto(&proto.adminKey)
            proto.scheduledTransactionID = scheduledTransactionId.toProtobuf()
            proto.waitForExpiry = waitForExpiry
            proto.memo = scheduleMemo
            expirationTime?.toProtobufInto(&proto.expirationTime)
            executedAt?.toProtobufInto(&proto.executionTime)
            deletedAt?.toProtobufInto(&proto.deletionTime)
            proto.ledgerID = ledgerId.bytes
        }
    }
}
