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
        self.stakedId = stakedAccountId.map(StakedId.accountId) ?? stakedNodeId.map(StakedId.nodeId)
        self.declineStakingReward = declineStakingReward

        super.init()
    }

    internal init(
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
        stakedId: StakedId? = nil,
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
        self.stakedId = stakedId
        self.declineStakingReward = declineStakingReward

        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        bytecode = try container.decodeIfPresent(.bytecode).map(Data.base64Encoded)
        bytecodeFileId = try container.decodeIfPresent(.bytecodeFileId)
        adminKey = try container.decodeIfPresent(.adminKey)
        gas = try container.decodeIfPresent(.gas) ?? 0
        initialBalance = try container.decodeIfPresent(.initialBalance) ?? 0
        autoRenewPeriod = try container.decodeIfPresent(.autoRenewPeriod)
        constructorParameters = try container.decodeIfPresent(.constructorParameters).map(Data.base64Encoded)
        contractMemo = try container.decodeIfPresent(.contractMemo) ?? ""
        maxAutomaticTokenAssociations = try container.decodeIfPresent(.maxAutomaticTokenAssociations) ?? 0
        autoRenewAccountId = try container.decodeIfPresent(.autoRenewAccountId)

        stakedId = try StakedId.flatDecode(from: decoder)
        declineStakingReward = try container.decodeIfPresent(.declineStakingReward) ?? false

        try super.init(from: decoder)
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

    /// The ID that to which this contract is / will be staking.
    private var stakedId: StakedId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// The ID of the account to which this contract is staking.
    public var stakedAccountId: AccountId? {
        get { stakedId?.accountId }
        set(value) { stakedId = value.map(StakedId.accountId) }
    }

    /// Sets the ID of the account to which this contract is staking.
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountId) -> Self {
        stakedId = .accountId(stakedAccountId)

        return self
    }

    /// The ID of the node to which this contract is staking.
    public var stakedNodeId: UInt64? {
        get { stakedId?.nodeId }
        set(value) { stakedId = value.map(StakedId.nodeId) }
    }

    /// Sets the ID of the node to which this contract is staking.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64) -> Self {
        stakedId = .nodeId(stakedNodeId)

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

    private enum CodingKeys: String, CodingKey {
        case bytecode
        case bytecodeFileId
        case adminKey
        case gas
        case initialBalance
        case autoRenewPeriod
        case constructorParameters
        case contractMemo
        case maxAutomaticTokenAssociations
        case autoRenewAccountId
        case declineStakingReward
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(bytecode?.base64EncodedString(), forKey: .bytecode)
        try container.encodeIfPresent(bytecodeFileId, forKey: .bytecodeFileId)
        try container.encodeIfPresent(adminKey, forKey: .adminKey)
        try container.encode(gas, forKey: .gas)
        try container.encode(initialBalance, forKey: .initialBalance)
        try container.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try container.encodeIfPresent(constructorParameters?.base64EncodedString(), forKey: .constructorParameters)
        try container.encode(contractMemo, forKey: .contractMemo)
        try container.encode(maxAutomaticTokenAssociations, forKey: .maxAutomaticTokenAssociations)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encode(declineStakingReward, forKey: .declineStakingReward)

        try stakedId?.encode(to: encoder)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try bytecodeFileId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try stakedAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal static func fromProtobufData(_ proto: Proto_ContractCreateTransactionBody) throws -> Self {
        let bytecode: Data?
        let bytecodeFileId: FileId?

        switch proto.initcodeSource {
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

        return Self(
            bytecode: bytecode,
            bytecodeFileId: bytecodeFileId,
            adminKey: try .fromProtobuf(proto.adminKey),
            gas: UInt64(proto.gas),
            initialBalance: .fromTinybars(proto.initialBalance),
            autoRenewPeriod: .fromProtobuf(proto.autoRenewPeriod),
            constructorParameters: !proto.constructorParameters.isEmpty ? proto.constructorParameters : nil,
            contractMemo: proto.memo,
            maxAutomaticTokenAssociations: UInt32(proto.maxAutomaticTokenAssociations),
            autoRenewAccountId: proto.hasAutoRenewAccountID ? try .fromProtobuf(proto.autoRenewAccountID) : nil,
            stakedId: try proto.stakedID.map(StakedId.fromProtobuf),
            declineStakingReward: proto.declineReward
        )
    }

    internal override func execute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {

        try await Proto_SmartContractServiceAsyncClient(channel: channel).createContract(request)
    }

    internal override func toTransactionDataProtobuf(_ nodeAccountId: AccountId, _ transactionId: TransactionId)
        -> Proto_TransactionBody.OneOf_Data
    {
        .contractCreateInstance(
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

                proto.stakedID = stakedId?.toProtobuf()

                proto.declineReward = declineStakingReward
            }
        )
    }
}

extension StakedId {
    fileprivate typealias Protobuf = Proto_ContractCreateTransactionBody.OneOf_StakedID
    fileprivate static func fromProtobuf(_ proto: Protobuf) throws -> Self {
        switch proto {
        case .stakedAccountID(let id):
            return .accountId(try .fromProtobuf(id))
        case .stakedNodeID(let id):
            return .nodeId(UInt64(id))
        }
    }

    fileprivate func toProtobuf() -> Protobuf {
        switch self {
        case .accountId(let id):
            return .stakedAccountID(id.toProtobuf())
        case .nodeId(let id):
            return .stakedNodeID(Int64(id))

        }
    }
}
