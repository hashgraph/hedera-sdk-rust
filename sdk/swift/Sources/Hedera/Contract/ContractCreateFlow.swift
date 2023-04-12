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

// swiftlint:disable file_length

import Foundation

// swiftlint:disable type_body_length
/// Create a new smart contract
///
/// The operation of this flow is as follows:
/// 1. Create a file for the contract's bytecode (via a ``FileCreateTransaction`` and zero or more ``FileAppendTransaction``s)
/// 2. Execute a ``ContractCreateTransaction`` using the provided information and the newly created file.
/// 3. Delete the file created in step 1.
///
///
/// >Note: This executes 3 or more transactions,
/// even if the contract size is small enough to fit directly into a ``ContractCreateTransaction``.
public final class ContractCreateFlow {
    private enum StakedId {
        case accountId(AccountId)
        case nodeId(UInt64)
    }

    public init() {
        self.bytecode = Data()
        self.nodeAccountIds = nil
        self.maxFileAppendChunks = nil
        self.contractCreateData = .init()
    }

    private struct ContractCreateTransactionData {
        internal init(
            constructorParameters: Data = Data(),
            gas: UInt64 = 0,
            initialBalance: Hbar = .zero,
            maxAutomaticTokenAssociations: UInt32 = 0,
            declineStakingReward: Bool = false,
            adminKey: Key? = nil,
            autoRenewAccountId: AccountId? = nil,
            autoRenewPeriod: Duration? = nil,
            contractMemo: String? = nil,
            stakedId: StakedId? = nil,
            freezeWithClient: Client? = nil,
            signer: Signer? = nil
        ) {
            self.constructorParameters = constructorParameters
            self.gas = gas
            self.initialBalance = initialBalance
            self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
            self.declineStakingReward = declineStakingReward
            self.adminKey = adminKey
            self.autoRenewAccountId = autoRenewAccountId
            self.autoRenewPeriod = autoRenewPeriod
            self.contractMemo = contractMemo
            self.stakedId = stakedId
            self.freezeWithClient = freezeWithClient
            self.signer = signer
        }

        fileprivate var constructorParameters: Data
        fileprivate var gas: UInt64
        fileprivate var initialBalance: Hbar
        fileprivate var maxAutomaticTokenAssociations: UInt32
        fileprivate var declineStakingReward: Bool
        fileprivate var adminKey: Key?
        // fileprivate var proxyAccountId: AccountId?
        fileprivate var autoRenewAccountId: AccountId?
        fileprivate var autoRenewPeriod: Duration?
        fileprivate var contractMemo: String?
        fileprivate var stakedId: StakedId?
        fileprivate var freezeWithClient: Client?
        fileprivate var signer: Signer?
    }

    private static let maxFileCreateDataBytes: Int = 2048

    private var contractCreateData: ContractCreateTransactionData

    /// The bytes of the smart contract.
    public var bytecode: Data

    /// Sets the raw bytes of the smart contract.
    ///
    /// - Returns: `self`
    @discardableResult
    public func bytecode(_ bytecode: Data) -> Self {
        self.bytecode = bytecode

        return self
    }

    /// Sets the bytecode of the smart contract in hex.
    ///
    /// - Returns: `self`
    @discardableResult
    public func bytecode<S: StringProtocol>(_ bytecode: S) -> Self {
        self.bytecode = Data(hexEncoded: bytecode)!

        return self
    }

    /// The account IDs of the nodes the transactions may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client.
    public var nodeAccountIds: [AccountId]?

    /// Sets the account IDs of the nodes the transactions may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client.
    ///
    /// - Returns: `self`
    @discardableResult
    public func nodeAccountIds(_ nodeAccountIds: [AccountId]) -> Self {
        self.nodeAccountIds = nodeAccountIds

        return self
    }

    // fixme: this exists because the name is better,
    // but is it worth the extra property required.
    private var maxFileAppendChunks: Int?

    /// The maximum number of chunks the FileAppendTransaction can be split into.
    ///
    /// If null, the default value for a ``FileAppendTransaction`` will be used.
    public var maxChunks: Int? {
        get { maxFileAppendChunks }
        set(value) { maxFileAppendChunks = value }
    }

    /// Sets the maximum number of chunks the FileAppendTransaction can be split into.
    ///
    /// - Returns: `self`
    @discardableResult
    public func maxChunks(_ maxChunks: Int) -> Self {
        self.maxChunks = maxChunks

        return self
    }

    /// The parameters to pass to the constructor.
    public var constructorParameters: Data {
        get { contractCreateData.constructorParameters }
        set(value) { contractCreateData.constructorParameters = value }
    }

    /// Sets the parameters to pass to the constructor.
    ///
    /// - Returns: `self`
    @discardableResult
    public func constructorParameters(_ constructorParameters: Data) -> Self {
        self.constructorParameters = constructorParameters

        return self
    }

    /// Sets the parameters to pass to the constructor.
    ///
    /// This is equivalent to calling `constructorParameters(parameters.toBytes())`
    ///
    /// - Returns: `self`
    @discardableResult
    public func constructorParameters(_ constructorParameters: ContractFunctionParameters) -> Self {
        self.constructorParameters = constructorParameters.toBytes()

        return self
    }

    /// The gas limit to deploy the smart contract.
    public var gas: UInt64 {
        get { contractCreateData.gas }
        set(value) { contractCreateData.gas = value }
    }

    /// Sets the gas limit to deploy the smart contract.
    ///
    /// - Returns: `self`
    @discardableResult
    public func gas(_ gas: UInt64) -> Self {
        self.gas = gas

        return self
    }

    /// The initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    public var initialBalance: Hbar {
        get { contractCreateData.initialBalance }
        set(value) { contractCreateData.initialBalance = value }
    }

    /// Sets the initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    ///
    /// - Returns: `self`
    @discardableResult
    public func initialBalance(_ initialBalance: Hbar) -> Self {
        self.initialBalance = initialBalance

        return self
    }

    /// The maximum number of tokens that the contract can be automatically associated with.
    public var maxAutomaticTokenAssociations: UInt32 {
        get { contractCreateData.maxAutomaticTokenAssociations }
        set(value) { contractCreateData.maxAutomaticTokenAssociations = value }
    }

    /// Sets the maximum number of tokens that the contract can be automatically associated with.
    ///
    /// - Returns: `self`
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// If `true`, the contract will decline receiving a staking reward.
    ///
    /// The default value is false.
    public var declineStakingReward: Bool {
        get { contractCreateData.declineStakingReward }
        set(value) { contractCreateData.declineStakingReward = value }
    }

    /// If set to `true`, the contract will decline receiving a staking reward.
    ///
    /// - Returns: `self`
    @discardableResult
    public func declineStakingReward(_ declineStakingReward: Bool) -> Self {
        self.declineStakingReward = declineStakingReward

        return self
    }

    /// The admin key for the new contract.
    public var adminKey: Key? {
        get { contractCreateData.adminKey }
        set(value) { contractCreateData.adminKey = value }
    }

    /// Sets the admin key for the new contract.
    ///
    /// - Returns: `self`
    @discardableResult
    public func adminKey(_ adminKey: Key) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The account to be used at the contract's expiration time to extend the life of the contract.
    public var autoRenewAccountId: AccountId? {
        get { contractCreateData.autoRenewAccountId }
        set(value) { contractCreateData.autoRenewAccountId = value }
    }

    /// Sets the account to be used at the contract's expiration time to extend the life of the contract.
    ///
    /// - Returns: `self`
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The auto renew period for the smart contract.
    public var autoRenewPeriod: Duration? {
        get { contractCreateData.autoRenewPeriod }
        set(value) { contractCreateData.autoRenewPeriod = value }
    }

    /// Sets the auto renew period for the smart contract.
    ///
    /// - Returns: `self`
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The memo for the new smart contract.
    public var contractMemo: String? {
        get { contractCreateData.contractMemo }
        set(value) { contractCreateData.contractMemo = value }
    }

    /// Sets the memo for the new smart contract.
    ///
    /// - Returns: `self`
    @discardableResult
    public func contractMemo(_ contractMemo: String) -> Self {
        self.contractMemo = contractMemo

        return self
    }

    /// The ID of the account to which the contract is staking.
    public var stakedAccountId: AccountId? {
        get {
            guard case .accountId(let accountId) = contractCreateData.stakedId else {
                return nil
            }

            return accountId
        }
        set(value) { contractCreateData.stakedId = value.map(StakedId.accountId) }
    }

    /// Sets the ID of the account to which the contract is staking.
    ///
    /// - Returns: `self`
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountId) -> Self {
        self.stakedAccountId = stakedAccountId

        return self
    }

    /// The ID of the node to which the contract is staking.
    public var stakedNodeId: UInt64? {
        get {
            guard case .nodeId(let nodeId) = contractCreateData.stakedId else {
                return nil
            }

            return nodeId
        }

        set(value) { contractCreateData.stakedId = value.map(StakedId.nodeId) }
    }

    /// Sets the ID of the node to which the contract is staking.
    ///
    /// - Returns: `self`
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: UInt64) -> Self {
        self.stakedNodeId = stakedNodeId

        return self
    }

    /// Sets the client to use for freezing the generated *``ContractCreateTransaction``*.
    ///
    /// By default freezing will use the client provided to ``execute``.
    ///
    /// >Note: This *only* affects the ``ContractCreateTransaction`` currently, that is not guaranteed to always be the case.
    ///
    /// - Returns: `self`
    @discardableResult
    public func freezeWith(_ client: Client) -> Self {
        self.contractCreateData.freezeWithClient = client

        return self
    }

    /// Sets the signer for use in the ``ContractCreateTransaction``
    ///
    /// >Important: Only *one* signer is allowed.
    ///
    /// - Returns: `self`
    @discardableResult
    public func sign(_ key: PrivateKey) -> Self {
        self.contractCreateData.signer = .privateKey(key)

        return self
    }

    /// Sets the signer for use in the ``ContractCreateTransaction``
    ///
    /// >Important: Only *one* signer is allowed.
    ///
    /// - Returns: `self`
    @discardableResult
    public func signWith(_ publicKey: PublicKey, _ signer: @escaping (Data) -> (Data)) -> Self {
        self.contractCreateData.signer = .init(publicKey, signer)

        return self
    }

    /// Generates the required transactions and executes them all.
    ///
    /// - Returns: The repsonse from the ``ContractCreateTransaction``.
    public func execute(_ client: Client, _ timeoutPerTransaction: TimeInterval? = nil) async throws
        -> TransactionResponse
    {
        guard let operatorPublicKey = client.operator?.signer.publicKey else {
            // todo: throw a proper error here
            fatalError("must call Client.setOperator before calling ContractCreateFlow.execute")
        }

        let bytecode = Self.splitBytecode(self.bytecode)
        let fileId = try await Self.makeFileCreateTransaction(
            bytecode: bytecode.fileCreate,
            key: operatorPublicKey,
            nodeAccountIds: nodeAccountIds
        )
        .execute(client, timeoutPerTransaction)
        .getReceipt(client, timeoutPerTransaction)
        .fileId!

        if let fileAppendBytecode = bytecode.fileAppend {
            // note: FileAppendTransaction already waits for receipts, so we don't need to wait for one before executing the ContractCreateTransaction.
            _ = try await Self.makeFileAppendTransaction(
                fileId: fileId,
                bytecode: fileAppendBytecode,
                maxChunks: maxFileAppendChunks,
                nodeAccountIds: nodeAccountIds
            ).executeAll(client, timeoutPerTransaction)
        }

        let response = try await Self.makeContractCreateTransaction(
            fileId: fileId,
            contractCreateData: contractCreateData,
            nodeAccountIds: nodeAccountIds
        ).execute(client, timeoutPerTransaction)

        _ = try await response.getReceipt(client, timeoutPerTransaction)

        // todo: Should this return `response` even if this fails?
        _ = try await Self.makeFileDeleteTransaction(fileId: fileId, nodeAccountIds: nodeAccountIds)
            .execute(client, timeoutPerTransaction)
            .getReceipt(client, timeoutPerTransaction)

        return response
    }

    private static func splitBytecode(_ bytecode: Data) -> (fileCreate: Data, fileAppend: Data?) {
        let bytecode = bytecode.hexStringEncoded().data(using: .utf8)!
        guard bytecode.count > maxFileCreateDataBytes else {
            return (bytecode, nil)
        }

        // note: this uses `subdata` because `Data` is it's own subsequence...
        // It's weirdly written such that the `fileAppendData` wouldn't start at index 0
        // even though that's literally what you'd expect.
        return (
            fileCreate: bytecode.safeSubdata(in: 0..<maxFileCreateDataBytes)!,
            fileAppend: bytecode.safeSubdata(in: maxFileCreateDataBytes..<bytecode.endIndex)!
        )
    }

    private static func makeFileCreateTransaction(
        bytecode: Data,
        key: PublicKey,
        nodeAccountIds: [AccountId]?
    ) -> FileCreateTransaction {
        let tmp = FileCreateTransaction().contents(bytecode).keys([.single(key)])

        if let nodeAccountIds = nodeAccountIds {
            tmp.nodeAccountIds = nodeAccountIds
        }

        return tmp
    }

    private static func makeFileAppendTransaction(
        fileId: FileId,
        bytecode: Data,
        maxChunks: Int?,
        nodeAccountIds: [AccountId]?
    ) -> FileAppendTransaction {
        let tmp = FileAppendTransaction().fileId(fileId).contents(bytecode)

        if let maxChunks = maxChunks {
            tmp.maxChunks = maxChunks
        }

        if let nodeAccountIds = nodeAccountIds {
            tmp.nodeAccountIds = nodeAccountIds
        }

        return tmp
    }

    private static func makeContractCreateTransaction(
        fileId: FileId,
        contractCreateData data: ContractCreateTransactionData,
        nodeAccountIds: [AccountId]?
    ) throws -> ContractCreateTransaction {
        let tmp = ContractCreateTransaction()
            .bytecodeFileId(fileId)
            .constructorParameters(data.constructorParameters)
            .gas(data.gas)
            .initialBalance(data.initialBalance)
            .maxAutomaticTokenAssociations(data.maxAutomaticTokenAssociations)
            .declineStakingReward(data.declineStakingReward)

        if let adminKey = data.adminKey {
            tmp.adminKey = adminKey
        }

        // if let proxyAccountId = data.proxyAccountId {
        //     tmp.proxyAccountId()
        // }

        if let autoRenewAccountId = data.autoRenewAccountId {
            tmp.autoRenewAccountId = autoRenewAccountId
        }

        if let autoRenewPeriod = data.autoRenewPeriod {
            tmp.autoRenewPeriod = autoRenewPeriod
        }

        if let contractMemo = data.contractMemo {
            tmp.contractMemo = contractMemo
        }

        switch data.stakedId {
        case .accountId(let accountId):
            tmp.stakedAccountId = accountId
        case .nodeId(let nodeId):
            tmp.stakedNodeId = nodeId
        case nil:
            break
        }

        if let nodeAccountIds = nodeAccountIds {
            tmp.nodeAccountIds = nodeAccountIds
        }

        if let client = data.freezeWithClient {
            try tmp.freezeWith(client)
        }

        if let signer = data.signer {
            tmp.signWithSigner(signer)
        }

        return tmp
    }

    private static func makeFileDeleteTransaction(fileId: FileId, nodeAccountIds: [AccountId]?) -> FileDeleteTransaction
    {
        let tmp = FileDeleteTransaction(fileId: fileId)

        if let nodeAccountIds = nodeAccountIds {
            tmp.nodeAccountIds = nodeAccountIds
        }

        return tmp
    }
}

// swiftlint:enable type_body_length
