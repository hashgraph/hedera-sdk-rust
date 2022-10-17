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

/// The complete record for a transaction on Hedera that has reached consensus.
/// Response from `TransactionRecordQuery`.
// TODO: assessed_custom_fees
public struct TransactionRecord: Codable {
    /// The status (reach consensus, or failed, or is unknown) and the ID of
    /// any new account/file/instance created.
    public let receipt: TransactionReceipt

    /// The hash of the Transaction that executed (not the hash of any Transaction that failed for
    /// having a duplicate TransactionID).
    public let transactionHash: Data

    /// The consensus timestamp.
    public let consensusTimestamp: Date

    // TODO: TransactionId
    /// The ID of the transaction this record represents.
    public let transactionId: String

    /// The memo that was submitted as part of the transaction.
    public let transactionMemo: String

    /// The actual transaction fee charged.
    public let transactionFee: Hbar

    /// Reference to the scheduled transaction ID that this transaction record represents.
    public let scheduleRef: ScheduleId?

    /// All token associations implicitly created while handling this transaction
    public let automaticTokenAssociations: [TokenAssociation]

    /// In the record of an internal transaction, the consensus timestamp of the user
    /// transaction that spawned it.
    public let parentConsensusTimestamp: Date?

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

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        receipt = try container.decode(TransactionReceipt.self, forKey: .receipt)
        transactionHash = Data(base64Encoded: try container.decode(String.self, forKey: .transactionHash))!
        consensusTimestamp = Date(unixTimestampNanos: try container.decode(UInt64.self, forKey: .consensusTimestamp))
        transactionId = try container.decode(String.self, forKey: .transactionId)
        transactionMemo = try container.decode(String.self, forKey: .transactionMemo)
        transactionFee = try container.decode(Hbar.self, forKey: .transactionFee)
        scheduleRef = try container.decodeIfPresent(ScheduleId.self, forKey: .scheduleRef)
        automaticTokenAssociations = try container.decode([TokenAssociation].self, forKey: .automaticTokenAssociations)

        if let parentConsensusTimestampNanos = try container.decodeIfPresent(
            UInt64.self, forKey: .parentConsensusTimestamp) {
            parentConsensusTimestamp = Date(unixTimestampNanos: parentConsensusTimestampNanos)
        } else {
            parentConsensusTimestamp = nil
        }

        aliasKey = try container.decodeIfPresent(PublicKey.self, forKey: .aliasKey)
        children = try container.decode([TransactionRecord].self, forKey: .children)
        duplicates = try container.decode([TransactionRecord].self, forKey: .duplicates)

        if let ethereumHashB64 = try container.decodeIfPresent(String.self, forKey: .ethereumHash) {
            ethereumHash = Data(base64Encoded: ethereumHashB64)!
        } else {
            ethereumHash = nil
        }
    }
}
