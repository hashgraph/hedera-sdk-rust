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

private struct TransactionSources {
    fileprivate init() {
        inner = []
    }

    fileprivate init(protos: [Proto_Transaction]) {
        self.inner = protos
    }

    static func signAll(_ signedTransactions: inout [Proto_SignedTransaction], _ signers: [Signer]) {
        // todo: don't be `O(nmk)`, we can do `O(m(n+k))` if we know all transactions already have the same signers.
        for index in signedTransactions.indices {
            var transaction = signedTransactions[index]
            for signer in signers {

                if (transaction.sigMap.sigPair.contains { signer.publicKey.toBytesRaw().starts(with: $0.pubKeyPrefix) })
                {
                    continue
                }

                let signaturePair = Transaction.SignaturePair(signer(transaction.bodyBytes))
                transaction.sigMap.sigPair.append(signaturePair.toProtobuf())
            }

            signedTransactions[index] = transaction
        }
    }

    func signWith(_ signers: [Signer]) -> Self {
        if signers.isEmpty {
            return self
        }

        var signedTransactions = inner.map { item in
            // unreachable: sources can only be non-none if we were created from `from_bytes`.
            // from_bytes checks all transaction bodies for equality, which involves desrializing all `signed_transaction_bytes`.
            // swiftlint:disable:next force_try
            try! Proto_SignedTransaction(serializedData: item.signedTransactionBytes)
        }

        Self.signAll(&signedTransactions, signers)

        let transactions = signedTransactions.map { transaction in
            Proto_Transaction.with { proto in

                // swiftlint:disable:next force_try
                proto.signedTransactionBytes = try! transaction.serializedData()
            }
        }

        return Self(protos: transactions)
    }

    func makeNodeIdsAndHashes() -> ([AccountId], [TransactionHash]) {
        var nodeAccountIds: [AccountId] = []
        var hashes: [TransactionHash] = []

        for transaction in inner {
            let tx = try! Proto_SignedTransaction(serializedData: transaction.signedTransactionBytes)

            hashes.append(.generate(tx.bodyBytes))

            let nodeAccountId = try! AccountId.fromProtobuf(
                Proto_TransactionBody(serializedData: tx.bodyBytes).nodeAccountID)
            nodeAccountIds.append(nodeAccountId)
        }

        return (nodeAccountIds, hashes)
    }

    var inner: [Proto_Transaction]

}

/// A transaction that can be executed on the Hedera network.
public class Transaction: Request, ValidateChecksums, Decodable {
    public init() {}

    fileprivate var signers: [Signer] = []
    fileprivate var sources: TransactionSources?
    public private(set) var isFrozen: Bool = false
    fileprivate var transactionValidDuration: Duration? = nil

    static let defaultTransactionValidDuration: Duration = Duration(seconds: 120)

    internal func execute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        fatalError("`Transaction.execute(_:GRPCChannel,_:Proto_Transaction) must be implemented by subclasses`")
    }

    internal func toTransactionDataProtobuf(_ nodeAccountId: AccountId, _ transactionId: TransactionId)
        -> Proto_TransactionBody.OneOf_Data
    {
        fatalError(
            "`Transaction.toTransactionDataProtobuf(_:AccountId,_:TransactionId) must be implemented by subclasses`")
    }

    internal func defaultMaxTransactionFee() -> Hbar {
        2
    }

    public typealias Response = TransactionResponse

    private enum CodingKeys: String, CodingKey {
        case maxTransactionFee
        case `operator`
        case isFrozen
        case nodeAccountIds
        case type = "$type"
        case transactionId
    }

    private var `operator`: Operator?

    internal var nodeAccountIds: [AccountId]? {
        willSet {
            ensureNotFrozen(fieldName: "nodeAccountIds")
        }
    }

    /// Explicit transaction ID for this transaction.
    public var transactionId: TransactionId? {
        willSet {
            ensureNotFrozen(fieldName: "transactionId")
        }
    }

    /// The maximum allowed transaction fee for this transaction.
    public var maxTransactionFee: Hbar? {
        willSet {
            ensureNotFrozen(fieldName: "maxTransactionFee")
        }
    }

    /// Sets the maximum allowed transaction fee for this transaction.
    @discardableResult
    public func maxTransactionFee(_ maxTransactionFee: Hbar) -> Self {
        self.maxTransactionFee = maxTransactionFee

        return self
    }

    /// Sets the explicit transaction ID for this transaction.
    @discardableResult
    public func transactionId(_ transactionId: TransactionId) -> Self {
        self.transactionId = transactionId

        return self
    }

    @discardableResult
    public func sign(_ privateKey: PrivateKey) -> Self {
        self.signWith(privateKey.getPublicKey()) { privateKey.sign($0) }
    }

    @discardableResult
    public func signWith(_ publicKey: PublicKey, _ signer: @escaping (Data) -> (Data)) -> Self {
        self.signers.append(Signer(publicKey, signer))

        return self
    }

    @discardableResult
    public func freeze() throws -> Self {
        try freezeWith(nil)
    }

    @discardableResult
    public func freezeWith(_ client: Client?) throws -> Self {
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

    public func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> Response {
        try freezeWith(client)
        if let sources = sources {
            return try await execute2(client, self, sources, timeout)
        } else {
            return try await executeAny(client, self, timeout)
        }
    }

    public func executeInternal(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> TransactionResponse {
        try freezeWith(client)

        if sources == nil {
            return try await executeAny(client, self, timeout)
        }

        // encode self as a JSON request to pass to Rust
        let requestBytes = try JSONEncoder().encode(self)

        let request = String(data: requestBytes, encoding: .utf8)!

        return try await executeEncoded(client, request: request, timeout: timeout)
    }

    private func executeEncoded(_ client: Client, request: String, timeout: TimeInterval?)
        async throws -> Response
    {
        if client.isAutoValidateChecksumsEnabled() {
            try self.validateChecksums(on: client)
        }

        fatalError()

        // // start an unmanaged continuation to bridge a C callback with Swift async
        // let responseBytes: Data = try await withUnmanagedThrowingContinuation { continuation in
        //     let signers = makeHederaSignersFromArray(signers: signers)
        //     // invoke `hedera_execute`, callback will be invoked on request completion
        //     let err = hedera_transaction_execute(
        //         client.ptr, request, continuation, signers, timeout != nil,
        //         timeout ?? 0.0, sources?.ptr
        //     ) { continuation, err, responsePtr in
        //         if let err = HError(err) {
        //             // an error has occurred, consume from the TLS storage for the error
        //             // and throw it up back to the async task
        //             resumeUnmanagedContinuation(continuation, throwing: err)
        //         } else {
        //             // NOTE: we are guaranteed to receive valid UTF-8 on a successful response
        //             let responseText = String(validatingUTF8: responsePtr!)!
        //             let responseBytes = responseText.data(using: .utf8)!

        //             // resumes the continuation which bridges us back into Swift async
        //             resumeUnmanagedContinuation(continuation, returning: responseBytes)
        //         }
        //     }

        //     if let err = HError(err) {
        //         resumeUnmanagedContinuation(continuation, throwing: err)
        //     }
        // }

        // return try Self.decodeResponse(responseBytes)
    }

    public required init(from decoder: Decoder) throws {
        // note: `AnyTransaction` is responsible for checking `$type`
        let container = try decoder.container(keyedBy: CodingKeys.self)

        transactionId = try container.decodeIfPresent(.transactionId)
        nodeAccountIds = try container.decodeIfPresent(.nodeAccountIds)
        isFrozen = try container.decodeIfPresent(.isFrozen) ?? false
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

        try container.encodeIfPresent(nodeAccountIds, forKey: .nodeAccountIds)
    }

    public static func fromBytes(_ bytes: Data) throws -> Transaction {
        let transactionList = try Proto_TransactionList(serializedData: bytes)

        let list: [Proto_Transaction]
        if transactionList.transactionList.isEmpty {
            list = [try Proto_Transaction(serializedData: bytes)]
        } else {
            list = transactionList.transactionList
        }

        let tmp = try list.map { try $0.makeBody() }

        let nodeIds = Set(try tmp.map { try AccountId.fromProtobuf($0.nodeAccountID) })

        let (first, rest) = (tmp[0], tmp[1...])

        for body in rest {
            guard body.transactionID == first.transactionID else {
                throw HError.fromProtobuf("chunked transactions not currently supported")
            }

            guard protoTransactionBodyEqual(body, first) else {
                throw HError.fromProtobuf("transaction parts unexpectedly unequal")
            }
        }

        return try .fromProtobuf(first, nodeAccountIds: Array(nodeIds), sources: TransactionSources(protos: list))
    }

    public func toBytes() throws -> Data {
        precondition(isFrozen, "Transaction must be frozen to call `toBytes`")

        if let sources = sources {
            let transactions = sources.signWith(signers).inner

            // swiftlint:disable:next force_try
            return try! Proto_TransactionList.with { $0.transactionList = transactions }.serializedData()
        }

        guard let transactionId = self.transactionId ?? (`operator`?.generateTransactionId()) else {
            throw HError(kind: .noPayerAccountOrTransactionId, description: "todo")
        }

        let transactionList = try nodeAccountIds!
            .map { try makeRequestInner($0, transactionId).0 }

        // swiftlint:disable:next force_try
        return try! Proto_TransactionList.with { $0.transactionList = transactionList }.serializedData()
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try nodeAccountIds?.validateChecksums(on: ledgerId)
        try transactionId?.validateChecksums(on: ledgerId)
    }

    internal func ensureNotFrozen(fieldName: String? = nil) {
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
    fileprivate static func fromProtobuf(
        _ proto: Proto_Transaction, nodeAccountIds: [AccountId], sources: TransactionSources
    ) throws -> Transaction {
        try .fromProtobuf(proto.makeBody(), nodeAccountIds: nodeAccountIds, sources: sources)
    }

    fileprivate static func fromProtobuf(
        _ proto: Proto_TransactionBody, nodeAccountIds: [AccountId], sources: TransactionSources
    ) throws -> Transaction {
        guard let data = proto.data else {
            throw HError.fromProtobuf("Unexpected missing `TransactionBody.data`x")
        }

        let tmp = try AnyTransaction.fromProtobuf(data).transaction
        tmp.nodeAccountIds = nodeAccountIds
        tmp.sources = sources
        tmp.isFrozen = true

        return tmp
    }
}

extension Proto_Transaction {
    fileprivate func makeBody() throws -> Proto_TransactionBody {
        guard !signedTransactionBytes.isEmpty else {
            throw HError.fromProtobuf("Transaction had no signed transaction bytes")
        }

        let transaction = try Proto_SignedTransaction(serializedData: signedTransactionBytes)

        return try Proto_TransactionBody(serializedData: transaction.bodyBytes)
    }
}

/// Returns true if `lhs == rhs`` other than `transactionId` and `nodeAccountId`, false otherwise.
private func protoTransactionBodyEqual(_ lhs: Proto_TransactionBody, _ rhs: Proto_TransactionBody) -> Bool {
    guard lhs.transactionFee == rhs.transactionFee else {
        return false
    }

    guard lhs.transactionValidDuration == rhs.transactionValidDuration else {
        return false
    }

    guard lhs.generateRecord == rhs.generateRecord else {
        return false
    }

    guard lhs.memo == rhs.memo else {
        return false
    }

    guard lhs.data == rhs.data else {
        return false
    }

    return true
}

extension Transaction {
    fileprivate struct SignaturePair: ToProtobuf {
        internal let signature: Data
        internal let publicKey: PublicKey

        init(_ pair: (PublicKey, Data)) {
            publicKey = pair.0
            signature = pair.1
        }

        func toProtobuf() -> Proto_SignaturePair {
            .with { proto in
                switch publicKey {
                case _ where publicKey.isEcdsa():
                    proto.ecdsaSecp256K1 = signature

                case _ where publicKey.isEd25519():
                    proto.ed25519 = signature

                default:
                    fatalError("Unknown public key kind")
                }
                proto.pubKeyPrefix = publicKey.toBytesRaw()
            }
        }
    }

    internal func makeRequestInner(_ nodeAccountId: AccountId, _ transactionId: TransactionId) throws -> (
        Proto_Transaction, TransactionHash
    ) {
        precondition(self.isFrozen)

        let transactionBody = self.toTransactionBodyProtobuf(nodeAccountId, transactionId)
        // swiftlint:disable:next force_try
        let bodyBytes = try! transactionBody.serializedData()

        var signatures: [SignaturePair] = []

        if let `operator` = self.operator {
            signatures.append(SignaturePair(`operator`.sign(bodyBytes)))
        }

        for signer in signers {
            signatures.append(SignaturePair(signer(bodyBytes)))
        }

        let signedTransaction = Proto_SignedTransaction.with { proto in
            proto.bodyBytes = bodyBytes
            proto.sigMap = Proto_SignatureMap.with { map in
                map.sigPair = signatures.toProtobuf()
            }
        }

        // swiftlint:disable:next force_try
        let signedTransactionBytes = try! signedTransaction.serializedData()
        let hash = TransactionHash.generate(signedTransactionBytes)
        let transaction = Proto_Transaction.with { proto in proto.signedTransactionBytes = signedTransactionBytes }

        return (transaction, hash)
    }

    internal func toTransactionBodyProtobuf(_ nodeAccountId: AccountId, _ transactionId: TransactionId)
        -> Proto_TransactionBody
    {
        precondition(self.isFrozen)

        let data = self.toTransactionDataProtobuf(nodeAccountId, transactionId)
        let maxTransactionFee = self.maxTransactionFee ?? defaultMaxTransactionFee()

        return .with { proto in
            proto.data = data
            proto.transactionID = transactionId.toProtobuf()
            proto.transactionValidDuration = (self.transactionValidDuration ?? Self.defaultTransactionValidDuration)
                .toProtobuf()
            // todo: memo
            proto.memo = ""
            proto.nodeAccountID = nodeAccountId.toProtobuf()
            proto.generateRecord = false
            proto.transactionFee = UInt64(maxTransactionFee.toTinybars())
        }
    }
}

extension Transaction: Execute {
    typealias GrpcRequest = Proto_Transaction

    typealias GrpcResponse = Proto_TransactionResponse

    typealias Context = TransactionHash

    var explicitTransactionId: TransactionId? {
        transactionId
    }

    var requiresTransactionId: Bool {
        true
    }

    func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError {
        if let transactionId = transactionId {
            return HError(
                kind: .transactionPreCheckStatus(status: status),
                description: "Transaction \(transactionId) failed with precheck status \(status)"
            )
        }

        return HError(
            kind: .transactionNoIdPreCheckStatus(status: status),
            description: "Transaction failed with precheck status \(status)"
        )
    }

    func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (
        HederaProtobufs.Proto_Transaction, TransactionHash
    ) {
        precondition(isFrozen)

        return try makeRequestInner(nodeAccountId, transactionId!)
    }

    func makeResponse(
        _ response: HederaProtobufs.Proto_TransactionResponse, _ context: TransactionHash, _ nodeAccountId: AccountId,
        _ transactionId: TransactionId?
    ) throws -> TransactionResponse {
        TransactionResponse(
            nodeAccountId: nodeAccountId, transactionId: transactionId!,
            transactionHash: context.data.base64EncodedString(),
            validateStatus: true)
    }

    static func responsePrecheckStatus(_ response: HederaProtobufs.Proto_TransactionResponse) throws -> Int32 {
        Int32(response.nodeTransactionPrecheckCode.rawValue)
    }
}

private func execute2(
    _ client: Client,
    _ transaction: Transaction,
    _ sources: TransactionSources,
    _ timeout: TimeInterval?
) async throws -> TransactionResponse {
    let sources = sources.signWith(transaction.signers)

    let (nodeAccountIds, hashes) = sources.makeNodeIdsAndHashes()

    let timeout = timeout ?? LegacyExponentialBackoff.defaultMaxElapsedTime

    var backoff = LegacyExponentialBackoff(maxElapsedTime: .limited(timeout))
    var lastError: HError? = nil

    if client.isAutoValidateChecksumsEnabled() {
        try transaction.validateChecksums(on: client)
    }

    var includeUnhealthy = false

    while true {
        let network = client.network
        let nodeIndexes = try network.nodeIndexesForIds(nodeAccountIds)

        inner: for (requestIndex, nodeIndex) in nodeIndexes.enumerated() {
            guard includeUnhealthy || network.isNodeHealthy(nodeIndex, Int64(Timestamp.now.unixTimestampNanos)) else {
                continue
            }

            let (nodeAccountId, channel) = network.channel(for: nodeIndex, on: client.eventLoop)

            let response: Transaction.GrpcResponse

            do {
                response = try await transaction.execute(channel, sources.inner[requestIndex])
            } catch let error as GRPCStatus {
                switch error.code {
                case .unavailable, .resourceExhausted:
                    // NOTE: this is an "unhealthy" node
                    // todo: mark unhealthy

                    // try the next node in our allowed list, immediately
                    lastError = HError(
                        kind: .grpcStatus(status: Int32(error.code.rawValue)),
                        description: error.description
                    )
                    continue inner

                case let code:
                    throw HError(
                        kind: .grpcStatus(status: Int32(code.rawValue)),
                        description: error.description
                    )
                }
            }

            let rawPrecheckStatus = try Transaction.responsePrecheckStatus(response)
            let precheckStatus = Status(rawValue: rawPrecheckStatus)

            switch precheckStatus {
            case .ok where transaction.shouldRetry(forResponse: response):
                lastError = transaction.makeErrorPrecheck(precheckStatus, transaction.transactionId)
                break inner

            case .ok:
                return try transaction.makeResponse(
                    response, hashes[requestIndex], nodeAccountId, transaction.transactionId)

            case .busy, .platformNotActive:
                // NOTE: this is a "busy" node
                // try the next node in our allowed list, immediately
                lastError = transaction.makeErrorPrecheck(precheckStatus, transaction.transactionId)

            case .UNRECOGNIZED(let value):
                throw HError(
                    kind: .responseStatusUnrecognized,
                    description: "response status \(value) unrecognized"
                )

            case let status where transaction.shouldRetryPrecheck(forStatus: precheckStatus):
                // conditional retry on pre-check should back-off and try again
                lastError = transaction.makeErrorPrecheck(status, transaction.transactionId)
                break inner

            default:
                throw transaction.makeErrorPrecheck(precheckStatus, transaction.transactionId)
            }

            guard let timeout = backoff.next() else {
                throw HError.timedOut(source: lastError)
            }

            try await Task.sleep(nanoseconds: UInt64(timeout * 1e9))
        }

        // we've gone through healthy nodes at least once, now we have to try other nodes.
        includeUnhealthy = true
    }
}
