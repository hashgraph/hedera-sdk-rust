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

internal final class TransactionSources {
    fileprivate init(unsafeFromPtr ptr: OpaquePointer) {
        self.ptr = ptr
    }

    internal let ptr: OpaquePointer

    internal func signedWithSingle(_ signer: Signer) -> Self {
        let sourcesOut = hedera_transaction_sources_sign_single(ptr, signer.unsafeIntoHederaSigner())

        return Self(unsafeFromPtr: sourcesOut!)
    }

    internal func signedWith(_ signers: [Signer]) -> Self {
        let sourcesOut = hedera_transaction_sources_sign(ptr, makeHederaSignersFromArray(signers: signers))

        return Self(unsafeFromPtr: sourcesOut!)
    }

    deinit {
        hedera_transaction_sources_free(ptr)
    }
}

/// A transaction that can be executed on the Hedera network.
public class Transaction: Request, ValidateChecksums, Decodable {
    public init() {}

    internal private(set) var signers: [Signer] = []
    internal private(set) var sources: TransactionSources?
    public private(set) var isFrozen: Bool = false

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

    private var nodeAccountIds: [AccountId]? {
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

    internal func addSignatureSigner(_ signer: Signer) {
        precondition(isFrozen)

        precondition(nodeAccountIds?.count == 1, "cannot manually add a signature to a transaction with multiple nodes")

        // swiftlint:disable:next force_try
        let sources = try! makeSources()

        self.sources = sources.signedWithSingle(signer)
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

    @discardableResult
    internal func makeSources() throws -> TransactionSources {
        precondition(isFrozen)
        if let sources = sources {
            return sources.signedWith(self.signers)
        }

        var out: OpaquePointer?

        let requestBytes = try JSONEncoder().encode(self)

        let request = String(data: requestBytes, encoding: .utf8)!

        try HError.throwing(
            error: hedera_transaction_make_sources(request, makeHederaSignersFromArray(signers: signers), &out))

        return TransactionSources(unsafeFromPtr: out!)
    }

    public func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> Response {
        try await executeInternal(client, timeout)
    }

    internal func executeInternal(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> TransactionResponse
    {
        try freezeWith(client)

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

        // start an unmanaged continuation to bridge a C callback with Swift async
        let responseBytes: Data = try await withUnmanagedThrowingContinuation { continuation in
            let signers = makeHederaSignersFromArray(signers: signers)
            // invoke `hedera_execute`, callback will be invoked on request completion
            let err = hedera_transaction_execute(
                client.ptr, request, continuation, signers, timeout != nil,
                timeout ?? 0.0, sources?.ptr
            ) { continuation, err, responsePtr in
                if let err = HError(err) {
                    // an error has occurred, consume from the TLS storage for the error
                    // and throw it up back to the async task
                    resumeUnmanagedContinuation(continuation, throwing: err)
                } else {
                    // NOTE: we are guaranteed to receive valid UTF-8 on a successful response
                    let responseText = String(validatingUTF8: responsePtr!)!
                    let responseBytes = responseText.data(using: .utf8)!

                    // resumes the continuation which bridges us back into Swift async
                    resumeUnmanagedContinuation(continuation, returning: responseBytes)
                }
            }

            if let err = HError(err) {
                resumeUnmanagedContinuation(continuation, throwing: err)
            }
        }

        return try Self.decodeResponse(responseBytes)
    }

    // hack: this should totally be on ChunkedTransaction
    internal func executeAllEncoded(_ client: Client, request: String, timeoutPerChunk: TimeInterval?)
        async throws -> [Response]
    {
        if client.isAutoValidateChecksumsEnabled() {
            try self.validateChecksums(on: client)
        }

        // start an unmanaged continuation to bridge a C callback with Swift async
        let responseBytes: Data = try await withUnmanagedThrowingContinuation { continuation in
            let signers = makeHederaSignersFromArray(signers: signers)
            // invoke `hedera_execute`, callback will be invoked on request completion
            let err = hedera_transaction_execute_all(
                client.ptr, request, continuation, signers, timeoutPerChunk != nil,
                timeoutPerChunk ?? 0.0, sources?.ptr
            ) { continuation, err, responsePtr in
                if let err = HError(err) {
                    // an error has occurred, consume from the TLS storage for the error
                    // and throw it up back to the async task
                    resumeUnmanagedContinuation(continuation, throwing: err)
                } else {
                    // NOTE: we are guaranteed to receive valid UTF-8 on a successful response
                    let responseText = String(validatingUTF8: responsePtr!)!
                    let responseBytes = responseText.data(using: .utf8)!

                    // resumes the continuation which bridges us back into Swift async
                    resumeUnmanagedContinuation(continuation, returning: responseBytes)
                }
            }

            if let err = HError(err) {
                resumeUnmanagedContinuation(continuation, throwing: err)
            }
        }

        return try JSONDecoder().decode([Response].self, from: responseBytes)
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
        try bytes.withUnsafeTypedBytes { buffer in
            var sourcesPointer: OpaquePointer?
            var transactionData: UnsafeMutablePointer<CChar>?

            try HError.throwing(
                error: hedera_transaction_from_bytes(
                    buffer.baseAddress,
                    buffer.count,
                    &sourcesPointer,
                    &transactionData
                )
            )

            let transactionString = String(hString: transactionData!)
            let responseBytes = transactionString.data(using: .utf8)!
            // decode the response as the generic output type of this query types
            let transaction = try JSONDecoder().decode(AnyTransaction.self, from: responseBytes).transaction

            transaction.sources = TransactionSources(unsafeFromPtr: sourcesPointer!)

            return transaction
        }
    }

    public func toBytes() throws -> Data {
        // encode self as a JSON request to pass to Rust
        let requestBytes = try JSONEncoder().encode(self)

        let request = String(data: requestBytes, encoding: .utf8)!

        var buf: UnsafeMutablePointer<UInt8>?
        var size = 0

        try HError.throwing(
            error: hedera_transaction_to_bytes(request, makeHederaSignersFromArray(signers: signers), &buf, &size))

        return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
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
