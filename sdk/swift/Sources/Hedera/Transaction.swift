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

/// A transaction that can be executed on the Hedera network.
public class Transaction: Request, ValidateChecksums {
    private var signers: [Signer] = []
    public private(set) var isFrozen: Bool = false

    public typealias Response = TransactionResponse

    private enum CodingKeys: String, CodingKey {
        case maxTransactionFee
        case `operator`
        case isFrozen
        case nodeAccountIds
        case type = "$type"
    }

    private var `operator`: Operator?

    private var nodeAccountIds: [AccountId]? {
        willSet(_it) {
            ensureNotFrozen(fieldName: "nodeAccountIds")
        }
    }

    /// The maximum allowed transaction fee for this transaction.
    public var maxTransactionFee: Hbar? = 1 {
        willSet(_it) {
            ensureNotFrozen(fieldName: "maxTransactionFee")
        }
    }

    /// Sets the maximum allowed transaction fee for this transaction.
    @discardableResult
    public func maxTransactionFee(_ maxTransactionFee: Hbar) -> Self {
        self.maxTransactionFee = maxTransactionFee

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

        return try await executeEncoded(client, request: request, signers: signers, timeout: timeout)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        let typeName = String(describing: type(of: self))
        let requestName = typeName.prefix(1).lowercased() + typeName.dropFirst().dropLast(11)

        try container.encode(requestName, forKey: .type)
        try container.encode(maxTransactionFee, forKey: .maxTransactionFee)
        try container.encode(`operator`, forKey: .operator)
        if isFrozen {
            try container.encode(isFrozen, forKey: .isFrozen)
        }

        try container.encodeIfPresent(nodeAccountIds, forKey: .nodeAccountIds)
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
        // do nothing
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
