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
import GRPC
import HederaProtobufs

/// A transaction that can be executed on the Hedera network.
public class Transaction: ValidateChecksums, Codable {
    public typealias Response = TransactionResponse

    public init() {}

    internal private(set) final var signers: [Signer] = []
    internal private(set) final var sources: TransactionSources?
    public private(set) final var isFrozen: Bool = false

    private enum CodingKeys: String, CodingKey {
        case maxTransactionFee
        case `operator`
        case isFrozen
        case nodeAccountIds
        case type = "$type"
        case transactionId
        case transactionMemo
        case transactionValidDuration
    }

    private final var `operator`: Operator?

    internal private(set) final var nodeAccountIds: [AccountId]? {
        willSet {
            ensureNotFrozen(fieldName: "nodeAccountIds")
        }
    }

    internal var defaultMaxTransactionFee: Hbar {
        2
    }

    internal func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        fatalError("Method `Transaction.toTransactionDataProtobuf` must be overridden by `\(type(of: self))`")
    }

    internal func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        fatalError("Method `Transaction.transactionExecute` must be overridden by `\(type(of: self))`")
    }

    /// Explicit transaction ID for this transaction.
    public final var transactionId: TransactionId? {
        willSet {
            ensureNotFrozen(fieldName: "transactionId")
        }
    }

    /// The maximum allowed transaction fee for this transaction.
    public final var maxTransactionFee: Hbar? {
        willSet {
            ensureNotFrozen(fieldName: "maxTransactionFee")
        }
    }

    /// Sets the maximum allowed transaction fee for this transaction.
    @discardableResult
    public final func maxTransactionFee(_ maxTransactionFee: Hbar) -> Self {
        self.maxTransactionFee = maxTransactionFee

        return self
    }

    public final var transactionValidDuration: Duration? {
        willSet {
            ensureNotFrozen(fieldName: "transactionValidDuration")
        }
    }

    @discardableResult
    public final func transactionValidDuration(_ transactionValidDuration: Duration) -> Self {
        self.transactionValidDuration = transactionValidDuration

        return self
    }

    public final var transactionMemo: String = "" {
        willSet {
            ensureNotFrozen(fieldName: "transactionMemo")
        }
    }

    @discardableResult
    public final func transactionMemo(_ transactionMemo: String) -> Self {
        self.transactionMemo = transactionMemo

        return self
    }

    /// Sets the explicit transaction ID for this transaction.
    @discardableResult
    public final func transactionId(_ transactionId: TransactionId) -> Self {
        self.transactionId = transactionId

        return self
    }

    internal func addSignatureSigner(_ signer: Signer) {
        precondition(isFrozen)

        precondition(nodeAccountIds?.count == 1, "cannot manually add a signature to a transaction with multiple nodes")

        // swiftlint:disable:next force_try
        let sources = try! makeSources()

        self.sources = sources.signWithSigners([signer])
    }

    @discardableResult
    public final func sign(_ privateKey: PrivateKey) -> Self {
        self.signWith(privateKey.publicKey) { privateKey.sign($0) }
    }

    @discardableResult
    public final func signWith(_ publicKey: PublicKey, _ signer: @escaping (Data) -> (Data)) -> Self {
        self.signers.append(Signer(publicKey, signer))

        return self
    }

    @discardableResult
    public final func freeze() throws -> Self {
        try freezeWith(nil)
    }

    @discardableResult
    public final func freezeWith(_ client: Client?) throws -> Self {
        if isFrozen {
            return self
        }

        guard let nodeAccountIds = self.nodeAccountIds ?? client?.randomNodeIds() else {
            throw HError(
                kind: .freezeUnsetNodeAccountIds, description: "transaction frozen without client or explicit node IDs")
        }

        let maxTransactionFee = self.maxTransactionFee ?? client?.maxTransactionFee

        let `operator` = client?.operator

        self.nodeAccountIds = nodeAccountIds
        self.maxTransactionFee = maxTransactionFee
        self.`operator` = `operator`

        isFrozen = true

        if client?.isAutoValidateChecksumsEnabled() == true {
            try validateChecksums(on: client!)
        }

        return self
    }

    @discardableResult
    internal final func makeSources() throws -> TransactionSources {
        precondition(isFrozen)
        if let sources = sources {
            return sources.signWithSigners(self.signers)
        }

        let transactions = try self.makeTransactionList()

        return try! TransactionSources(transactions: transactions)
    }

    public func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> Response {
        try freezeWith(client)

        return try await executeAny(client, self, timeout)
    }

    public required init(from decoder: Decoder) throws {
        // note: `AnyTransaction` is responsible for checking `$type`
        let container = try decoder.container(keyedBy: CodingKeys.self)

        transactionId = try container.decodeIfPresent(.transactionId)
        nodeAccountIds = try container.decodeIfPresent(.nodeAccountIds)
        isFrozen = try container.decodeIfPresent(.isFrozen) ?? false
        transactionValidDuration = try container.decodeIfPresent(.transactionValidDuration)
        transactionMemo = try container.decodeIfPresent(.transactionMemo) ?? ""
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        let typeName = String(describing: type(of: self))
        let requestName = typeName.prefix(1).lowercased() + typeName.dropFirst().dropLast(11)

        try container.encode(requestName, forKey: .type)
        try container.encodeIfPresent(maxTransactionFee, forKey: .maxTransactionFee)
        try container.encode(`operator`, forKey: .operator)
        try container.encodeIfPresent(isFrozen ? isFrozen : nil, forKey: .isFrozen)
        try container.encodeIfPresent(transactionId, forKey: .transactionId)
        try container.encodeIfPresent(transactionValidDuration, forKey: .transactionValidDuration)
        try container.encodeIfPresent(transactionMemo, forKey: .transactionMemo)
        try container.encodeIfPresent(nodeAccountIds, forKey: .nodeAccountIds)
    }

    public static func fromBytes(_ bytes: Data) throws -> Transaction {
        let list: [Proto_Transaction]
        do {
            let tmp = try Proto_TransactionList(contiguousBytes: bytes)

            if tmp.transactionList.isEmpty {
                list = [try Proto_Transaction(contiguousBytes: bytes)]
            } else {
                list = tmp.transactionList
            }
        } catch {
            throw HError.fromProtobuf(String(describing: error))
        }

        let sources = try TransactionSources(transactions: list)

        return try bytes.withUnsafeTypedBytes { buffer in
            var transactionData: UnsafeMutablePointer<CChar>?

            try HError.throwing(
                error: hedera_transaction_from_bytes(
                    buffer.baseAddress,
                    buffer.count,
                    &transactionData
                )
            )

            let transactionString = String(hString: transactionData!)
            let responseBytes = transactionString.data(using: .utf8)!
            // decode the response as the generic output type of this query types
            let transaction = try JSONDecoder().decode(AnyTransaction.self, from: responseBytes).transaction

            transaction.sources = sources

            return transaction
        }
    }

    public final func toBytes() throws -> Data {
        precondition(isFrozen, "Transaction must be frozen to call `toBytes`")

        if let sources = self.sources {
            return sources.toBytes()
        }

        let transactionList = try Proto_TransactionList.with { proto in
            proto.transactionList = try self.makeTransactionList()
        }

        // swiftlint:allow:next force_try
        return try! transactionList.serializedData()
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try nodeAccountIds?.validateChecksums(on: ledgerId)
        try transactionId?.validateChecksums(on: ledgerId)
    }

    internal final func ensureNotFrozen(fieldName: String? = nil) {
        if let fieldName = fieldName {
            precondition(!isFrozen, "\(fieldName) cannot be set while `\(type(of: self))` is frozen")
        } else {
            precondition(
                !isFrozen,
                "`\(type(of: self))` is immutable; it has at least one signature or has been explicitly frozen")
        }
    }
}

extension Transaction {
    internal final func makeResponse(
        _: Proto_TransactionResponse, _ context: TransactionHash, _ nodeAccountId: AccountId,
        _ transactionId: TransactionId?
    ) -> Response {
        TransactionResponse(nodeAccountId: nodeAccountId, transactionId: transactionId!, transactionHash: context)
    }

    internal final func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError {
        if let transactionId = transactionId {
            return HError(
                kind: .transactionPreCheckStatus(status: status, transactionId: transactionId),
                description: "transaction `\(transactionId)` failed pre-check with status `\(status)"
            )
        } else {
            return HError(
                kind: .transactionNoIdPreCheckStatus(status: status),
                description: "transaction without transaction id failed pre-check with status `\(status)`"
            )
        }
    }

    static func responsePrecheckStatus(_ response: Proto_TransactionResponse) -> Int32 {
        Int32(response.nodeTransactionPrecheckCode.rawValue)
    }
}

extension Transaction {
    fileprivate func makeTransactionList() throws -> [Proto_Transaction] {
        assert(self.isFrozen)

        // todo: fix this with chunked transactions.
        guard let initialTransactionId = self.transactionId ?? self.operator?.generateTransactionId() else {
            throw HError.noPayerAccountOrTransactionId
        }

        let usedChunks = (self as? ChunkedTransaction)?.usedChunks ?? 1
        let nodeAccountIds = nodeAccountIds!

        var transactionList: [Proto_Transaction] = []

        // Note: This ordering is *important*,
        // there's no documentation for it but `TransactionList` is sorted by chunk number,
        // then `node_id` (in the order they were added to the transaction)
        for chunk in 0..<usedChunks {
            let currentTransactionId: TransactionId
            switch chunk {
            case 0:
                currentTransactionId = initialTransactionId
            default:
                guard let `operator` = self.operator else {
                    throw HError.noPayerAccountOrTransactionId
                }

                currentTransactionId = `operator`.generateTransactionId()
            }

            for nodeAccountId in nodeAccountIds {
                let chunkInfo = ChunkInfo(
                    current: chunk,
                    total: usedChunks,
                    initialTransactionId: initialTransactionId,
                    currentTransactionId: currentTransactionId,
                    nodeAccountId: nodeAccountId
                )

                transactionList.append(self.makeRequestInner(chunkInfo: chunkInfo).0)
            }
        }

        return transactionList
    }

    internal func makeRequestInner(chunkInfo: ChunkInfo) -> (Proto_Transaction, TransactionHash) {
        assert(self.isFrozen)

        let body: Proto_TransactionBody = self.toTransactionBodyProtobuf(chunkInfo)

        // swiftlint:disable:next force_try
        let bodyBytes = try! body.serializedData()

        var signatures: [SignaturePair] = []

        if let `operator` = self.operator {
            // todo: avoid the `.map(xyz).collect()`
            let operatorSignature = `operator`.signer.sign(bodyBytes)

            signatures.append(SignaturePair((`operator`.signer.publicKey, operatorSignature)))
        }

        for signer in self.signers where signatures.allSatisfy({ $0.publicKey != signer.publicKey }) {
            let signature = signer(bodyBytes)
            signatures.append(SignaturePair(signature))
        }

        let signedTransaction = Proto_SignedTransaction.with { proto in
            proto.bodyBytes = bodyBytes
            proto.sigMap.sigPair = signatures.toProtobuf()
        }

        // swiftlint:disable:next force_try
        let signedTransactionBytes = try! signedTransaction.serializedData()

        let transactionHash = TransactionHash(hashing: signedTransactionBytes)

        let transaction = Proto_Transaction.with { $0.signedTransactionBytes = signedTransactionBytes }

        return (transaction, transactionHash)
    }

    private func toTransactionBodyProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody {
        assert(isFrozen)
        let data = toTransactionDataProtobuf(chunkInfo)

        let maxTransactionFee = self.maxTransactionFee ?? self.defaultMaxTransactionFee

        return .with { proto in
            proto.data = data
            proto.transactionID = chunkInfo.currentTransactionId.toProtobuf()
            proto.transactionValidDuration = (self.transactionValidDuration ?? .minutes(2)).toProtobuf()
            proto.memo = self.transactionMemo
            proto.nodeAccountID = chunkInfo.nodeAccountId.toProtobuf()
            proto.generateRecord = false
            proto.transactionFee = UInt64(maxTransactionFee.toTinybars())
        }
    }
}

extension Transaction: Execute {
    internal typealias GrpcRequest = Proto_Transaction
    internal typealias GrpcResponse = Proto_TransactionResponse
    internal typealias Context = TransactionHash

    var explicitTransactionId: TransactionId? {
        transactionId
    }

    var requiresTransactionId: Bool { true }

    func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (
        GrpcRequest, TransactionHash
    ) {
        assert(isFrozen)

        guard let transactionId = transactionId else {
            throw HError.noPayerAccountOrTransactionId
        }

        return self.makeRequestInner(chunkInfo: .single(transactionId: transactionId, nodeAccountId: nodeAccountId))
    }

    func execute(_ channel: GRPCChannel, _ request: GrpcRequest) async throws -> GrpcResponse {
        try await transactionExecute(channel, request)
    }
}
