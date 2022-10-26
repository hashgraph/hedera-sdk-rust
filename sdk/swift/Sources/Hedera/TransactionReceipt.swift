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

// TODO: exchangeRate
/// The summary of a transaction's result so far, if the transaction has reached consensus.
public struct TransactionReceipt: Codable {
    // TODO: enum Status
    /// The consensus status of the transaction; is UNKNOWN if consensus has not been reached, or if
    /// the associated transaction did not have a valid payer signature.
    public let status: String

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
    ///
    public let newTotalSupply: UInt64

    /// In the receipt for a `ScheduleCreateTransaction`, the id of the newly created schedule.
    public let scheduleId: ScheduleId?

    // TODO: TransactionId type
    /// In the receipt of a `ScheduleCreateTransaction` or `ScheduleSignTransaction` that resolves
    /// to `Success`, the `TransactionId` that should be used to query for the receipt or
    /// record of the relevant scheduled transaction.
    public let scheduledTransactionId: String?

    /// In the receipt of a `TokenMintTransaction` for tokens of type `NonFungibleUnique`,
    /// the serial numbers of the newly created NFTs.
    public let serials: [UInt64]?

    /// The receipts of processing all transactions with the given id, in consensus time order.
    public let duplicates: [TransactionReceipt]

    /// The receipts (if any) of all child transactions spawned by the transaction with the
    /// given top-level id, in consensus order.
    public let children: [TransactionReceipt]

    public static func fromBytes(_ bytes: Data) throws -> Self {
        let json: String = try bytes.withUnsafeBytes { (pointer: UnsafeRawBufferPointer) in
            var ptr: UnsafeMutablePointer<CChar>? = UnsafeMutablePointer(bitPattern: 0)
            let err = hedera_transaction_receipt_from_bytes(
                    pointer.baseAddress,
                    pointer.count,
                    &ptr
            )

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return String(hString: ptr!)
        }

        return try JSONDecoder().decode(Self.self, from: json.data(using: .utf8)!)
    }

    private func toBytesInner() throws -> Data {
        let jsonBytes = try JSONEncoder().encode(self)
        let json = String(data: jsonBytes, encoding: .utf8)!
        var buf: UnsafeMutablePointer<UInt8>?
        var buf_size: Int = 0
        let err = hedera_transaction_receipt_to_bytes(json, &buf, &buf_size)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return Data(bytesNoCopy: buf!, count: buf_size, deallocator: Data.unsafeCHederaBytesFree)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        try! toBytesInner()
    }
}
