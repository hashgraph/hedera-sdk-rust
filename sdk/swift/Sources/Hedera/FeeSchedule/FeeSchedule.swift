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

/// The fee schedules for hedera functionality and the time at which this fee schedule will expire.
///
/// See the [Hedera documentation].
///
/// [Hedera documentation]: https://docs.hedera.com/guides/docs/hedera-api/basic-types/feeschedule
public struct FeeSchedule {
    public init(transactionFeeSchedules: [TransactionFeeSchedule] = [], expirationTime: Timestamp) {
        self.transactionFeeSchedules = transactionFeeSchedules
        self.expirationTime = expirationTime
    }

    /// The fee schedules per specific piece of functionality.
    public var transactionFeeSchedules: [TransactionFeeSchedule]

    /// The time this fee schedule will expire at.
    public var expirationTime: Timestamp

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension FeeSchedule: TryProtobufCodable {
    internal typealias Protobuf = Proto_FeeSchedule

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            transactionFeeSchedules: try .fromProtobuf(proto.transactionFeeSchedule),
            expirationTime: .init(seconds: UInt64(proto.expiryTime.seconds), subSecondNanos: 0)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.transactionFeeSchedule = transactionFeeSchedules.toProtobuf()
            proto.expiryTime = .with { $0.seconds = Int64(expirationTime.seconds) }
        }
    }
}
