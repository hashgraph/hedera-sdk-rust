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

/// Response from `TopicInfoQuery`.
public final class TopicInfo: Codable {
    internal init(
        topicId: TopicId,
        topicMemo: String,
        runningHash: Data,
        sequenceNumber: UInt64,
        expirationTime: Timestamp?,
        adminKey: Key?,
        submitKey: Key?,
        autoRenewAccountId: AccountId?,
        autoRenewPeriod: Duration?,
        ledgerId: LedgerId
    ) {
        self.topicId = topicId
        self.topicMemo = topicMemo
        self.runningHash = runningHash
        self.sequenceNumber = sequenceNumber
        self.expirationTime = expirationTime
        self.adminKey = adminKey
        self.submitKey = submitKey
        self.autoRenewAccountId = autoRenewAccountId
        self.autoRenewPeriod = autoRenewPeriod
        self.ledgerId = ledgerId
    }

    /// The ID of the topic for which information is requested.
    public let topicId: TopicId

    /// Short publicly visible memo about the topic. No guarantee of uniqueness
    public let topicMemo: String

    /// SHA-384 running hash of (previousRunningHash, topicId, consensusTimestamp, sequenceNumber, message).
    public let runningHash: Data

    /// Sequence number (starting at 1 for the first submitMessage) of messages on the topic.
    public let sequenceNumber: UInt64

    /// Effective consensus timestamp at (and after) which submitMessage calls will no longer succeed on the topic.
    public let expirationTime: Timestamp?

    /// Access control for update/delete of the topic.
    public let adminKey: Key?

    /// Access control for submit message.
    public let submitKey: Key?

    /// An account which will be automatically charged to renew the topic's expiration, at
    /// `auto_renew_period` interval.
    public let autoRenewAccountId: AccountId?

    /// The interval at which the auto-renew account will be charged to extend the topic's expiry.
    public let autoRenewPeriod: Duration?

    /// The ledger ID the response was returned from
    public let ledgerId: LedgerId

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension TopicInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_ConsensusGetTopicInfoResponse

    internal convenience init(fromProtobuf proto: Protobuf) throws {
        let info = proto.topicInfo

        let expirationTime = info.hasExpirationTime ? info.expirationTime : nil
        let adminKey = info.hasAdminKey ? info.adminKey : nil
        let submitKey = info.hasSubmitKey ? info.submitKey : nil
        let autoRenewAccountId = info.hasAutoRenewAccount ? info.autoRenewAccount : nil
        let autoRenewPeriod = info.hasAutoRenewPeriod ? info.autoRenewPeriod : nil

        self.init(
            topicId: .fromProtobuf(proto.topicID),
            topicMemo: info.memo,
            runningHash: info.runningHash,
            sequenceNumber: info.sequenceNumber,
            expirationTime: .fromProtobuf(expirationTime),
            adminKey: try .fromProtobuf(adminKey),
            submitKey: try .fromProtobuf(submitKey),
            autoRenewAccountId: try .fromProtobuf(autoRenewAccountId),
            autoRenewPeriod: .fromProtobuf(autoRenewPeriod),
            ledgerId: LedgerId(info.ledgerID)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.topicID = topicId.toProtobuf()

            proto.topicInfo = .with { info in
                info.memo = topicMemo

                info.runningHash = runningHash
                info.sequenceNumber = sequenceNumber

                if let expirationTime = expirationTime {
                    info.expirationTime = expirationTime.toProtobuf()
                }

                if let adminKey = adminKey {
                    info.adminKey = adminKey.toProtobuf()
                }

                if let submitKey = submitKey {
                    info.submitKey = submitKey.toProtobuf()
                }

                if let autoRenewAccountId = autoRenewAccountId {
                    info.autoRenewAccount = autoRenewAccountId.toProtobuf()
                }
                if let autoRenewPeriod = autoRenewPeriod {
                    info.autoRenewPeriod = autoRenewPeriod.toProtobuf()
                }

                info.ledgerID = ledgerId.bytes
            }
        }
    }
}
