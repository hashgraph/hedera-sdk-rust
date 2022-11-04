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

/// Response from `TopicInfoQuery`.
public final class TopicInfo: Codable {
    /// The ID of the topic for which information is requested.
    public let topicId: TopicId

    /// Short publicly visible memo about the topic. No guarantee of uniqueness
    public let topicMemo: String

    /// SHA-384 running hash of (previousRunningHash, topicId, consensusTimestamp, sequenceNumber, message).
    public let runningHash: Data

    /// Sequence number (starting at 1 for the first submitMessage) of messages on the topic.
    public let sequenceNumber: UInt64

    /// Effective consensus timestamp at (and after) which submitMessage calls will no longer succeed on the topic.
    public let expirationTime: Duration?

    /// Access control for update/delete of the topic.
    public let adminKey: Key?

    /// Access control for submit message.
    public let submitKey: Key?

    /// An account which will be automatically charged to renew the topic's expiration, at
    /// `auto_renew_period` interval.
    public let autoRenewAccountId: AccountId?

    /// The interval at which the auto-renew account will be charged to extend the topic's expiry.
    public let autoRenewPeriod: Duration?
}
