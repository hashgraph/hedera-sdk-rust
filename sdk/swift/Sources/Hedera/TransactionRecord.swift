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

// TODO: assessed_custom_fees
/// The complete record for a transaction on Hedera that has reached consensus.
/// Response from `TransactionRecordQuery`.
public struct TransactionRecord: Codable {
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

        receipt = try container.decode(TransactionReceipt.self, forKey: .receipt)
        transactionHash = Data(base64Encoded: try container.decode(String.self, forKey: .transactionHash))!
        consensusTimestamp = try container.decode(Timestamp.self, forKey: .consensusTimestamp)
        contractFunctionResult = try container.decodeIfPresent(
            ContractFunctionResult.self, forKey: .contractFunctionResult)
        transfers = try container.decode([Transfer].self, forKey: .transfers)
        tokenTransfers = try container.decode(Dictionary.self, forKey: .tokenTransfers)
        tokenNftTransfers = try container.decode(Dictionary.self, forKey: .tokenNftTransfers)
        transactionId = try container.decode(TransactionId.self, forKey: .transactionId)
        transactionMemo = try container.decode(String.self, forKey: .transactionMemo)
        transactionFee = try container.decode(Hbar.self, forKey: .transactionFee)
        scheduleRef = try container.decodeIfPresent(ScheduleId.self, forKey: .scheduleRef)
        assessedCustomFees = try container.decode([AssessedCustomFee].self, forKey: .assessedCustomFees)
        automaticTokenAssociations = try container.decode([TokenAssociation].self, forKey: .automaticTokenAssociations)
        evmAddress = try container.decode(EvmAddress.self, forKey: .evmAddress)

        parentConsensusTimestamp = try container.decodeIfPresent(Timestamp.self, forKey: .parentConsensusTimestamp)

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
