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

/// Submit a message for consensus.
///
/// Valid and authorized messages on valid topics will be ordered by the consensus service, gossipped to the
/// mirror net, and published (in order) to all subscribers (from the mirror net) on this topic.
///
/// The `submitKey` (if any) must sign this transaction.
///
/// On success, the resulting `TransactionReceipt` contains the topic's updated `topicSequenceNumber` and
/// `topicRunningHash`.
///
public final class TopicMessageSubmitTransaction: Transaction {
    /// Create a new `TopicMessageSubmitTransaction` ready for configuration.
    public override init() {}

    /// The topic ID to submit this message to.
    public var topicId: TopicId? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the topic ID to submit this message to.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

        return self
    }

    /// Message to be submitted.
    /// Max size of the Transaction (including signatures) is 6KiB.
    public var message: Data = Data() {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the message to be submitted.
    @discardableResult
    public func message(_ message: Data) -> Self {
        self.message = message

        return self
    }

    /// The `TransactionId` of the first chunk.
    ///
    /// Should get copied to every subsequent chunk in a fragmented message.
    public var initialTransactionId: TransactionId? {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the `TransactionId` of the first chunk.
    @discardableResult
    public func initialTransactionId(_ initialTransactionId: TransactionId) -> Self {
        self.initialTransactionId = initialTransactionId

        return self
    }

    /// The total number of chunks in the message.
    /// Defaults to 1.
    public var chunkTotal: Int = 1 {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the total number of chunks in the message.
    @discardableResult
    public func chunkTotal(_ chunkTotal: Int) -> Self {
        self.chunkTotal = chunkTotal

        return self
    }

    /// The sequence number (from 1 to total) of the current chunk in the message.
    /// Defaults to 1.
    public var chunkNumber: Int = 1 {
        willSet(_it) {
            ensureNotFrozen()
        }
    }

    /// Sets the sequence number (from 1 to total) of the current chunk in the message.
    @discardableResult
    public func chunkNumber(_ chunkNumber: Int) -> Self {
        self.chunkNumber = chunkNumber

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case topicId
        case message
        case initialTransactionId
        case chunkTotal
        case chunkNumber
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(topicId, forKey: .topicId)
        try container.encodeIfPresent(message.base64EncodedString(), forKey: .message)
        try container.encodeIfPresent(initialTransactionId, forKey: .initialTransactionId)
        try container.encode(chunkTotal, forKey: .chunkTotal)
        try container.encode(chunkNumber, forKey: .chunkNumber)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try topicId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
