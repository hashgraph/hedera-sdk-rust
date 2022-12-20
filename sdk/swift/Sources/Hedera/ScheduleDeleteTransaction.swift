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

/// Marks a schedule in the network's action queue as deleted.
public final class ScheduleDeleteTransaction: Transaction {
    /// Create a new `ScheduleDeleteTransaction`.
    public init(
        scheduleId: ScheduleId? = nil
    ) {
        self.scheduleId = scheduleId
    }

    /// The schedule to delete.
    public var scheduleId: ScheduleId?

    /// Sets the schedule to delete.
    @discardableResult
    public func scheduleId(_ scheduleId: ScheduleId) -> Self {
        self.scheduleId = scheduleId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case scheduleId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(scheduleId, forKey: .scheduleId)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try scheduleId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
