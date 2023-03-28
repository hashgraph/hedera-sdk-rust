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
import GRPC
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
    fileprivate init(map: TransactionSources.Ref, index: Int) {
        self.map = map
        self.index = index
    }

    // Note: This is very explicitly a *strong* reference.
    private let map: TransactionSources.Ref
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
internal struct TransactionSources: Sendable {
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
                    // should be unreachable.
                    // swiftlint:disable:next force_try
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
    internal var nodeAccountIds: [AccountId] { guts.nodeIds }

    // note: this *would* be `some Sequence<SourceChunk>` but that is, of course, a swift 5.7 only feature.
    internal var chunks: LazyMapSequence<Range<Int>, SourceChunk> {
        (0..<chunksCount).lazy.map { SourceChunk(map: self.guts, index: $0) }
    }

    internal var transactionHashes: [TransactionHash] { guts.hashes }
}

extension TransactionSources {
    // this is every bit as insane as the rust method I ported it from :/
    // swiftlint:disable:next function_body_length
    internal init(transactions: [Proto_Transaction]) throws {
        if transactions.isEmpty {
            throw HError.fromProtobuf("`TransactionList` had no transactions")
        }

        let signedTransactions = try transactions.map { transaction -> Proto_SignedTransaction in
            guard !transaction.signedTransactionBytes.isEmpty else {
                throw HError.fromProtobuf("Transaction had no signed transaction bytes")
            }

            return try Proto_SignedTransaction(contiguousBytes: transaction.signedTransactionBytes)
        }

        // ensure all signers (if any) are consistent for all signed transactions.
        // this doesn't compare or validate the signatures,
        // instead it ensures that all signatures in the first signed transation exist in *all* transactions and none extra exist.
        do {
            var iter = signedTransactions.lazy.map { signedTx -> [Data] in
                var tmp = signedTx.sigMap.sigPair.map { $0.pubKeyPrefix }

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

        let transactionInfo = try signedTransactions.map {
            signedTx -> (transactionId: TransactionId, nodeAccountId: AccountId) in
            let transactionBody: Proto_TransactionBody
            do {
                transactionBody = try Proto_TransactionBody(contiguousBytes: signedTx.bodyBytes)
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

            let chunkStarts = transactionInfo.enumerated().lazy.compactMap { (index, rest) -> Int? in
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
            let key = signer.publicKey.toBytesRaw()

            let sigPairs = signedTransactions.first?.sigMap.sigPair

            if sigPairs?.contains(where: { key.starts(with: $0.pubKeyPrefix) }) ?? false {
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

    internal static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    internal func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension TransactionSources: TryProtobufCodable {
    internal typealias Protobuf = Proto_TransactionList

    internal init(protobuf proto: Proto_TransactionList) throws {
        try self.init(transactions: proto.transactionList)
    }

    internal func toProtobuf() -> Proto_TransactionList {
        .with { $0.transactionList = self.transactions }
    }
}

// fixme: find a better name.
internal struct SourceTransaction<Tx: Transaction> {
    private let inner: Tx
    private let sources: TransactionSources

    internal init(inner: Tx, sources: TransactionSources) {
        self.inner = inner
        self.sources = sources.signWithSigners(inner.signers)
    }

    internal func execute(_ client: Client, timeout: TimeInterval? = nil) async throws -> TransactionResponse {
        try await self.executeAll(client, timeoutPerChunk: timeout)[0]
    }

    internal func executeAll(_ client: Client, timeoutPerChunk: TimeInterval? = nil) async throws
        -> [TransactionResponse]
    {
        var responses: [TransactionResponse] = []

        // fixme: remove the downcast, somehow.
        let waitForReceipt = (inner as? ChunkedTransaction)?.waitForReceipt ?? false

        for chunk in sources.chunks {
            let response = try await executeAny(
                client, SourceTransactionExecuteView(inner: inner, chunk: chunk), timeoutPerChunk)

            if waitForReceipt {
                _ = try await response.getReceipt(client)
            }

            responses.append(response)
        }

        return responses
    }
}

// fixme: better name.
private struct SourceTransactionExecuteView<Tx: Transaction> {
    fileprivate let inner: Tx
    fileprivate let chunk: SourceChunk
    fileprivate let indicesByNodeId: [AccountId: Int]

    internal init(inner: Tx, chunk: SourceChunk) {
        self.inner = inner
        self.chunk = chunk

        indicesByNodeId = Dictionary(
            chunk.nodeIds.enumerated().map { (key: $0.element, value: $0.offset) },
            uniquingKeysWith: { (first, _) in first }
        )
    }
}

extension SourceTransactionExecuteView: ValidateChecksums {
    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try inner.validateChecksums(on: ledgerId)
    }
}

extension SourceTransactionExecuteView: Execute {
    internal typealias GrpcRequest = Proto_Transaction
    internal typealias GrpcResponse = Proto_TransactionResponse
    internal typealias Context = TransactionHash
    internal typealias Response = TransactionResponse

    internal var nodeAccountIds: [AccountId]? {
        chunk.nodeIds
    }

    internal var explicitTransactionId: TransactionId? {
        chunk.transactionId
    }

    internal var requiresTransactionId: Bool {
        true
    }

    internal func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (
        GrpcRequest, Context
    ) {
        assert(transactionId == chunk.transactionId)

        let index = self.indicesByNodeId[nodeAccountId]!

        return (self.chunk.transactions[index], self.chunk.transactionHashes[index])
    }

    internal func execute(_ channel: GRPCChannel, _ request: GrpcRequest) async throws -> GrpcResponse {
        try await inner.transactionExecute(channel, request)
    }

    internal func makeResponse(
        _ response: GrpcResponse, _ context: TransactionHash, _ nodeAccountId: AccountId,
        _ transactionId: TransactionId?
    ) -> Response {
        inner.makeResponse(response, context, nodeAccountId, transactionId)
    }

    internal func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError {
        inner.makeErrorPrecheck(status, transactionId)
    }

    internal static func responsePrecheckStatus(_ response: GrpcResponse) -> Int32 {
        Tx.responsePrecheckStatus(response)
    }
}
