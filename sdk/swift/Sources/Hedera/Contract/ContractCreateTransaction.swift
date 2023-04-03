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

/// Start a new smart contract instance.
public final class ContractCreateTransaction: Transaction {
    /// Create a new `ContractCreateTransaction`.
    public init(
        bytecode: Data? = nil,
        bytecodeFileId: FileId? = nil,
        adminKey: Key? = nil,
        gas: UInt64 = 0,
        initialBalance: Hbar = 0,
        autoRenewPeriod: Duration? = nil,
        constructorParameters: Data? = nil,
        contractMemo: String = "",
        maxAutomaticTokenAssociations: UInt32 = 0,
        autoRenewAccountId: AccountId? = nil,
        stakedAccountId: AccountId? = nil,
        stakedNodeId: UInt64? = nil,
        declineStakingReward: Bool = false
    ) {
        self.bytecode = bytecode
        self.bytecodeFileId = bytecodeFileId
        self.adminKey = adminKey
        self.gas = gas
        self.initialBalance = initialBalance
        self.autoRenewPeriod = autoRenewPeriod
        self.constructorParameters = constructorParameters
        self.contractMemo = contractMemo
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.autoRenewAccountId = autoRenewAccountId

        self.stakedAccountId = stakedAccountId

        if let stakedNodeId = stakedNodeId {
            // just ensure one "wins" for now.
            self.stakedAccountId = nil
            self.stakedNodeId = stakedNodeId
        }
        self.declineStakingReward = declineStakingReward

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_ContractCreateTransactionBody) throws {
        let stakedAccountId: AccountId?
        let stakedNodeId: UInt64?

        switch data.stakedID {
        case .stakedAccountID(let value):
            stakedAccountId = try .fromProtobuf(value)
            stakedNodeId = nil
        case .stakedNodeID(let value):
            stakedNodeId = UInt64(value)
            stakedAccountId = nil
        case nil:
            stakedAccountId = nil
            stakedNodeId = nil
        }

        let bytecode: Data?
        let bytecodeFileId: FileId?

        switch data.initcodeSource {
        case .initcode(let initcode):
            bytecode = initcode
            bytecodeFileId = nil
        case .fileID(let fileId):
            bytecode = nil
            bytecodeFileId = .fromProtobuf(fileId)
        case nil:
            bytecode = nil
            bytecodeFileId = nil
        }

        self.bytecode = bytecode
        self.bytecodeFileId = bytecodeFileId
        self.adminKey = try .fromProtobuf(data.adminKey)
        self.gas = UInt64(data.gas)
        self.initialBalance = .fromTinybars(data.initialBalance)
        self.autoRenewPeriod = .fromProtobuf(data.autoRenewPeriod)
        self.constructorParameters = !data.constructorParameters.isEmpty ? data.constructorParameters : nil
        self.contractMemo = data.memo
        self.maxAutomaticTokenAssociations = UInt32(data.maxAutomaticTokenAssociations)
        self.autoRenewAccountId = data.hasAutoRenewAccountID ? try .fromProtobuf(data.autoRenewAccountID) : nil
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
        self.declineStakingReward = data.declineReward

        try super.init(protobuf: proto)
    }

    /// The bytes of the smart contract.
    public var bytecode: Data? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the bytes of the smart contract.
    @discardableResult
    public func bytecode(_ bytecode: Data) -> Self {
        self.bytecode = bytecode

        return self
    }

    /// The file to use as the bytes for the smart contract.
    public var bytecodeFileId: FileId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the file to use as the bytes for the smart contract.
    @discardableResult
    public func bytecodeFileId(_ bytecodeFileId: FileId) -> Self {
        self.bytecodeFileId = bytecodeFileId

        return self
    }

    /// The admin key.
    public var adminKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the admin key.
    @discardableResult
    public func adminKey(_ adminKey: Key) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The gas limit to deploy the smart contract.
    public var gas: UInt64 {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the gas limit to deploy the smart contract.
    @discardableResult
    public func gas(_ gas: UInt64) -> Self {
        self.gas = gas

        return self
    }

    /// The initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    public var initialBalance: Hbar {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    @discardableResult
    public func initialBalance(_ initialBalance: Hbar) -> Self {
        self.initialBalance = initialBalance

        return self
    }

    /// The auto renew period for this smart contract.
    public var autoRenewPeriod: Duration? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the auto renew period for this smart contract.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The parameters to pass to the constructor.
    public var constructorParameters: Data? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the parameters to pass to the constructor.
    ///
    /// - Returns: `self`
    @discardableResult
    public func constructorParameters(_ parameters: Data) -> Self {
        self.constructorParameters = parameters

        return self
    }

    /// Sets the parameters to pass to the constructor.
    ///
    /// This is equivalent to calling `constructorParameters(parameters.toBytes())`
    ///
    /// - Returns: `self`
    @discardableResult
    public func constructorParameters(_ parameters: ContractFunctionParameters) -> Self {
        constructorParameters(parameters.toBytes())
    }

    /// The memo for the new smart contract.
    public var contractMemo: String {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the memo for the new smart contract.
    @discardableResult
    public func contractMemo(_ contractMemo: String) -> Self {
        self.contractMemo = contractMemo

        return self
    }

    /// The maximum number of tokens that this contract can be automatically associated with.
    public var maxAutomaticTokenAssociations: UInt32 {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// The account to be used at the contract's expiration time to extend the life of the contract.
    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account to be used at the contract's expiration time to extend the life of the contract.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The ID of the account to which this contract is staking.
    public var stakedAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the ID of the account to which this contract is staking.
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountId) -> Self {
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = nil

        return self
    }

    /// The ID of the node to which this contract is staking.
    public var stakedNodeId: UInt64? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the ID of the node to which this contract is staking.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64) -> Self {
        self.stakedNodeId = stakedNodeId
        self.stakedAccountId = nil

        return self
    }

    /// If true, the contract declines receiving a staking reward. The default value is false.
    public var declineStakingReward: Bool {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set to true, the contract declines receiving a staking reward. The default value is false.
    @discardableResult
    public func declineStakingReward(_ declineStakingReward: Bool) -> Self {
        self.declineStakingReward = declineStakingReward

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try bytecodeFileId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try stakedAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_SmartContractServiceAsyncClient(channel: channel).createContract(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .contractCreateInstance(toProtobuf())
    }
}

extension ContractCreateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_ContractCreateTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            switch (bytecode, bytecodeFileId) {
            // todo: just do whatever rust does
            case (.some, .some): fatalError("Cannot set both bytecode and bytecodeFileId")
            case (.some(let code), nil): proto.initcode = code
            case (nil, .some(let fileId)): proto.fileID = fileId.toProtobuf()
            default:
                break
            }

            adminKey?.toProtobufInto(&proto.adminKey)
            proto.gas = Int64(gas)
            proto.initialBalance = initialBalance.toTinybars()
            autoRenewPeriod?.toProtobufInto(&proto.autoRenewPeriod)
            autoRenewAccountId?.toProtobufInto(&proto.autoRenewAccountID)
            proto.constructorParameters = constructorParameters ?? Data()
            proto.memo = contractMemo
            proto.maxAutomaticTokenAssociations = Int32(maxAutomaticTokenAssociations)

            if let stakedAccountId = stakedAccountId?.toProtobuf() {
                proto.stakedAccountID = stakedAccountId
            }

            if let stakedNodeId = stakedNodeId {
                proto.stakedNodeID = Int64(stakedNodeId)
            }

            proto.declineReward = declineStakingReward
        }
    }
}

extension ContractCreateTransaction: ToSchedulableTransactionData {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .contractCreateInstance(toProtobuf())
    }
}
