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

/// Call a function of the given smart contract instance, giving it
/// parameters as its inputs.
///
/// It can use the given amount of gas, and any unspent gas will
/// be refunded to the paying account.
///
/// If this function stores information, it is charged gas to store it.
/// There is a fee in hbars to maintain that storage until the expiration time,
/// and that fee is added as part of the transaction fee.
///
public final class ContractExecuteTransaction: Transaction {
    /// Create a new `ContractExecuteTransaction`.
    public init(
        contractId: ContractId? = nil,
        gas: UInt64 = 0,
        payableAmount: Hbar = 0,
        functionParameters: Data? = nil
    ) {
        self.contractId = contractId
        self.gas = gas
        self.payableAmount = payableAmount
        self.functionParameters = functionParameters
    }

    /// The contract instance to call.
    public var contractId: ContractId? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Set the contract instance to call.
    @discardableResult
    public func contractId(_ contractId: ContractId?) -> Self {
        self.contractId = contractId

        return self
    }

    /// The maximum amount of gas to use for the call.
    public var gas: UInt64 {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Set the maximum amount of gas to use for the call.
    @discardableResult
    public func gas(_ gas: UInt64) -> Self {
        self.gas = gas

        return self
    }

    /// The number of hbars sent with this function call.
    public var payableAmount: Hbar {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Set the number of hbars sent with this function call.
    @discardableResult
    public func payableAmount(_ payableAmount: Hbar) -> Self {
        self.payableAmount = payableAmount

        return self
    }

    /// The raw bytes of the function parameters.
    public var functionParameters: Data? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the function parameters as their raw bytes.
    @discardableResult
    public func functionParameters(_ functionParameters: Data?) -> Self {
        self.functionParameters = functionParameters

        return self
    }

    /// Sets the function name to call.
    ///
    /// The function will be called with no parameters.
    /// Use ``function(_:_)`` to call a function with parameters.
    ///
    /// - Parameter name: The name of the function to call.
    ///
    /// - Returns: `self`
    @discardableResult
    public func function(_ name: String) -> Self {
        function(name, ContractFunctionParameters())
    }

    /// Sets the function to call, and the parameters to pass to the function.
    ///
    /// This is equivalent to calling `functionParameters(parameters.toBytes(name))`
    ///
    /// - Parameters:
    ///   - name: The name of the function to call.
    ///   - parameters: The parameters to pass to the function.
    ///
    /// - Returns: `self`
    @discardableResult
    public func function(_ name: String, _ parameters: ContractFunctionParameters) -> Self {
        functionParameters(parameters.toBytes(name))
    }

    private enum CodingKeys: String, CodingKey {
        case contractId
        case gas
        case payableAmount
        case functionParameters
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(contractId, forKey: .contractId)
        try container.encode(gas, forKey: .gas)
        try container.encode(payableAmount, forKey: .payableAmount)
        try container.encodeIfPresent(functionParameters?.base64EncodedString(), forKey: .functionParameters)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try contractId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

}
