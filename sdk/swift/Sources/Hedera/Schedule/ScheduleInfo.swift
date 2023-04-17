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

/// Response from `ScheduleInfoQuery`.
public struct ScheduleInfo {
    /// The ID of the schedule for which information is requested.
    public let scheduleId: ScheduleId

    /// The account that created the scheduled transaction.
    public let creatorAccountId: AccountId

    /// The account paying for the execution of the scheduled transaction.
    public let payerAccountId: AccountId?

    /// The signatories that have provided signatures so far for the schedule
    /// transaction.
    public let signatories: KeyList

    /// The key which is able to delete the schedule transaction if set.
    public let adminKey: Key?

    /// The transaction id that will be used in the record of the scheduled transaction (if
    /// it executes).
    public let scheduledTransactionId: TransactionId

    private let scheduledTransaction: Proto_SchedulableTransactionBody

    /// When set to true, the transaction will be evaluated for execution at `expiration_time`
    /// instead of when all required signatures are received.
    public let waitForExpiry: Bool

    /// Publicly visible information about the Schedule entity.
    public let memo: String

    /// The date and time the schedule transaction will expire
    public let expirationTime: Timestamp?

    /// The time the schedule transaction was executed.
    public let executedAt: Timestamp?

    /// The time the schedule transaction was deleted.
    public let deletedAt: Timestamp?

    public let ledgerId: LedgerId

    /// Returns the transaction associated with this schedule
    ///
    /// This function may or may not be O(1)
    ///
    /// This function may or may not throw
    ///
    /// This function name is not final.
    public func getScheduledTransaction() throws -> Transaction {
        let transactionBody = Proto_TransactionBody.with { proto in
            switch scheduledTransaction.data {
            case .contractCall(let data): proto.data = .contractCall(data)
            case .contractCreateInstance(let data): proto.data = .contractCreateInstance(data)
            case .contractUpdateInstance(let data): proto.data = .contractUpdateInstance(data)
            case .contractDeleteInstance(let data): proto.data = .contractDeleteInstance(data)
            case .cryptoApproveAllowance(let data): proto.data = .cryptoApproveAllowance(data)
            case .cryptoDeleteAllowance(let data): proto.data = .cryptoDeleteAllowance(data)
            case .cryptoCreateAccount(let data): proto.data = .cryptoCreateAccount(data)
            case .cryptoDelete(let data): proto.data = .cryptoDelete(data)
            case .cryptoTransfer(let data): proto.data = .cryptoTransfer(data)
            case .cryptoUpdateAccount(let data): proto.data = .cryptoUpdateAccount(data)
            case .fileAppend(let data): proto.data = .fileAppend(data)
            case .fileCreate(let data): proto.data = .fileCreate(data)
            case .fileDelete(let data): proto.data = .fileDelete(data)
            case .fileUpdate(let data): proto.data = .fileUpdate(data)
            case .systemDelete(let data): proto.data = .systemDelete(data)
            case .systemUndelete(let data): proto.data = .systemUndelete(data)
            case .freeze(let data): proto.data = .freeze(data)
            case .consensusCreateTopic(let data): proto.data = .consensusCreateTopic(data)
            case .consensusUpdateTopic(let data): proto.data = .consensusUpdateTopic(data)
            case .consensusDeleteTopic(let data): proto.data = .consensusDeleteTopic(data)
            case .consensusSubmitMessage(let data): proto.data = .consensusSubmitMessage(data)
            case .tokenCreation(let data): proto.data = .tokenCreation(data)
            case .tokenFreeze(let data): proto.data = .tokenFreeze(data)
            case .tokenUnfreeze(let data): proto.data = .tokenUnfreeze(data)
            case .tokenGrantKyc(let data): proto.data = .tokenGrantKyc(data)
            case .tokenRevokeKyc(let data): proto.data = .tokenRevokeKyc(data)
            case .tokenDeletion(let data): proto.data = .tokenDeletion(data)
            case .tokenUpdate(let data): proto.data = .tokenUpdate(data)
            case .tokenMint(let data): proto.data = .tokenMint(data)
            case .tokenBurn(let data): proto.data = .tokenBurn(data)
            case .tokenWipe(let data): proto.data = .tokenWipe(data)
            case .tokenAssociate(let data): proto.data = .tokenAssociate(data)
            case .tokenDissociate(let data): proto.data = .tokenDissociate(data)
            case .tokenFeeScheduleUpdate(let data): proto.data = .tokenFeeScheduleUpdate(data)
            case .tokenPause(let data): proto.data = .tokenPause(data)
            case .tokenUnpause(let data): proto.data = .tokenUnpause(data)
            case .scheduleDelete(let data): proto.data = .scheduleDelete(data)
            case .utilPrng(let data): proto.data = .utilPrng(data)
            case nil: break
            }

            proto.memo = scheduledTransaction.memo
            proto.transactionFee = scheduledTransaction.transactionFee
        }

        return try AnyTransaction.fromProtobuf(transactionBody, [transactionBody.data!]).transaction
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension ScheduleInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_ScheduleInfo

    internal init(protobuf proto: Protobuf) throws {
        let deletedAt: Timestamp?
        let executedAt: Timestamp?

        switch proto.data {
        case .deletionTime(let data):
            deletedAt = .fromProtobuf(data)
            executedAt = nil
        case .executionTime(let data):
            deletedAt = nil
            executedAt = .fromProtobuf(data)
        case nil:
            executedAt = nil
            deletedAt = nil
        }

        self.init(
            scheduleId: .fromProtobuf(proto.scheduleID),
            creatorAccountId: try .fromProtobuf(proto.creatorAccountID),
            payerAccountId: proto.hasPayerAccountID ? try .fromProtobuf(proto.payerAccountID) : nil,
            signatories: try .fromProtobuf(proto.signers),
            adminKey: proto.hasAdminKey ? try .fromProtobuf(proto.adminKey) : nil,
            scheduledTransactionId: try .fromProtobuf(proto.scheduledTransactionID),
            scheduledTransaction: proto.scheduledTransactionBody,
            waitForExpiry: proto.waitForExpiry,
            memo: proto.memo,
            expirationTime: proto.hasExpirationTime ? .fromProtobuf(proto.expirationTime) : nil,
            executedAt: executedAt,
            deletedAt: deletedAt,
            ledgerId: .fromBytes(proto.ledgerID)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.scheduleID = scheduleId.toProtobuf()
            if let expirationTime = expirationTime?.toProtobuf() {
                proto.expirationTime = expirationTime
            }

            proto.memo = memo
            if let adminKey = adminKey?.toProtobuf() {
                proto.adminKey = adminKey
            }

            if !signatories.isEmpty {
                proto.signers = signatories.toProtobuf()
            }

            proto.creatorAccountID = creatorAccountId.toProtobuf()

            if let payerAccountId = payerAccountId?.toProtobuf() {
                proto.payerAccountID = payerAccountId
            }

            proto.scheduledTransactionID = scheduledTransactionId.toProtobuf()

            proto.ledgerID = self.ledgerId.bytes
            proto.waitForExpiry = self.waitForExpiry

            if let executedAt = self.executedAt {
                proto.executionTime = executedAt.toProtobuf()
            }

            if let deletedAt = self.deletedAt {
                proto.deletionTime = deletedAt.toProtobuf()
            }

            proto.scheduledTransactionBody = scheduledTransaction
        }
    }
}
