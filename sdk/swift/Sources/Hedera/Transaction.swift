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

private final class TransactionSources {
    fileprivate init(unsafeFromPtr ptr: OpaquePointer) {
        self.ptr = ptr
    }

    fileprivate let ptr: OpaquePointer

    deinit {
        hedera_transaction_sources_free(ptr)
    }
}

/// A transaction that can be executed on the Hedera network.
public class Transaction: Request, ValidateChecksums, Decodable {
    public init() {}

    private var signers: [Signer] = []
    private var sources: TransactionSources?
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
        try await executeInternal(client, timeout)
    }

    public func executeInternal(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> TransactionResponse {
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

// big type
// swiftlint:disable:next type_body_length
internal enum AnyTransaction {
    case accountCreate(AccountCreateTransaction)
    case accountUpdate(AccountUpdateTransaction)
    case accountDelete(AccountDeleteTransaction)
    case accountAllowanceApprove(AccountAllowanceApproveTransaction)
    case accountAllowanceDelete(AccountAllowanceDeleteTransaction)
    case contractCreate(ContractCreateTransaction)
    case contractUpdate(ContractUpdateTransaction)
    case contractDelete(ContractDeleteTransaction)
    case contractExecute(ContractExecuteTransaction)
    case transfer(TransferTransaction)
    case topicCreate(TopicCreateTransaction)
    case topicUpdate(TopicUpdateTransaction)
    case topicDelete(TopicDeleteTransaction)
    case topicMessageSubmit(TopicMessageSubmitTransaction)
    case fileAppend(FileAppendTransaction)
    case fileCreate(FileCreateTransaction)
    case fileUpdate(FileUpdateTransaction)
    case fileDelete(FileDeleteTransaction)
    case tokenAssociate(TokenAssociateTransaction)
    case tokenBurn(TokenBurnTransaction)
    case tokenCreate(TokenCreateTransaction)
    case tokenDelete(TokenDeleteTransaction)
    case tokenDissociate(TokenDissociateTransaction)
    case tokenFeeScheduleUpdate(TokenFeeScheduleUpdateTransaction)
    case tokenFreeze(TokenFreezeTransaction)
    case tokenGrantKyc(TokenGrantKycTransaction)
    case tokenMint(TokenMintTransaction)
    case tokenPause(TokenPauseTransaction)
    case tokenRevokeKyc(TokenRevokeKycTransaction)
    case tokenUnfreeze(TokenUnfreezeTransaction)
    case tokenUnpause(TokenUnpauseTransaction)
    case tokenUpdate(TokenUpdateTransaction)
    case tokenWipe(TokenWipeTransaction)
    case systemDelete(SystemDeleteTransaction)
    case systemUndelete(SystemUndeleteTransaction)
    case freeze(FreezeTransaction)
    case scheduleCreate(ScheduleCreateTransaction)
    case scheduleSign(ScheduleSignTransaction)
    case scheduleDelete(ScheduleDeleteTransaction)
    case ethereum(EthereumTransaction)

    // swiftlint:disable:next function_body_length cyclomatic_complexity
    internal init(upcasting transaction: Transaction) {
        if let transaction = transaction as? AccountCreateTransaction {
            self = .accountCreate(transaction)
            return
        }

        if let transaction = transaction as? AccountUpdateTransaction {
            self = .accountUpdate(transaction)
            return
        }

        if let transaction = transaction as? AccountDeleteTransaction {
            self = .accountDelete(transaction)
            return
        }

        if let transaction = transaction as? AccountAllowanceApproveTransaction {
            self = .accountAllowanceApprove(transaction)
            return
        }

        if let transaction = transaction as? AccountAllowanceDeleteTransaction {
            self = .accountAllowanceDelete(transaction)
            return
        }

        if let transaction = transaction as? ContractCreateTransaction {
            self = .contractCreate(transaction)
            return
        }

        if let transaction = transaction as? ContractUpdateTransaction {
            self = .contractUpdate(transaction)
            return
        }

        if let transaction = transaction as? ContractDeleteTransaction {
            self = .contractDelete(transaction)
            return
        }

        if let transaction = transaction as? ContractExecuteTransaction {
            self = .contractExecute(transaction)
            return
        }

        if let transaction = transaction as? TransferTransaction {
            self = .transfer(transaction)
            return
        }

        if let transaction = transaction as? TopicCreateTransaction {
            self = .topicCreate(transaction)
            return
        }

        if let transaction = transaction as? TopicUpdateTransaction {
            self = .topicUpdate(transaction)
            return
        }

        if let transaction = transaction as? TopicDeleteTransaction {
            self = .topicDelete(transaction)
            return
        }

        if let transaction = transaction as? TopicMessageSubmitTransaction {
            self = .topicMessageSubmit(transaction)
            return
        }

        if let transaction = transaction as? FileAppendTransaction {
            self = .fileAppend(transaction)
            return
        }

        if let transaction = transaction as? FileCreateTransaction {
            self = .fileCreate(transaction)
            return
        }

        if let transaction = transaction as? FileUpdateTransaction {
            self = .fileUpdate(transaction)
            return
        }

        if let transaction = transaction as? FileDeleteTransaction {
            self = .fileDelete(transaction)
            return
        }

        if let transaction = transaction as? TokenAssociateTransaction {
            self = .tokenAssociate(transaction)
            return
        }

        if let transaction = transaction as? TokenBurnTransaction {
            self = .tokenBurn(transaction)
            return
        }

        if let transaction = transaction as? TokenCreateTransaction {
            self = .tokenCreate(transaction)
            return
        }

        if let transaction = transaction as? TokenDeleteTransaction {
            self = .tokenDelete(transaction)
            return
        }

        if let transaction = transaction as? TokenDissociateTransaction {
            self = .tokenDissociate(transaction)
            return
        }

        if let transaction = transaction as? TokenFeeScheduleUpdateTransaction {
            self = .tokenFeeScheduleUpdate(transaction)
            return
        }

        if let transaction = transaction as? TokenFreezeTransaction {
            self = .tokenFreeze(transaction)
            return
        }

        if let transaction = transaction as? TokenGrantKycTransaction {
            self = .tokenGrantKyc(transaction)
            return
        }

        if let transaction = transaction as? TokenMintTransaction {
            self = .tokenMint(transaction)
            return
        }

        if let transaction = transaction as? TokenPauseTransaction {
            self = .tokenPause(transaction)
            return
        }

        if let transaction = transaction as? TokenRevokeKycTransaction {
            self = .tokenRevokeKyc(transaction)
            return
        }

        if let transaction = transaction as? TokenUnfreezeTransaction {
            self = .tokenUnfreeze(transaction)
            return
        }

        if let transaction = transaction as? TokenUnpauseTransaction {
            self = .tokenUnpause(transaction)
            return
        }

        if let transaction = transaction as? TokenUpdateTransaction {
            self = .tokenUpdate(transaction)
            return
        }

        if let transaction = transaction as? TokenWipeTransaction {
            self = .tokenWipe(transaction)
            return
        }

        if let transaction = transaction as? SystemDeleteTransaction {
            self = .systemDelete(transaction)
            return
        }

        if let transaction = transaction as? SystemUndeleteTransaction {
            self = .systemUndelete(transaction)
            return
        }

        if let transaction = transaction as? FreezeTransaction {
            self = .freeze(transaction)
            return
        }

        if let transaction = transaction as? ScheduleCreateTransaction {
            self = .scheduleCreate(transaction)
            return
        }

        if let transaction = transaction as? ScheduleSignTransaction {
            self = .scheduleSign(transaction)
            return
        }

        if let transaction = transaction as? ScheduleDeleteTransaction {
            self = .scheduleDelete(transaction)
            return
        }

        if let transaction = transaction as? EthereumTransaction {
            self = .ethereum(transaction)
            return
        }

        fatalError("Unrecognized transaction type")
    }

    internal enum Kind: String, Codable {
        case accountCreate
        case accountUpdate
        case accountDelete
        case accountAllowanceApprove
        case accountAllowanceDelete
        case contractCreate
        case contractUpdate
        case contractDelete
        case contractExecute
        case transfer
        case topicCreate
        case topicUpdate
        case topicDelete
        case topicMessageSubmit
        case fileAppend
        case fileCreate
        case fileUpdate
        case fileDelete
        case tokenAssociate
        case tokenBurn
        case tokenCreate
        case tokenDelete
        case tokenDissociate
        case tokenFeeScheduleUpdate
        case tokenFreeze
        case tokenGrantKyc
        case tokenMint
        case tokenPause
        case tokenRevokeKyc
        case tokenUnfreeze
        case tokenUnpause
        case tokenUpdate
        case tokenWipe
        case systemDelete
        case systemUndelete
        case freeze
        case scheduleCreate
        case scheduleSign
        case scheduleDelete
        case ethereum
    }

    fileprivate var transaction: Transaction {
        switch self {
        case .accountCreate(let transaction):
            return transaction
        case .accountUpdate(let transaction):
            return transaction
        case .accountDelete(let transaction):
            return transaction
        case .accountAllowanceApprove(let transaction):
            return transaction
        case .accountAllowanceDelete(let transaction):
            return transaction
        case .contractCreate(let transaction):
            return transaction
        case .contractUpdate(let transaction):
            return transaction
        case .contractDelete(let transaction):
            return transaction
        case .contractExecute(let transaction):
            return transaction
        case .transfer(let transaction):
            return transaction
        case .topicCreate(let transaction):
            return transaction
        case .topicUpdate(let transaction):
            return transaction
        case .topicDelete(let transaction):
            return transaction
        case .topicMessageSubmit(let transaction):
            return transaction
        case .fileAppend(let transaction):
            return transaction
        case .fileCreate(let transaction):
            return transaction
        case .fileUpdate(let transaction):
            return transaction
        case .fileDelete(let transaction):
            return transaction
        case .tokenAssociate(let transaction):
            return transaction
        case .tokenBurn(let transaction):
            return transaction
        case .tokenCreate(let transaction):
            return transaction
        case .tokenDelete(let transaction):
            return transaction
        case .tokenDissociate(let transaction):
            return transaction
        case .tokenFeeScheduleUpdate(let transaction):
            return transaction
        case .tokenFreeze(let transaction):
            return transaction
        case .tokenGrantKyc(let transaction):
            return transaction
        case .tokenMint(let transaction):
            return transaction
        case .tokenPause(let transaction):
            return transaction
        case .tokenRevokeKyc(let transaction):
            return transaction
        case .tokenUnfreeze(let transaction):
            return transaction
        case .tokenUnpause(let transaction):
            return transaction
        case .tokenUpdate(let transaction):
            return transaction
        case .tokenWipe(let transaction):
            return transaction
        case .systemDelete(let transaction):
            return transaction
        case .systemUndelete(let transaction):
            return transaction
        case .freeze(let transaction):
            return transaction
        case .scheduleCreate(let transaction):
            return transaction
        case .scheduleSign(let transaction):
            return transaction
        case .scheduleDelete(let transaction):
            return transaction
        case .ethereum(let transaction):
            return transaction
        }
    }
}

extension AnyTransaction: Decodable {
    internal enum CodingKeys: String, CodingKey {
        case type = "$type"
    }

    // swiftlint:disable:next function_body_length cyclomatic_complexity
    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        let kind = try container.decode(Kind.self, forKey: .type)

        switch kind {
        case .accountCreate:
            self = .accountCreate(try AccountCreateTransaction(from: decoder))
        case .accountUpdate:
            self = .accountUpdate(try AccountUpdateTransaction(from: decoder))
        case .accountDelete:
            self = .accountDelete(try AccountDeleteTransaction(from: decoder))
        case .accountAllowanceApprove:
            self = .accountAllowanceApprove(try AccountAllowanceApproveTransaction(from: decoder))
        case .accountAllowanceDelete:
            self = .accountAllowanceDelete(try AccountAllowanceDeleteTransaction(from: decoder))
        case .contractCreate:
            self = .contractCreate(try ContractCreateTransaction(from: decoder))
        case .contractUpdate:
            self = .contractUpdate(try ContractUpdateTransaction(from: decoder))
        case .contractDelete:
            self = .contractDelete(try ContractDeleteTransaction(from: decoder))
        case .contractExecute:
            self = .contractExecute(try ContractExecuteTransaction(from: decoder))
        case .transfer:
            self = .transfer(try TransferTransaction(from: decoder))
        case .topicCreate:
            self = .topicCreate(try TopicCreateTransaction(from: decoder))
        case .topicUpdate:
            self = .topicUpdate(try TopicUpdateTransaction(from: decoder))
        case .topicDelete:
            self = .topicDelete(try TopicDeleteTransaction(from: decoder))
        case .topicMessageSubmit:
            self = .topicMessageSubmit(try TopicMessageSubmitTransaction(from: decoder))
        case .fileAppend:
            self = .fileAppend(try FileAppendTransaction(from: decoder))
        case .fileCreate:
            self = .fileCreate(try FileCreateTransaction(from: decoder))
        case .fileUpdate:
            self = .fileUpdate(try FileUpdateTransaction(from: decoder))
        case .fileDelete:
            self = .fileDelete(try FileDeleteTransaction(from: decoder))
        case .tokenAssociate:
            self = .tokenAssociate(try TokenAssociateTransaction(from: decoder))
        case .tokenBurn:
            self = .tokenBurn(try TokenBurnTransaction(from: decoder))
        case .tokenCreate:
            self = .tokenCreate(try TokenCreateTransaction(from: decoder))
        case .tokenDelete:
            self = .tokenDelete(try TokenDeleteTransaction(from: decoder))
        case .tokenDissociate:
            self = .tokenDissociate(try TokenDissociateTransaction(from: decoder))
        case .tokenFeeScheduleUpdate:
            self = .tokenFeeScheduleUpdate(try TokenFeeScheduleUpdateTransaction(from: decoder))
        case .tokenFreeze:
            self = .tokenFreeze(try TokenFreezeTransaction(from: decoder))
        case .tokenGrantKyc:
            self = .tokenGrantKyc(try TokenGrantKycTransaction(from: decoder))
        case .tokenMint:
            self = .tokenMint(try TokenMintTransaction(from: decoder))
        case .tokenPause:
            self = .tokenPause(try TokenPauseTransaction(from: decoder))
        case .tokenRevokeKyc:
            self = .tokenRevokeKyc(try TokenRevokeKycTransaction(from: decoder))
        case .tokenUnfreeze:
            self = .tokenUnfreeze(try TokenUnfreezeTransaction(from: decoder))
        case .tokenUnpause:
            self = .tokenUnpause(try TokenUnpauseTransaction(from: decoder))
        case .tokenUpdate:
            self = .tokenUpdate(try TokenUpdateTransaction(from: decoder))
        case .tokenWipe:
            self = .tokenWipe(try TokenWipeTransaction(from: decoder))
        case .systemDelete:
            self = .systemDelete(try SystemDeleteTransaction(from: decoder))
        case .systemUndelete:
            self = .systemUndelete(try SystemUndeleteTransaction(from: decoder))
        case .freeze:
            self = .freeze(try FreezeTransaction(from: decoder))
        case .scheduleCreate:
            self = .scheduleCreate(try ScheduleCreateTransaction(from: decoder))
        case .scheduleSign:
            self = .scheduleSign(try ScheduleSignTransaction(from: decoder))
        case .scheduleDelete:
            self = .scheduleDelete(try ScheduleDeleteTransaction(from: decoder))
        case .ethereum:
            self = .ethereum(try EthereumTransaction(from: decoder))
        }
    }
}
