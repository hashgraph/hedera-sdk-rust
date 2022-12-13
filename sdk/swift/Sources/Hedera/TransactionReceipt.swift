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

import CHedera
import Foundation
import HederaProtobufs

// TODO: exchangeRate
/// The summary of a transaction's result so far, if the transaction has reached consensus.
public struct TransactionReceipt: Codable {
    internal init(
        transactionId: TransactionId?,
        status: Status,
        accountId: AccountId?,
        fileId: FileId?,
        contractId: ContractId?,
        topicId: TopicId?,
        topicSequenceNumber: UInt64,
        topicRunningHash: String?,
        topicRunningHashVersion: UInt64,
        tokenId: TokenId?,
        totalSupply: UInt64,
        scheduleId: ScheduleId?,
        scheduledTransactionId: TransactionId?,
        serials: [UInt64]?,
        duplicates: [TransactionReceipt],
        children: [TransactionReceipt]
    ) {
        self.transactionId = transactionId
        self.status = status
        self.accountId = accountId
        self.fileId = fileId
        self.contractId = contractId
        self.topicId = topicId
        self.topicSequenceNumber = topicSequenceNumber
        self.topicRunningHash = topicRunningHash
        self.topicRunningHashVersion = topicRunningHashVersion
        self.tokenId = tokenId
        self.totalSupply = totalSupply
        self.scheduleId = scheduleId
        self.scheduledTransactionId = scheduledTransactionId
        self.serials = serials
        self.duplicates = duplicates
        self.children = children
    }

    /// The ID of the transaction that this is a receipt for.
    public let transactionId: TransactionId?

    /// The consensus status of the transaction; is UNKNOWN if consensus has not been reached, or if
    /// the associated transaction did not have a valid payer signature.
    public let status: Status

    /// In the receipt for an `AccountCreateTransaction`, the id of the newly created account.
    public let accountId: AccountId?

    /// In the receipt for a `FileCreateTransaction`, the id of the newly created file.
    public let fileId: FileId?

    /// In the receipt for a `ContractCreateTransaction`, the id of the newly created contract.
    public let contractId: ContractId?

    /// In the receipt for a `TopicCreateTransaction`, the id of the newly created topic.
    public let topicId: TopicId?
    /// In the receipt for a `TopicMessageSubmitTransaction`, the new sequence number of the topic
    /// that received the message.
    public let topicSequenceNumber: UInt64

    // fixme: make this a `Data``
    // TODO: hash type (?)
    /// In the receipt for a `TopicMessageSubmitTransaction`, the new running hash of the
    /// topic that received the message.
    public let topicRunningHash: String?

    /// In the receipt of a `TopicMessageSubmitTransaction`, the version of the SHA-384
    /// digest used to update the running hash.
    public let topicRunningHashVersion: UInt64

    /// In the receipt for a `TokenCreateTransaction`, the id of the newly created token.
    public let tokenId: TokenId?

    /// Populated in the receipt of `TokenMint`, `TokenWipe`, and `TokenBurn` transactions.
    ///
    /// For fungible tokens, the current total supply of this token.
    /// For non-fungible tokens, the total number of NFTs issued for a given token id.
    public let totalSupply: UInt64

    /// In the receipt for a `ScheduleCreateTransaction`, the id of the newly created schedule.
    public let scheduleId: ScheduleId?

    /// In the receipt of a `ScheduleCreateTransaction` or `ScheduleSignTransaction` that resolves
    /// to `Success`, the `TransactionId` that should be used to query for the receipt or
    /// record of the relevant scheduled transaction.
    public let scheduledTransactionId: TransactionId?

    /// In the receipt of a `TokenMintTransaction` for tokens of type `NonFungibleUnique`,
    /// the serial numbers of the newly created NFTs.
    public let serials: [UInt64]?

    /// The receipts of processing all transactions with the given id, in consensus time order.
    public let duplicates: [TransactionReceipt]

    /// The receipts (if any) of all child transactions spawned by the transaction with the
    /// given top-level id, in consensus order.
    public let children: [TransactionReceipt]

    internal init(
        protobuf proto: Proto_TransactionReceipt,
        duplicates: [TransactionReceipt] = [],
        children: [TransactionReceipt] = [],
        transactionId: TransactionId? = nil
    ) throws {
        let accountId = proto.hasAccountID ? proto.accountID : nil
        let fileId = proto.hasFileID ? proto.fileID : nil
        let contractId = proto.hasContractID ? proto.contractID : nil
        let topicId = proto.hasTopicID ? proto.topicID : nil
        let topicRunningHash = !proto.topicRunningHash.isEmpty ? proto.topicRunningHash : nil
        let tokenId = proto.hasTokenID ? proto.tokenID : nil
        let scheduleId = proto.hasScheduleID ? proto.scheduleID : nil
        let scheduledTransactionId = proto.hasScheduledTransactionID ? proto.scheduledTransactionID : nil
        let serials = !proto.serialNumbers.isEmpty ? proto.serialNumbers : nil
        self.init(
            transactionId: transactionId,
            status: Status(rawValue: Int32(proto.status.rawValue)),
            accountId: try .fromProtobuf(accountId),
            fileId: .fromProtobuf(fileId),
            contractId: try .fromProtobuf(contractId),
            topicId: .fromProtobuf(topicId),
            topicSequenceNumber: proto.topicSequenceNumber,
            topicRunningHash: topicRunningHash?.base64EncodedString(),
            topicRunningHashVersion: proto.topicRunningHashVersion,
            tokenId: .fromProtobuf(tokenId),
            totalSupply: proto.newTotalSupply,
            scheduleId: .fromProtobuf(scheduleId),
            scheduledTransactionId: try .fromProtobuf(scheduledTransactionId),
            serials: serials?.map(UInt64.init),
            duplicates: duplicates,
            children: children
        )
    }

    @discardableResult
    public func validateStatus(_ doValidate: Bool) throws -> Self {
        if doValidate && status != Status.ok {
            throw HError(kind: .receiptStatus(status: status), description: "")
        }

        return self
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension TransactionReceipt: TryProtobufCodable {
    internal typealias Protobuf = Proto_TransactionReceipt

    internal init(fromProtobuf proto: Protobuf) throws {
        try self.init(protobuf: proto)
    }

    func toProtobuf() -> Protobuf {
        .with { proto in
            proto.status = Proto_ResponseCodeEnum(rawValue: Int(status.rawValue))!
            accountId?.toProtobufInto(&proto.accountID)
            fileId?.toProtobufInto(&proto.fileID)
            contractId?.toProtobufInto(&proto.contractID)
            topicId?.toProtobufInto(&proto.topicID)
            proto.topicSequenceNumber = topicSequenceNumber
            proto.topicRunningHashVersion = topicRunningHashVersion
            tokenId?.toProtobufInto(&proto.tokenID)
            proto.newTotalSupply = totalSupply
            scheduleId?.toProtobufInto(&proto.scheduleID)
            scheduledTransactionId?.toProtobufInto(&proto.scheduledTransactionID)
            proto.serialNumbers = serials?.map(Int64.init) ?? []
        }
    }
}
