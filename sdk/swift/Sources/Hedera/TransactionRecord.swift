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
public struct TransactionRecord: Decodable {
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

    private enum CodingKeys: CodingKey {
        case receipt
        case transactionHash
        case consensusTimestamp
        case contractFunctionResult
        case transfers
        case tokenTransfers
        case tokenNftTransfers
        case transactionId
        case transactionMemo
        case transactionFee
        case scheduleRef
        case assessedCustomFees
        case automaticTokenAssociations
        case evmAddress
        case parentConsensusTimestamp
        case aliasKey
        case children
        case duplicates
        case ethereumHash
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        receipt = try container.decode(.receipt)
        transactionHash = try container.decodeIfPresent(.transactionHash).map(Data.base64Encoded) ?? Data()
        consensusTimestamp = try container.decode(.consensusTimestamp)
        contractFunctionResult = try container.decodeIfPresent(.contractFunctionResult)
        transfers = try container.decode(.transfers)

        let tokenTransfersWrapper = try container.decode(
            DictionaryWrapper<TokenId, DictionaryWrapper<AccountId, Int64>>.self,
            forKey: .tokenTransfers
        )

        tokenTransfers = tokenTransfersWrapper.value.mapValues { $0.value }
        tokenNftTransfers =
            try
            container.decodeIfPresent(
                DictionaryWrapper<TokenId, [TokenNftTransfer]>.self,
                forKey: .tokenNftTransfers
            )?.value ?? [:]

        transactionId = try container.decode(.transactionId)
        transactionMemo = try container.decode(.transactionMemo)
        transactionFee = try container.decode(.transactionFee)
        scheduleRef = try container.decodeIfPresent(.scheduleRef)
        assessedCustomFees = try container.decode(.assessedCustomFees)
        automaticTokenAssociations = try container.decode(.automaticTokenAssociations)
        evmAddress = try container.decodeIfPresent(.evmAddress)

        parentConsensusTimestamp = try container.decodeIfPresent(.parentConsensusTimestamp)

        aliasKey = try container.decodeIfPresent(.aliasKey)
        children = try container.decodeIfPresent(.children) ?? []
        duplicates = try container.decodeIfPresent(.duplicates) ?? []
        ethereumHash = try container.decodeIfPresent(.ethereumHash).map(Data.base64Encoded) ?? Data()
    }
}

/// Hacky struct to work around the lack of CodingKeyRepresentable before swift 5.6 and macos 12.3
private struct DictionaryWrapper<Key, Value: Codable> where Key: LosslessStringConvertible & Hashable {
    let value: [Key: Value]
}

extension DictionaryWrapper: Codable {
    struct CodingKeys: CodingKey {
        let stringValue: String

        init(stringValue: String) {
            self.stringValue = stringValue
        }

        var intValue: Int? { nil }

        init?(intValue: Int) {
            nil
        }
    }

    init(from decoder: Decoder) throws {
        var dict: [Key: Value] = [:]
        let container = try decoder.container(keyedBy: CodingKeys.self)

        for key in container.allKeys {
            let value = try container.decode(Value.self, forKey: key)

            guard let key = Key(key.stringValue) else {
                throw DecodingError.typeMismatch(
                    Key.self, .init(codingPath: container.codingPath, debugDescription: "Invalid key"))
            }

            dict[key] = value
        }

        self.value = dict
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        for (key, value) in self.value {
            try container.encode(value, forKey: CodingKeys(stringValue: String(describing: key)))
        }
    }
}
