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

/// The fees for a specific transaction or query based on the fee data.
///
/// See the [Hedera documentation].
///
/// [Hedera documentation]: https://docs.hedera.com/guides/docs/hedera-api/basic-types/transactionfeeschedule
public struct TransactionFeeSchedule {
    public init(requestType: RequestType, feeData: FeeData? = nil, fees: [FeeData]) {
        self.requestType = requestType
        self.feeData = feeData
        self.fees = fees
    }

    /// The request type that this fee schedule applies to.
    public var requestType: RequestType

    /// Resource price coefficients.
    public var feeData: FeeData?

    /// Resource price coefficients.
    ///
    /// Supports subtype definition.
    public var fees: [FeeData]

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension TransactionFeeSchedule: TryProtobufCodable {
    internal typealias Protobuf = Proto_TransactionFeeSchedule

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            requestType: try .fromProtobuf(proto.hederaFunctionality),
            feeData: proto.hasFeeData ? try .fromProtobuf(proto.feeData) : nil,
            fees: try .fromProtobuf(proto.fees)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.hederaFunctionality = requestType.toProtobuf()
            if let feeData = feeData?.toProtobuf() {
                proto.feeData = feeData
            }

            proto.fees = fees.toProtobuf()
        }
    }
}
