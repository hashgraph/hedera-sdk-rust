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
}

extension ContractFunctionResult: Codable {
    private enum CodingKeys: CodingKey {
        case contractId
        case evmAddress
        case errorMessage
        case bloom
        case gasUsed
        case gas
        case logs
        case hbarAmount
        case contractFunctionParametersBytes
        case bytes
        case senderAccountId
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        contractId = try container.decode(.contractId)
        evmAddress = try container.decodeIfPresent(.evmAddress)
        errorMessage = try container.decodeIfPresent(.errorMessage)
        bloom = try container.decodeIfPresent(.bloom).map(Data.base64Encoded) ?? Data()
        gasUsed = try container.decodeIfPresent(.gasUsed) ?? 0
        gas = try container.decodeIfPresent(.gas) ?? 0
        logs = try container.decodeIfPresent(.logs) ?? []
        hbarAmount = try container.decode(.hbarAmount)
        contractFunctionParametersBytes =
            try container.decodeIfPresent(.contractFunctionParametersBytes)
            .map(Data.base64Encoded)
            ?? Data()
        bytes = try container.decodeIfPresent(.bytes).map(Data.base64Encoded) ?? Data()
        senderAccountId = try container.decodeIfPresent(.senderAccountId)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(contractId, forKey: .contractId)
        try container.encodeIfPresent(evmAddress, forKey: .evmAddress)
        try container.encodeIfPresent(errorMessage, forKey: .errorMessage)
        try container.encode(bloom.base64EncodedString(), forKey: .bloom)
        try container.encode(gasUsed, forKey: .gasUsed)
        try container.encode(gas, forKey: .gas)
        try container.encode(logs, forKey: .logs)
        try container.encode(hbarAmount, forKey: .hbarAmount)
        try container.encode(
            contractFunctionParametersBytes.base64EncodedString(), forKey: .contractFunctionParametersBytes)
        try container.encode(bytes.base64EncodedString(), forKey: .bytes)
        try container.encodeIfPresent(senderAccountId, forKey: .senderAccountId)
    }
}

// TODO: func getString(_ index: UInt) -> String
// TODO: func getStringArray(_ index: UInt) -> [String]
// TODO: func getBytes(_ index: UInt) -> Data
// TODO: func getBytes32(_ index: UInt) -> Data
// TODO: func getBool(_ index: UInt) -> Bool
// TODO: func getInt8(_ index: UInt) -> Int8
// TODO: func getInt32(_ index: UInt) -> Int32
// TODO: func getInt64(_ index: UInt) -> Int64
// TODO: func getInt256(_ index: UInt) -> BigInt
// TODO: func getUInt8(_ index: UInt) -> UInt8
// TODO: func getUInt32(_ index: UInt) -> UInt32
// TODO: func getUInt64(_ index: UInt) -> UInt64
// TODO: func getUInt256(_ index: UInt) -> BigUInt
// TODO: func getAddress(_ index: UInt) -> String
