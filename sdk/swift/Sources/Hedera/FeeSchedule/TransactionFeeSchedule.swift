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

/// The fees for a specific transaction or query based on the fee data.
///
/// See the [Hedera documentation].
///
/// [Hedera documentation]: https://docs.hedera.com/guides/docs/hedera-api/basic-types/transactionfeeschedule
public struct TransactionFeeSchedule: Codable {
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
        try Self.fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension TransactionFeeSchedule: ToFromJsonBytes {
    internal static var cFromBytes: FromJsonBytesFunc { hedera_fee_schedule_from_bytes }
    internal static var cToBytes: ToJsonBytesFunc { hedera_fee_schedule_to_bytes }
}
