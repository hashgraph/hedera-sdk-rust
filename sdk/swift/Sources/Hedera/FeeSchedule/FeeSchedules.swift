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

/// Contains the current and next `FeeSchedule`s.
///
/// See the [Hedera documentation]
///
/// [Hedera documentation]: https://docs.hedera.com/guides/docs/hedera-api/basic-types/currentandnextfeeschedule
public struct FeeSchedules {
    public init(current: FeeSchedule? = nil, next: FeeSchedule? = nil) {
        self.current = current
        self.next = next
    }

    /// The current fee schedule.
    public var current: FeeSchedule?

    /// The next fee schedule.
    public var next: FeeSchedule?

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension FeeSchedules: TryProtobufCodable {
    internal typealias Protobuf = Proto_CurrentAndNextFeeSchedule

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            current: proto.hasCurrentFeeSchedule ? try .fromProtobuf(proto.currentFeeSchedule) : nil,
            next: proto.hasNextFeeSchedule ? try .fromProtobuf(proto.nextFeeSchedule) : nil
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            if let current = current?.toProtobuf() {
                proto.currentFeeSchedule = current
            }

            if let next = next?.toProtobuf() {
                proto.nextFeeSchedule = next
            }
        }
    }
}
