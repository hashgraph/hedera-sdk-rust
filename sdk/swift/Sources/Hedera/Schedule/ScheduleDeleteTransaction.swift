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

/// Marks a schedule in the network's action queue as deleted.
public final class ScheduleDeleteTransaction: Transaction {
    /// Create a new `ScheduleDeleteTransaction`.
    public init(
        scheduleId: ScheduleId? = nil
    ) {
        self.scheduleId = scheduleId
        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_ScheduleDeleteTransactionBody) throws {
        scheduleId = data.hasScheduleID ? .fromProtobuf(data.scheduleID) : nil

        try super.init(protobuf: proto)
    }

    /// The schedule to delete.
    public var scheduleId: ScheduleId? {
        willSet {
            ensureNotFrozen(fieldName: "scheduleId")
        }
    }

    /// Sets the schedule to delete.
    @discardableResult
    public func scheduleId(_ scheduleId: ScheduleId) -> Self {
        self.scheduleId = scheduleId

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try scheduleId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_ScheduleServiceAsyncClient(channel: channel).deleteSchedule(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .scheduleDelete(toProtobuf())
    }
}

extension ScheduleDeleteTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_ScheduleDeleteTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            scheduleId?.toProtobufInto(&proto.scheduleID)
        }
    }
}

extension ScheduleDeleteTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .scheduleDelete(toProtobuf())
    }
}
