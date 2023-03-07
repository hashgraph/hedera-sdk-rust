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

/// The fee schedules for hedera functionality and the time at which this fee schedule will expire.
///
/// See the [Hedera documentation].
///
/// [Hedera documentation]: https://docs.hedera.com/guides/docs/hedera-api/basic-types/feeschedule
public struct FeeSchedule: Codable {
    public init(transactionFeeSchedules: TransactionFeeSchedule? = nil, expirationTime: Timestamp) {
        self.transactionFeeSchedules = transactionFeeSchedules
        self.expirationTime = expirationTime
    }

    /// The fee schedules per specific piece of functionality.
    public var transactionFeeSchedules: TransactionFeeSchedule?

    /// The time this fee schedule will expire at.
    public var expirationTime: Timestamp

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self.fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension FeeSchedule: ToFromJsonBytes {
    internal static var cFromBytes: FromJsonBytesFunc { hedera_fee_schedule_from_bytes }
    internal static var cToBytes: ToJsonBytesFunc { hedera_fee_schedule_to_bytes }
}
