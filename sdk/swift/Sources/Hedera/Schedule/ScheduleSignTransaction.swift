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

/// Adds zero or more signing keys to a schedule.
public final class ScheduleSignTransaction: Transaction {
    /// Create a new `ScheduleSignTransaction`.
    public init(
        scheduleId: ScheduleId? = nil
    ) {
        self.scheduleId = scheduleId
        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_ScheduleSignTransactionBody) throws {
        scheduleId = data.hasScheduleID ? .fromProtobuf(data.scheduleID) : nil

        try super.init(protobuf: proto)
    }

    /// The schedule to add signing keys to.
    public var scheduleId: ScheduleId? {
        willSet {
            ensureNotFrozen(fieldName: "scheduleId")
        }
    }

    /// Set the schedule to add signing keys to.
    @discardableResult
    public func scheduleId(_ scheduleId: ScheduleId) -> Self {
        self.scheduleId = scheduleId

        return self
    }

    @discardableResult
    public func clearScheduleId() -> Self {
        scheduleId = nil

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try scheduleId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_ScheduleServiceAsyncClient(channel: channel).signSchedule(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .scheduleSign(toProtobuf())
    }
}

extension ScheduleSignTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_ScheduleSignTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            scheduleId?.toProtobufInto(&proto.scheduleID)
        }
    }
}
