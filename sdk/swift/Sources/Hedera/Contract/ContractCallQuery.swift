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
import GRPC
import HederaProtobufs

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

    /// The raw bytes of the function parameters.
    public var functionParameters: Data?

    /// Sets the function parameters as their raw bytes.
    @discardableResult
    public func functionParameters(_ functionParameters: Data?) -> Self {
        self.functionParameters = functionParameters

        return self
    }

    /// Sets the function name to call.
    ///
    /// The function will be called with no parameters.
    /// Use ``function(_:_:)`` to call a function with parameters.
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

    /// The sender for this transaction.
    public var senderAccountId: AccountId?

    /// Set the sender for this transaction.
    @discardableResult
    public func senderAccountId(_ senderAccountId: AccountId?) -> Self {
        self.senderAccountId = senderAccountId

        return self
    }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.contractCallLocal = .with { proto in
                proto.header = header
                proto.gas = Int64(gas)
                senderAccountId?.toProtobufInto(&proto.senderID)
                if let parameters = functionParameters {
                    proto.functionParameters = parameters
                }

                contractId?.toProtobufInto(&proto.contractID)
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_SmartContractServiceAsyncClient(channel: channel).contractCallLocalMethod(request)
    }

    internal override func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        guard case .contractCallLocal(let proto) = response else {
            throw HError.fromProtobuf("unexpected \(response) received, expected `contractCallLocal`")
        }

        return try .fromProtobuf(proto.functionResult)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try contractId?.validateChecksums(on: ledgerId)
        try senderAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
