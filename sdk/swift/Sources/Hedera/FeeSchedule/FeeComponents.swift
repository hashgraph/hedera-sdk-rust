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

/// The different components used for fee calculation.
public struct FeeComponents: Codable {
    public init(
        min: UInt64,
        max: UInt64,
        constant: UInt64,
        bandwidthByte: UInt64,
        verification: UInt64,
        storageByteHour: UInt64,
        ramByteHour: UInt64,
        contractTransactionGas: UInt64,
        transferVolumeHbar: UInt64,
        responseMemoryByte: UInt64,
        responseDiskByte: UInt64
    ) {
        self.min = min
        self.max = max
        self.constant = constant
        self.bandwidthByte = bandwidthByte
        self.verification = verification
        self.storageByteHour = storageByteHour
        self.ramByteHour = ramByteHour
        self.contractTransactionGas = contractTransactionGas
        self.transferVolumeHbar = transferVolumeHbar
        self.responseMemoryByte = responseMemoryByte
        self.responseDiskByte = responseDiskByte
    }

    /// The minimum fee that needs to be paid.
    public var min: UInt64

    /// The maximum fee that can be submitted.
    public var max: UInt64

    /// A constant determined by the business to calculate the fee.
    public var constant: UInt64

    /// The cost of each byte in a transaction.
    public var bandwidthByte: UInt64

    /// The cost of each signature in a transaction.
    public var verification: UInt64

    /// Cost of storage measured in byte-hours.
    public var storageByteHour: UInt64

    /// Cost of memory measured in byte-hours.
    public var ramByteHour: UInt64

    /// Price of gas.
    public var contractTransactionGas: UInt64

    /// Cost per hbar transfered.
    ///
    /// fee = `floor(transferValue in tinybars / (transferVolumeHbar / 1000))`
    public var transferVolumeHbar: UInt64

    /// The price per byte of bandwidth spent for data retrieved from memory for a response.
    public var responseMemoryByte: UInt64

    /// The price per byte of bandwidth spent for data retrieved from disk for a response.
    public var responseDiskByte: UInt64

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self.fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension FeeComponents: ToFromJsonBytes {
    internal static var cFromBytes: FromJsonBytesFunc { hedera_fee_components_from_bytes }
    internal static var cToBytes: ToJsonBytesFunc { hedera_fee_components_to_bytes }
}
