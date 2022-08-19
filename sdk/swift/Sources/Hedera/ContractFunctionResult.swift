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
public struct ContractFunctionResult: Codable {
    /// The smart contract instance whose function was called.
    public let contractId: ContractId

    /// The new contract's 20-byte EVM address.
    // TODO: public let evmAddress: ContractId?

    /// Message if there was an error during smart contract execution.
    public let errorMessage: String?

    /// Bloom filter for record.
    public let bloom: Data

    /// Units of gas used to execute contract.
    public let gasUsed: UInt64

    // TODO: public let logs: [ContractLogInfo]

    /// The amount of gas available for the call.
    public let gas: UInt64

    /// Number of HBAR sent (the function must be payable if this is nonzero).
    // TODO: Use Hbar type
    public let hbarAmount: UInt64

    /// The parameters passed into the contract call.
    public let contractFunctionParametersBytes: Data

    /// The raw bytes returned by the contract function.
    public let bytes: Data

    /// The account that is the "sender." If not present it is the accountId from the transactionId.
    public let senderAccountId: AccountId?

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        contractId = try container.decode(ContractId.self, forKey: .contractId)
        errorMessage = try container.decodeIfPresent(String.self, forKey: .errorMessage)
        bloom = Data(base64Encoded: try container.decode(String.self, forKey: .bloom))!
        gasUsed = try container.decode(UInt64.self, forKey: .gasUsed)
        gas = try container.decode(UInt64.self, forKey: .gas)
        hbarAmount = try container.decode(UInt64.self, forKey: .hbarAmount)
        contractFunctionParametersBytes = Data(base64Encoded: try container.decode(String.self, forKey: .contractFunctionParametersBytes))!
        bytes = Data(base64Encoded: try container.decode(String.self, forKey: .bytes))!
        senderAccountId = try container.decodeIfPresent(AccountId.self, forKey: .senderAccountId)
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
