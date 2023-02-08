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

// TODO: assessed_custom_fees
/// The complete record for a transaction on Hedera that has reached consensus.
/// Response from `TransactionRecordQuery`.
public struct TransactionRecord: Codable {
    internal init(
        receipt: TransactionReceipt,
        transactionHash: Data,
        consensusTimestamp: Timestamp,
        contractFunctionResult: ContractFunctionResult?,
        transfers: [Transfer],
        tokenTransfers: [TokenId: [AccountId: Int64]],
        tokenNftTransfers: [TokenId: [TokenNftTransfer]],
        transactionId: TransactionId,
        transactionMemo: String,
        transactionFee: Hbar,
        scheduleRef: ScheduleId?,
        assessedCustomFees: [AssessedCustomFee],
        automaticTokenAssociations: [TokenAssociation],
        parentConsensusTimestamp: Timestamp?,
        aliasKey: PublicKey?,
        children: [TransactionRecord],
        duplicates: [TransactionRecord],
        ethereumHash: Data?,
        evmAddress: EvmAddress?
    ) {
        self.receipt = receipt
        self.transactionHash = transactionHash
        self.consensusTimestamp = consensusTimestamp
        self.contractFunctionResult = contractFunctionResult
        self.transfers = transfers
        self.tokenTransfers = tokenTransfers
        self.tokenNftTransfers = tokenNftTransfers
        self.transactionId = transactionId
        self.transactionMemo = transactionMemo
        self.transactionFee = transactionFee
        self.scheduleRef = scheduleRef
        self.assessedCustomFees = assessedCustomFees
        self.automaticTokenAssociations = automaticTokenAssociations
        self.parentConsensusTimestamp = parentConsensusTimestamp
        self.aliasKey = aliasKey
        self.children = children
        self.duplicates = duplicates
        self.ethereumHash = ethereumHash
        self.evmAddress = evmAddress
    }

    /// The status (reach consensus, or failed, or is unknown) and the ID of
    /// any new account/file/instance created.
    public let receipt: TransactionReceipt

    /// The hash of the Transaction that executed (not the hash of any Transaction that failed for
    /// having a duplicate TransactionID).
    public let transactionHash: Data

    /// The consensus timestamp.
    public let consensusTimestamp: Timestamp

    /// Record of the value returned by the smart contract function or constructor.
    public let contractFunctionResult: ContractFunctionResult?

    /// All hbar transfers as a result of this transaction, such as fees, or
    /// transfers performed by the transaction, or by a smart contract it calls,
    /// or by the creation of threshold records that it triggers.
    public let transfers: [Transfer]

    /// All fungible token transfers as a result of this transaction.
    public let tokenTransfers: [TokenId: [AccountId: Int64]]

    /// All NFT Token transfers as a result of this transaction.
    public let tokenNftTransfers: [TokenId: [TokenNftTransfer]]

    /// The ID of the transaction this record represents.
    public let transactionId: TransactionId

    /// The memo that was submitted as part of the transaction.
    public let transactionMemo: String

    /// The actual transaction fee charged.
    public let transactionFee: Hbar

    /// Reference to the scheduled transaction ID that this transaction record represents.
    public let scheduleRef: ScheduleId?

    /// All custom fees that were assessed during a ``TransferTransaction``, and must be paid if the
    /// transaction status resolved to SUCCESS.
    public let assessedCustomFees: [AssessedCustomFee]

    /// All token associations implicitly created while handling this transaction
    public let automaticTokenAssociations: [TokenAssociation]

    /// In the record of an internal transaction, the consensus timestamp of the user
    /// transaction that spawned it.
    public let parentConsensusTimestamp: Timestamp?

    /// In the record of an internal CryptoCreate transaction triggered by a user
    /// transaction with a (previously unused) alias, the new account's alias.
    public let aliasKey: PublicKey?

    /// The records of processing all child transaction spawned by the transaction with the given
    /// top-level id, in consensus order. Always empty if the top-level status is UNKNOWN.
    public let children: [TransactionRecord]

    /// The records of processing all consensus transaction with the same id as the distinguished
    /// record above, in chronological order.
    public let duplicates: [TransactionRecord]

    /// The keccak256 hash of the ethereumData. This field will only be populated for
    /// `EthereumTransaction`.
    public let ethereumHash: Data?

    /// The last 20 bytes of the keccak-256 hash of a ECDSA_SECP256K1 primitive key.
    public let evmAddress: EvmAddress?

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        receipt = try container.decode(.receipt)
        transactionHash = try container.decodeIfPresent(.transactionHash).map(Data.base64Encoded) ?? Data()
        consensusTimestamp = try container.decode(.consensusTimestamp)
        contractFunctionResult = try container.decodeIfPresent(.contractFunctionResult)
        transfers = try container.decode(.transfers)
        tokenTransfers = try container.decode(.tokenTransfers)
        tokenNftTransfers = try container.decode(.tokenNftTransfers)
        transactionId = try container.decode(.transactionId)
        transactionMemo = try container.decode(.transactionMemo)
        transactionFee = try container.decode(.transactionFee)
        scheduleRef = try container.decodeIfPresent(.scheduleRef)
        assessedCustomFees = try container.decode(.assessedCustomFees)
        automaticTokenAssociations = try container.decode(.automaticTokenAssociations)
        evmAddress = try container.decode(.evmAddress)

        parentConsensusTimestamp = try container.decodeIfPresent(.parentConsensusTimestamp)

        aliasKey = try container.decodeIfPresent(.aliasKey)
        children = try container.decodeIfPresent(.children) ?? []
        duplicates = try container.decodeIfPresent(.duplicates) ?? []
        ethereumHash = try container.decodeIfPresent(.ethereumHash).map(Data.base64Encoded) ?? Data()
    }
}

extension TransactionRecord {
    internal static func fromProtobuf(_ proto: Proto_TransactionGetRecordResponse) throws -> Self {
        return try Self(
            fromProtobuf: proto.transactionRecord,
            duplicates: .fromProtobuf(proto.duplicateTransactionRecords),
            children: .fromProtobuf(proto.childTransactionRecords)
        )
    }

    fileprivate init(fromProtobuf proto: Protobuf, duplicates: [Self], children: [Self]) throws {
        let contractFunctionResult = try proto.body.map { body in
            switch body {
            case .contractCallResult(let result), .contractCreateResult(let result):
                return try ContractFunctionResult.fromProtobuf(result)
            }
        }

        var tokenTransfers: [TokenId: [AccountId: Int64]] = [:]
        var tokenNftTransfers: [TokenId: [TokenNftTransfer]] = [:]

        for transfer in proto.tokenTransferLists {
            let tokenId = TokenId.fromProtobuf(transfer.token)

            var innerTokenTransfers = tokenTransfers[tokenId] ?? [:]

            for accountAmount in transfer.transfers {
                let accountId = try AccountId.fromProtobuf(accountAmount.accountID)
                innerTokenTransfers[accountId] = accountAmount.amount
            }

            var nftTransfers = tokenNftTransfers[tokenId] ?? []

            nftTransfers.append(
                contentsOf: try transfer.nftTransfers.map { try TokenNftTransfer.fromProtobuf($0, tokenId: tokenId) }
            )

            tokenNftTransfers[tokenId] = nftTransfers

            tokenTransfers[tokenId] = innerTokenTransfers
        }

        let evmAddress = !proto.evmAddress.isEmpty ? try EvmAddress(proto.evmAddress) : nil

        self.init(
            receipt: try .fromProtobuf(proto.receipt),
            transactionHash: proto.transactionHash,
            consensusTimestamp: .fromProtobuf(proto.consensusTimestamp),
            contractFunctionResult: contractFunctionResult,
            transfers: try .fromProtobuf(proto.transferList.accountAmounts),
            tokenTransfers: tokenTransfers,
            tokenNftTransfers: tokenNftTransfers,
            transactionId: try .fromProtobuf(proto.transactionID),
            transactionMemo: proto.memo,
            transactionFee: .fromTinybars(Int64(proto.transactionFee)),
            scheduleRef: proto.hasScheduleRef ? .fromProtobuf(proto.scheduleRef) : nil,
            assessedCustomFees: try .fromProtobuf(proto.assessedCustomFees),
            automaticTokenAssociations: try .fromProtobuf(proto.automaticTokenAssociations),
            parentConsensusTimestamp: .fromProtobuf(proto.parentConsensusTimestamp),
            aliasKey: try .fromAliasBytes(proto.alias),
            children: children,
            duplicates: duplicates,
            ethereumHash: proto.ethereumHash,
            evmAddress: evmAddress
        )
    }
}

extension TransactionRecord: TryFromProtobuf {
    internal typealias Protobuf = Proto_TransactionRecord

    internal init(fromProtobuf proto: Protobuf) throws {
        try self.init(fromProtobuf: proto, duplicates: [], children: [])
    }
}
