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

/// Get all the information about a schedule.
public final class ScheduleInfoQuery: Query<ScheduleInfo> {
    /// Create a new `ScheduleInfoQuery`.
    public init(
        scheduleId: ScheduleId? = nil
    ) {
        self.scheduleId = scheduleId
    }

    /// The schedule ID for which information is requested.
    public var scheduleId: ScheduleId?

    /// Sets the schedule ID for which information is requested.
    @discardableResult
    public func scheduleId(_ scheduleId: ScheduleId) -> Self {
        self.scheduleId = scheduleId

        return self
    }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.scheduleGetInfo = .with { proto in
                proto.header = header
                scheduleId?.toProtobufInto(&proto.scheduleID)
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_ScheduleServiceAsyncClient(channel: channel).getScheduleInfo(request)
    }

    internal override func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        guard case .scheduleGetInfo(let proto) = response else {
            throw HError.fromProtobuf("unexpected \(response) received, expected `scheduleGetInfo`")
        }

        return try .fromProtobuf(proto.scheduleInfo)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try scheduleId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
