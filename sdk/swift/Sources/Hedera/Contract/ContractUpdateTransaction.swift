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
import SwiftProtobuf

/// Updates the fields of a smart contract to the given values.
public final class ContractUpdateTransaction: Transaction {
    /// Create a new `ContractUpdateTransaction`.
    public init(
        contractId: ContractId? = nil,
        expirationTime: Timestamp? = nil,
        adminKey: Key? = nil,
        autoRenewPeriod: Duration? = nil,
        contractMemo: String? = nil,
        maxAutomaticTokenAssociations: UInt32? = nil,
        autoRenewAccountId: AccountId? = nil,
        proxyAccountId: AccountId? = nil,
        stakedAccountId: AccountId? = nil,
        stakedNodeId: Int64? = nil,
        declineStakingReward: Bool? = nil
    ) {
        self.contractId = contractId
        self.expirationTime = expirationTime
        self.adminKey = adminKey
        self.autoRenewPeriod = autoRenewPeriod
        self.contractMemo = contractMemo
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations
        self.autoRenewAccountId = autoRenewAccountId
        self.proxyAccountId = proxyAccountId
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
        self.declineStakingReward = declineStakingReward

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_ContractUpdateTransactionBody) throws {
        let stakedAccountId: AccountId?
        let stakedNodeId: Int64?

        switch data.stakedID {
        case .stakedAccountID(let value):
            stakedAccountId = try .fromProtobuf(value)
            stakedNodeId = nil
        case .stakedNodeID(let value):
            stakedNodeId = value
            stakedAccountId = nil
        case nil:
            stakedAccountId = nil
            stakedNodeId = nil
        }

        let memo: String?

        switch data.memoField {
        case .memo(let value):
            memo = value
        case .memoWrapper(let value):
            memo = value.value
        case nil:
            memo = nil
        }

        self.contractId = data.hasContractID ? try .fromProtobuf(data.contractID) : nil
        self.expirationTime = data.hasExpirationTime ? .fromProtobuf(data.expirationTime) : nil
        self.adminKey = data.hasAdminKey ? try .fromProtobuf(data.adminKey) : nil
        self.autoRenewPeriod = data.hasAutoRenewPeriod ? .fromProtobuf(data.autoRenewPeriod) : nil
        self.contractMemo = memo
        self.maxAutomaticTokenAssociations =
            data.hasMaxAutomaticTokenAssociations ? UInt32(data.maxAutomaticTokenAssociations.value) : nil
        self.autoRenewAccountId = data.hasAutoRenewAccountID ? try .fromProtobuf(data.autoRenewAccountID) : nil
        self.proxyAccountId = data.hasProxyAccountID ? try .fromProtobuf(data.proxyAccountID) : nil
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
        self.declineStakingReward = data.hasDeclineReward ? data.declineReward.value : nil

        try super.init(protobuf: proto)
    }

    /// The contract to be updated.
    public var contractId: ContractId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the contract to be updated.
    @discardableResult
    public func contractId(_ contractId: ContractId?) -> Self {
        self.contractId = contractId

        return self
    }

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    public var expirationTime: Timestamp? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp?) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// The new admin key.
    public var adminKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the new admin key.
    @discardableResult
    public func adminKey(_ adminKey: Key?) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// The auto renew period for this smart contract.
    public var autoRenewPeriod: Duration? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the auto renew period for this smart contract.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: Duration?) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The memo for the new smart contract.
    public var contractMemo: String? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the memo for the new smart contract.
    @discardableResult
    public func contractMemo(_ contractMemo: String?) -> Self {
        self.contractMemo = contractMemo

        return self
    }

    @discardableResult
    public func clearMemo() -> Self {
        contractMemo = nil

        return self
    }

    /// The maximum number of tokens that this contract can be automatically associated with.
    public var maxAutomaticTokenAssociations: UInt32? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the maximum number of tokens that this contract can be automatically associated with.
    @discardableResult
    public func maxAutomaticTokenAssociations(_ maxAutomaticTokenAssociations: UInt32?) -> Self {
        self.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return self
    }

    /// The account to be used at the contract's expiration time to extend the

    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account to be used at the contract's expiration time to extend the
    /// life of the contract.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId?) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    @discardableResult
    public func clearAutoRenewAccountId() -> Self {
        autoRenewAccountId = nil

        return self
    }

    /// The ID of the account to which this account is proxy staked.
    public var proxyAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the ID of the account to which this account is proxy staked.
    @discardableResult
    public func proxyAccountId(_ proxyAccountId: AccountId?) -> Self {
        self.proxyAccountId = proxyAccountId

        return self
    }

    /// The ID of the account to which this contract is staking.
    /// This is mutually exclusive with `staked_node_id`.
    public var stakedAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the ID of the account to which this contract is staking.
    /// This is mutually exclusive with `staked_node_id`.
    @discardableResult
    public func stakedAccountId(_ stakedAccountId: AccountId?) -> Self {
        self.stakedAccountId = stakedAccountId
        stakedNodeId = nil

        return self
    }

    @discardableResult
    public func clearStakedAccountId() -> Self {
        stakedAccountId = 0
        stakedNodeId = nil

        return self
    }

    /// The ID of the node to which this contract is staking.
    /// This is mutually exclusive with `staked_account_id`.
    public var stakedNodeId: Int64? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the ID of the node to which this contract is staking.
    /// This is mutually exclusive with `stakedAccountId`.
    @discardableResult
    public func stakedNodeId(_ stakedNodeId: Int64?) -> Self {
        self.stakedNodeId = stakedNodeId
        stakedAccountId = nil

        return self
    }

    @discardableResult
    public func clearStakedNodeId() -> Self {
        stakedNodeId = -1
        stakedAccountId = nil

        return self
    }

    /// If true, the contract declines receiving a staking reward. The default value is false.
    public var declineStakingReward: Bool? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set to true, the contract declines receiving a staking reward. The default value is false.
    @discardableResult
    public func declineStakingReward(_ declineStakingReward: Bool?) -> Self {
        self.declineStakingReward = declineStakingReward

        return self
    }

    @discardableResult
    public func clearDeclineStakingReward() -> Self {
        declineStakingReward = nil

        return self
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_SmartContractServiceAsyncClient(channel: channel).updateContract(request)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try contractId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try proxyAccountId?.validateChecksums(on: ledgerId)
        try stakedAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .contractUpdateInstance(toProtobuf())
    }
}

extension ContractUpdateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_ContractUpdateTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            contractId?.toProtobufInto(&proto.contractID)
            expirationTime?.toProtobufInto(&proto.expirationTime)
            adminKey?.toProtobufInto(&proto.adminKey)
            autoRenewPeriod?.toProtobufInto(&proto.autoRenewPeriod)

            if let maxAutomaticTokenAssociations = maxAutomaticTokenAssociations {
                proto.maxAutomaticTokenAssociations = Google_Protobuf_Int32Value(
                    Int32(maxAutomaticTokenAssociations))
            }

            autoRenewAccountId?.toProtobufInto(&proto.autoRenewAccountID)
            proxyAccountId?.toProtobufInto(&proto.proxyAccountID)

            if let stakedNodeId = stakedNodeId {
                proto.stakedNodeID = Int64(stakedNodeId)
            }

            if let stakedAccountId = stakedAccountId {
                proto.stakedAccountID = stakedAccountId.toProtobuf()
            }

            if let declineStakingReward = declineStakingReward {
                proto.declineReward = Google_Protobuf_BoolValue(declineStakingReward)
            }
        }
    }
}

extension ContractUpdateTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .contractUpdateInstance(toProtobuf())
    }
}
