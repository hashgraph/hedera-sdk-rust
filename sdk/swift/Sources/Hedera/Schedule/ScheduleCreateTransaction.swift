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

/// Create a new schedule entity (or simply, schedule) in the network's action queue.
///
/// Upon `SUCCESS`, the receipt contains the `ScheduleId` of the created schedule. A schedule
/// entity includes a `scheduled_transaction_body` to be executed.
///
/// When the schedule has collected enough signing keys to satisfy the schedule's signing
/// requirements, the schedule can be executed.
///
public final class ScheduleCreateTransaction: Transaction {
    /// Create a new `ScheduleCreateTransaction`.
    public init(
        expirationTime: Timestamp? = nil,
        isWaitForExpiry: Bool = false,
        payerAccountId: AccountId? = nil,
        scheduledTransaction: Transaction? = nil,
        adminKey: Key? = nil,
        scheduleMemo: String = ""
    ) {
        self.expirationTime = expirationTime
        self.isWaitForExpiry = isWaitForExpiry
        self.payerAccountId = payerAccountId
        self.scheduledTransactionInner = scheduledTransaction.map(AnySchedulableTransaction.init(upcasting:))
        self.adminKey = adminKey
        self.scheduleMemo = scheduleMemo

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_ScheduleCreateTransactionBody) throws {
        fatalError("Fixme: ScheduleCreateTransaction from bytes")
    }

    /// The timestamp for when the transaction should be evaluated for execution and then expire.
    public var expirationTime: Timestamp? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the timestamp for when the transaction should be evaluated for execution and then expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp?) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// If true, the transaction will be evaluated for execution at expiration_time instead
    /// of when all required signatures are received.
    public var isWaitForExpiry: Bool {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set if the transaction will be evaluated for execution at expiration_time instead
    /// of when all required signatures are received.
    @discardableResult
    public func isWaitForExpiry(_ isWaitForExpiry: Bool) -> Self {
        self.isWaitForExpiry = isWaitForExpiry

        return self
    }

    /// The id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time that it executes (if ever).
    public var payerAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time that it executes (if ever).
    @discardableResult
    public func payerAccountId(_ payerAccountId: AccountId?) -> Self {
        self.payerAccountId = payerAccountId

        return self
    }

    private var scheduledTransactionInner: AnySchedulableTransaction? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// The scheduled transaction.
    public var scheduledTransaction: Transaction? {
        get { scheduledTransactionInner?.transaction }
        set(value) {
            scheduledTransactionInner = value.map(AnySchedulableTransaction.init(upcasting:))
        }
    }

    /// Set the scheduled transaction.
    @discardableResult
    public func scheduledTransaction(_ scheduledTransaction: Transaction?) -> Self {
        self.scheduledTransaction = scheduledTransaction

        return self
    }

    /// The Hedera key which can be used to sign a ScheduleDelete and remove the schedule.
    public var adminKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the Hedera key which can be used to sign a ScheduleDelete and remove the schedule.
    @discardableResult
    public func adminKey(_ adminKey: Key?) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The memo for the schedule entity.
    public var scheduleMemo: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the memo for the schedule entity.
    @discardableResult
    public func scheduleMemo(_ scheduleMemo: String) -> Self {
        self.scheduleMemo = scheduleMemo

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try payerAccountId?.validateChecksums(on: ledgerId)
        try scheduledTransaction?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_ScheduleServiceAsyncClient(channel: channel).createSchedule(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .scheduleCreate(toProtobuf())
    }
}

extension ScheduleCreateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_ScheduleCreateTransactionBody

    internal func toProtobuf() -> Protobuf {
        let body = self.scheduledTransactionInner.map { scheduledTransaction in
            Proto_SchedulableTransactionBody.with { proto in
                proto.data = scheduledTransaction.toSchedulableTransactionData()
                proto.memo = scheduledTransaction.transaction.transactionMemo

                let transactionFee =
                    scheduledTransaction.transaction.maxTransactionFee
                    ?? scheduledTransaction.transaction.defaultMaxTransactionFee

                // FIXME: does not use the client to default the max transaction fee
                proto.transactionFee = UInt64(transactionFee.toTinybars())
            }
        }

        return
            .with { proto in
                if let body = body {
                    proto.scheduledTransactionBody = body
                }

                proto.memo = self.scheduleMemo

                if let adminKey = adminKey?.toProtobuf() {
                    proto.adminKey = adminKey
                }

                if let payerAccountId = payerAccountId?.toProtobuf() {
                    proto.payerAccountID = payerAccountId
                }

                if let expirationTime = expirationTime?.toProtobuf() {
                    proto.expirationTime = expirationTime
                }

                if isWaitForExpiry {
                    proto.waitForExpiry = true
                }
            }
    }
}
