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

/// The different components used for fee calculation.
public struct FeeComponents {
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
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension FeeComponents: ProtobufCodable {
    internal typealias Protobuf = Proto_FeeComponents

    internal init(protobuf proto: Proto_FeeComponents) {
        self.init(
            min: UInt64(proto.min),
            max: UInt64(proto.max),
            constant: UInt64(proto.constant),
            bandwidthByte: UInt64(proto.bpt),
            verification: UInt64(proto.vpt),
            storageByteHour: UInt64(proto.sbh),
            ramByteHour: UInt64(proto.rbh),
            contractTransactionGas: UInt64(proto.gas),
            transferVolumeHbar: UInt64(proto.tv),
            responseMemoryByte: UInt64(proto.bpr),
            responseDiskByte: UInt64(proto.sbpr)
        )
    }

    internal func toProtobuf() -> Proto_FeeComponents {
        .with { proto in
            proto.min = Int64(min)
            proto.max = Int64(max)
            proto.constant = Int64(constant)
            proto.bpt = Int64(bandwidthByte)
            proto.vpt = Int64(verification)
            proto.sbh = Int64(storageByteHour)
            proto.rbh = Int64(ramByteHour)
            proto.gas = Int64(contractTransactionGas)
            proto.tv = Int64(transferVolumeHbar)
            proto.bpr = Int64(responseMemoryByte)
            proto.sbpr = Int64(responseDiskByte)
        }
    }
}
