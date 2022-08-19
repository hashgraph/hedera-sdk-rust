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

private let timeZoneUTC: TimeZone = TimeZone(abbreviation: "UTC")!

private let unixEpoch: Date = Calendar.current.date(from: DateComponents(timeZone: timeZoneUTC, year: 1970))!

extension Date {
    /// Construct a `Date` from the provided Unix timestamp (in nanoseconds).
    internal init(unixTimestampNanos: UInt64) {
        let seconds = Int(unixTimestampNanos / 1_000_000_000)
        let nanoseconds = Int(unixTimestampNanos % 1_000_000_000)

        let components = DateComponents(timeZone: timeZoneUTC, second: seconds, nanosecond: nanoseconds)

        self = Calendar.current.date(byAdding: components, to: unixEpoch)!
    }

    /// Get the Unix timestamp in nanoseconds.
    internal var unixTimestampNanos: UInt64 {
        let components = Calendar.current.dateComponents([.second, .nanosecond], from: unixEpoch, to: self)

        var timestamp = UInt64(components.second!) * 1_000_000_000
        timestamp += UInt64(components.nanosecond!)

        return timestamp
    }
}
