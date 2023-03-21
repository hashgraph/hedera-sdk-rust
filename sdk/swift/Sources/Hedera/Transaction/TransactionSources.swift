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

import Atomics
import Foundation
import HederaProtobufs
import NIOConcurrencyHelpers

// Note that while we cannot constrain T it is very important that it is a value type.
private final class Box<T> {
    init(value: T) {
        self.value = value
    }

    let value: T
}

extension Box: Sendable where T: Sendable {}

// todo: be more OnceCelly
// Note that while we cannot constrain T it is very important that it is a value type.
private struct OnceRace<T> {
    internal init() {
        value = .init()
    }

    internal init(_ value: T) {
        self.value = .init()
        _ = self.value.storeIfNilThenLoad(Box(value: value))
    }

    private let value: ManagedAtomicLazyReference<Box<T>>

    internal func getOrInit(_ initializer: () -> T) -> Box<T> {
        if let value = value.load() {
            return value
        }

        let value = value.storeIfNilThenLoad(Box(value: initializer()))

        return value
    }
}

// Note: This is fine: https://github.com/apple/swift-atomics/pull/46/files#diff-a94af62997ccad839d2a78a1fe0e824e43ee0523f01a6034a2e2abc34aa440a7R156
// ideally this wouldn't need to be @unchecked
extension OnceRace: @unchecked Sendable where T: Sendable {}

internal struct SourceChunk: Sendable {
    fileprivate init(map: TransactionSources2.Ref, index: Int) {
        self.map = map
        self.index = index
    }

    // Note: This is very explicitly a *strong* reference.
    private let map: TransactionSources2.Ref
    private let index: Int

    internal var range: Range<Int> { map.chunks[index] }
    internal var transactionId: TransactionId { map.transactionIds[index] }
    internal var transactions: ArraySlice<Proto_Transaction> { map.transactions[range] }
    internal var signedTransactions: ArraySlice<Proto_SignedTransaction> { map.signedTransactions[range] }
    internal var nodeIds: [AccountId] { map.nodeIds }
    internal var transactionHashes: ArraySlice<TransactionHash> { map.hashes[range] }
}

/// Serialized transaction data.
///
/// This has COW semantics.
internal struct TransactionSources2: Sendable {
    fileprivate final class Ref: Sendable {
        fileprivate init(
            signedTransactions: [Proto_SignedTransaction],
            transactions: OnceRace<[Proto_Transaction]>,
            chunks: [Range<Int>],
            transactionIds: [TransactionId],
            nodeIds: [AccountId],
            hashes: OnceRace<[TransactionHash]>
        ) {
            self.signedTransactions = signedTransactions
            self.lazyTransactions = transactions
            self.chunks = chunks
            self.transactionIds = transactionIds
            self.nodeIds = nodeIds
            self.lazyHashes = hashes
        }

        fileprivate let signedTransactions: [Proto_SignedTransaction]
        private let lazyTransactions: OnceRace<[Proto_Transaction]>
        fileprivate let chunks: [Range<Int>]
        fileprivate let transactionIds: [TransactionId]
        fileprivate let nodeIds: [AccountId]
        fileprivate let lazyHashes: OnceRace<[TransactionHash]>

        fileprivate var transactions: [Proto_Transaction] {
            lazyTransactions.getOrInit {
                return signedTransactions.map { signed in
                    .with { $0.signedTransactionBytes = try! signed.serializedData() }
                }
            }.value
        }

        fileprivate var hashes: [TransactionHash] {
            lazyHashes.getOrInit {
                signedTransactions.map { TransactionHash(hashing: $0.bodyBytes) }
            }.value
        }
    }

    private let guts: Ref
    internal var signedTransactions: [Proto_SignedTransaction] { guts.signedTransactions }
    internal var transactions: [Proto_Transaction] {
        guts.transactions
    }
    internal var chunksCount: Int { guts.chunks.count }

    // note: this *would* be `some Sequence<SourceChunk>` but that is, of course, a swift 5.7 only feature.
    internal var chunks: LazyMapSequence<Range<Int>, SourceChunk> {
        (0..<chunksCount).lazy.map { SourceChunk(map: self.guts, index: $0) }
    }

    internal var transactionHashes: [TransactionHash] { guts.hashes }
}

// todo: put this where it belongs.
public struct TransactionHash: Sendable {
    internal init(hashing data: Data) {
        self.data = Crypto.Sha2.sha384(data)
    }

    public let data: Data
}

extension TransactionSources2 {
    // this is every bit as insane as the rust method I ported it from :/
    init(transactions: [Proto_Transaction]) throws {
        if transactions.isEmpty {
            throw HError.fromProtobuf("`TransactionList` had no transactions")
        }

        let signedTransactions = try transactions.map { transaction in
            guard !transaction.signedTransactionBytes.isEmpty else {
                throw HError.fromProtobuf("Transaction had no signed transaction bytes")
            }

            return try Proto_SignedTransaction(contiguousBytes: transaction.signedTransactionBytes)
        }

        // ensure all signers (if any) are consistent for all signed transactions.
        // this doesn't compare or validate the signatures,
        // instead it ensures that all signatures in the first signed transation exist in *all* transactions and none extra exist.
        do {
            var iter = signedTransactions.lazy.map { it in
                var tmp = it.sigMap.sigPair.map { $0.pubKeyPrefix }

                // sort to be generous about signature ordering.
                tmp.sort { $0.lexicographicallyPrecedes($1) }

                return tmp
            }.makeIterator()

            if let first = iter.next() {
                guard iter.allSatisfy({ first == $0 }) else {
                    throw HError.fromProtobuf("Transaction has mismatched signatures")
                }
            }
        }

        let transactionInfo = try signedTransactions.map { it in
            let transactionBody: Proto_TransactionBody
            do {
                transactionBody = try Proto_TransactionBody(contiguousBytes: it.bodyBytes)
            } catch {
                throw HError.fromProtobuf(String(describing: error))
            }

            let transactionId = try TransactionId.fromProtobuf(transactionBody.transactionID)
            let nodeAccountId = try AccountId.fromProtobuf(transactionBody.nodeAccountID)

            return (transactionId: transactionId, nodeAccountId: nodeAccountId)
        }

        let chunks: [Range<Int>]
        let transactionIds: [TransactionId]
        let nodeAccountIds: [AccountId]

        do {
            var current: TransactionId?

            let chunkStarts = transactionInfo.enumerated().lazy.compactMap { (index, rest) in
                let (id, _) = rest

                if current != id {
                    current = id

                    return index
                }

                return nil
            }

            var chunksTmp: [Range<Int>] = []

            var previousStart: Int?

            for end in chunkStarts {
                let start = previousStart
                previousStart = end

                if let start = start {
                    chunksTmp.append(start..<end)
                }
            }

            if let start = previousStart {
                chunksTmp.append(start..<transactionInfo.count)
            }

            chunks = chunksTmp

            var transactionIdsTmp: [TransactionId] = []
            var nodeIdsTmp: [AccountId] = []

            for chunk in chunks {
                let transactionId = transactionInfo[chunk.startIndex].transactionId
                guard !transactionIdsTmp.contains(transactionId) else {
                    throw HError.fromProtobuf("duplicate transaction ID between chunked transaction chunks")
                }

                transactionIdsTmp.append(transactionId)

                if nodeIdsTmp.isEmpty {
                    nodeIdsTmp = transactionInfo[chunk].map { $0.nodeAccountId }
                } else {
                    guard nodeIdsTmp.elementsEqual(transactionInfo[chunk].lazy.map { $0.nodeAccountId }) else {
                        throw HError.fromProtobuf("TransactionList has inconsistent node account IDs")
                    }
                }
            }

            transactionIds = transactionIdsTmp
            nodeAccountIds = nodeIdsTmp
        }

        self.guts = Ref(
            signedTransactions: signedTransactions,
            transactions: OnceRace(transactions),
            chunks: chunks,
            transactionIds: transactionIds,
            nodeIds: nodeAccountIds,
            hashes: OnceRace()
        )
    }

    /// Signs all of the transactions with the given signers.
    internal func signWithSigners(_ signers: [Signer]) -> Self {
        guard !signers.isEmpty else {
            return self
        }

        var signedTransactions = self.signedTransactions
        // hack: arrays have no way of telling if they've been cowed ._.
        var mutated: Bool = false

        for signer in signers {
            let pk = signer.publicKey.toBytesRaw()

            if signedTransactions.first?.sigMap.sigPair.contains(where: { pk.starts(with: $0.pubKeyPrefix) }) ?? false {
                // this signer already signed these transactions.
                continue
            }

            mutated = true

            for index in signedTransactions.indices {
                var transaction = signedTransactions[index]

                let signaturePair = Transaction.SignaturePair(signer(transaction.bodyBytes))
                transaction.sigMap.sigPair.append(signaturePair.toProtobuf())

                signedTransactions[index] = transaction
            }
        }

        // Don't COW if all signers were duplicates.
        guard mutated else {
            return self
        }

        return Self(
            guts: Ref(
                signedTransactions: signedTransactions,
                transactions: OnceRace(),
                chunks: guts.chunks,
                transactionIds: guts.transactionIds,
                nodeIds: guts.nodeIds,
                hashes: guts.lazyHashes
            )
        )
    }
}
