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

/// Change properties for the given topic.
///
/// Any null field is ignored (left unchanged).
///
public final class TopicUpdateTransaction: Transaction {
    /// Create a new `TopicUpdateTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    public init(
        topicId: TopicId? = nil,
        expirationTime: Timestamp? = nil,
        topicMemo: String = "",
        adminKey: Key? = nil,
        submitKey: Key? = nil,
        autoRenewPeriod: Duration? = nil,
        autoRenewAccountId: AccountId? = nil
    ) {
        self.topicId = topicId
        self.expirationTime = expirationTime
        self.topicMemo = topicMemo
        self.adminKey = adminKey
        self.submitKey = submitKey
        self.autoRenewPeriod = autoRenewPeriod
        self.autoRenewAccountId = autoRenewAccountId

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_ConsensusUpdateTopicTransactionBody) throws {
        topicId = data.hasTopicID ? .fromProtobuf(data.topicID) : nil
        expirationTime = data.hasExpirationTime ? .fromProtobuf(data.expirationTime) : nil
        topicMemo = data.hasMemo ? data.memo.value : ""
        adminKey = data.hasAdminKey ? try .fromProtobuf(data.adminKey) : nil
        submitKey = data.hasSubmitKey ? try .fromProtobuf(data.submitKey) : nil
        autoRenewPeriod = data.hasAutoRenewPeriod ? .fromProtobuf(data.autoRenewPeriod) : nil
        autoRenewAccountId = data.hasAutoRenewAccount ? try .fromProtobuf(data.autoRenewAccount) : nil

        try super.init(protobuf: proto)
    }

    /// The topic ID which is being updated in this transaction.
    public var topicId: TopicId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the topic ID which is being updated in this transaction.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

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
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    /// Short publicly visible memo about the topic. No guarantee of uniqueness.
    public var topicMemo: String = "" {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the short publicly visible memo about the topic.
    @discardableResult
    public func topicMemo(_ topicMemo: String) -> Self {
        self.topicMemo = topicMemo

        return self
    }

    /// Access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    public var adminKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    @discardableResult
    public func adminKey(_ adminKey: Key) -> Self {
        self.adminKey = adminKey

        return self
    }

    /// Access control for `TopicMessageSubmitTransaction`.
    public var submitKey: Key? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the access control for `TopicMessageSubmitTransaction`.
    @discardableResult
    public func submitKey(_ submitKey: Key) -> Self {
        self.submitKey = submitKey

        return self
    }

    /// The initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time, if
    /// the `autoRenewAccountId` is configured.
    public var autoRenewPeriod: Duration? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time.
    @discardableResult
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// Account to be used at the topic's expiration time to extend the life of the topic.
    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account to be used at the topic's expiration time to extend the life of the topic.
    @discardableResult
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try topicId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_ConsensusServiceAsyncClient(channel: channel).updateTopic(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .consensusUpdateTopic(toProtobuf())
    }
}

extension TopicUpdateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_ConsensusUpdateTopicTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            topicId?.toProtobufInto(&proto.topicID)
            expirationTime?.toProtobufInto(&proto.expirationTime)
            proto.memo = Google_Protobuf_StringValue(topicMemo)
            adminKey?.toProtobufInto(&proto.adminKey)
            submitKey?.toProtobufInto(&proto.submitKey)
            autoRenewPeriod?.toProtobufInto(&proto.autoRenewPeriod)
            autoRenewAccountId?.toProtobufInto(&proto.autoRenewAccount)
        }
    }
}

extension TopicUpdateTransaction: ToSchedulableTransactionData {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .consensusUpdateTopic(toProtobuf())
    }
}
