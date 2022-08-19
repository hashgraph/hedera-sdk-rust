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

/// Call a function of the given smart contract instance.
/// It will consume the entire given amount of gas.
///
/// This is performed locally on the particular node that the client is communicating with.
/// It cannot change the state of the contract instance (and so, cannot spend
/// anything from the instance's cryptocurrency account).
///
public final class ContractCallQuery: Query<ContractFunctionResult> {
    /// Create a new `ContractCallQuery`.
    public init(
        contractId: ContractId? = nil,
        gas: UInt64 = 0,
        functionParameters: Data? = nil,
        senderAccountId: AccountId? = nil
    ) {
        self.contractId = contractId
        self.gas = gas
        self.functionParameters = functionParameters
        self.senderAccountId = senderAccountId
    }

    /// The contract instance to call.
    public var contractId: ContractId?

    /// Set the contract instance to call.
    @discardableResult
    public func contractId(_ contractId: ContractId?) -> Self {
        self.contractId = contractId

        return self
    }

    /// The amount of gas to use for the call.
    public var gas: UInt64

    /// Set the amount of gas to use for the call.
    @discardableResult
    public func gas(_ gas: UInt64) -> Self {
        self.gas = gas

        return self
    }

    /// The function parameters as their raw bytes.
    public var functionParameters: Data?

    /// Set the function parameters as their raw bytes.
    @discardableResult
    public func functionParameters(_ functionParameters: Data?) -> Self {
        self.functionParameters = functionParameters

        return self
    }

    /// The sender for this transaction.
    public var senderAccountId: AccountId?

    /// Set the sender for this transaction.
    @discardableResult
    public func senderAccountId(_ senderAccountId: AccountId?) -> Self {
        self.senderAccountId = senderAccountId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case contractId
        case gas
        case functionParameters
        case senderAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(contractId, forKey: .contractId)
        try container.encode(gas, forKey: .gas)
        try container.encodeIfPresent(functionParameters?.base64EncodedString(), forKey: .functionParameters)
        try container.encodeIfPresent(senderAccountId, forKey: .senderAccountId)

        try super.encode(to: encoder)
    }
}
