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
import NumberKit

private let slotSize: UInt = 32

private func rangeFromSlot(slot: UInt, size: UInt) -> Range<Int> {
    let start = slot * slotSize + (slotSize - size)
    return Int(start)..<Int(start + size)
}

/// Result of invoking a contract via `ContractCallQuery`, `ContractExecuteTransaction`,
/// or `ContractCreateTransaction`.
public struct ContractFunctionResult {
    /// The smart contract instance whose function was called.
    public let contractId: ContractId

    /// The new contract's 20-byte EVM address.
    public let evmAddress: ContractId?

    /// Message if there was an error during smart contract execution.
    public let errorMessage: String?

    /// Bloom filter for record.
    public let bloom: Data

    /// Units of gas used to execute contract.
    public let gasUsed: UInt64

    /// The amount of gas available for the call.
    public let gas: UInt64

    /// Logs that this call and any called functions produced.
    public let logs: [ContractLogInfo]

    /// Number of HBAR sent (the function must be payable if this is nonzero).
    public let hbarAmount: Hbar

    /// The parameters passed into the contract call.
    public let contractFunctionParametersBytes: Data

    /// The raw bytes returned by the contract function.
    public let bytes: Data

    /// The account that is the "sender." If not present it is the accountId from the transactionId.
    public let senderAccountId: AccountId?

    internal init(
        contractId: ContractId,
        evmAddress: ContractId? = nil,
        errorMessage: String? = nil,
        bloom: Data,
        gasUsed: UInt64,
        gas: UInt64,
        hbarAmount: Hbar,
        contractFunctionParametersBytes: Data,
        bytes: Data,
        senderAccountId: AccountId? = nil,
        logs: [ContractLogInfo] = []
    ) {
        self.contractId = contractId
        self.evmAddress = evmAddress
        self.errorMessage = errorMessage
        self.bloom = bloom
        self.gasUsed = gasUsed
        self.gas = gas
        self.hbarAmount = hbarAmount
        self.contractFunctionParametersBytes = contractFunctionParametersBytes
        self.bytes = bytes
        self.senderAccountId = senderAccountId
        self.logs = logs
    }

    private func getFixedBytesAt(slot: UInt, size: UInt) -> Data? {
        self.bytes[slicing: rangeFromSlot(slot: slot, size: size)]
    }

    private func getAt<T: FixedWidthInteger>(slot: UInt) -> T? {
        getFixedBytesAt(slot: slot, size: UInt(MemoryLayout<T>.size)).flatMap(T.init(bigEndianBytes:))
    }

    public func asBytes() -> Data {
        bytes
    }

    public func getUInt8(_ index: UInt) -> UInt8? {
        getAt(slot: index)
    }

    public func getInt8(_ index: UInt) -> Int8? {
        getAt(slot: index)
    }

    public func getBool(_ index: UInt) -> Bool? {
        getUInt8(index).map { $0 != 0 }
    }

    public func getUInt32(_ index: UInt) -> UInt32? {
        getAt(slot: index)
    }

    private func getUIntAt(slot: UInt) -> UInt? {
        getUInt32(slot).map(UInt.init)
    }

    private func getUInt32At(offset: UInt) -> UInt32? {
        let size = Int(MemoryLayout<UInt32>.size)
        let offset = Int(offset + 28)

        return bytes[slicing: offset...]?[slicing: ..<size].map { UInt32(bigEndianBytes: $0)! }
    }

    private func getUIntAt(offset: UInt) -> UInt? {
        getUInt32At(offset: offset).map(UInt.init)
    }

    public func getInt32(_ index: UInt) -> Int32? {
        self.getAt(slot: index)
    }

    public func getUInt64(_ index: UInt) -> UInt64? {
        self.getAt(slot: index)
    }

    public func getInt64(_ index: UInt) -> Int64? {
        self.getAt(slot: index)
    }

    public func getBytes32(_ index: UInt) -> Data? {
        self.getFixedBytesAt(slot: index, size: 32).map(Data.init(_:))
    }

    public func getAddress(_ index: UInt) -> String? {
        self.getFixedBytesAt(slot: index, size: 20)?.hexStringEncoded()
    }

    public func getBytes(_ index: UInt) -> Data? {
        guard let offset = getUIntAt(slot: index) else { return nil }
        guard let len = getUIntAt(offset: offset) else { return nil }

        return bytes.safeSubdata(in: Int(offset + slotSize)..<Int(offset + len + slotSize))
    }

    public func getString(_ index: UInt) -> String? {
        getBytes(index).map { String(decoding: $0, as: UTF8.self) }
    }

    public func getStringArray(_ index: UInt) -> [String]? {
        guard let offset = getUIntAt(slot: index) else { return nil }
        guard let count = getUIntAt(offset: offset) else { return nil }

        var array: [String] = []

        for index in 0..<count {
            guard let strOffset = getUIntAt(offset: offset + slotSize + (index * slotSize)) else { return nil }
            guard let len = getUIntAt(offset: offset + strOffset + slotSize) else { return nil }
            let range = Int(offset + strOffset + slotSize * 2)..<Int(offset + strOffset + slotSize * 2 + len)

            guard let bytes = bytes[slicing: range] else { return nil }

            array.append(String(decoding: bytes, as: UTF8.self))
        }

        return array
    }

    public func getInt256(_ index: UInt) -> BigInt? {
        self.getBytes32(index).map { BigInt(signedBEBytes: $0) }
    }

    public func getUInt256(_ index: UInt) -> BigInt? {
        self.getBytes32(index).map { BigInt(unsignedBEBytes: $0) }
    }
}

extension ContractFunctionResult: TryFromProtobuf {
    internal typealias Protobuf = Proto_ContractFunctionResult

    internal init(protobuf proto: Protobuf) throws {
        let bytes: Data

        let errorMessage = !proto.errorMessage.isEmpty ? proto.errorMessage : nil

        if errorMessage != nil && proto.contractCallResult.starts(with: [0x08, 0xc3, 0x79, 0xa0]) {
            bytes = proto.contractCallResult.subdata(in: 4..<proto.contractCallResult.count)
        } else {
            bytes = proto.contractCallResult
        }

        let contractId = try ContractId.fromProtobuf(proto.contractID)

        self.init(
            contractId: contractId,
            evmAddress: proto.hasEvmAddress
                ? try ContractId.fromEvmAddressBytes(contractId.shard, contractId.realm, proto.evmAddress.value) : nil,
            errorMessage: errorMessage,
            bloom: proto.bloom,
            gasUsed: proto.gasUsed,
            gas: UInt64(proto.gas),
            hbarAmount: .fromTinybars(proto.amount),
            contractFunctionParametersBytes: proto.functionParameters,
            bytes: bytes,
            senderAccountId: proto.hasSenderID ? try .fromProtobuf(proto.senderID) : nil,
            logs: try .fromProtobuf(proto.logInfo)
        )
    }
}
