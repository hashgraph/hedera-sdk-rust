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

internal protocol ValidateChecksums {
    func validateChecksums(on ledgerId: LedgerId) throws

    func validateChecksums(on client: Client) throws
}

extension ValidateChecksums {
    internal func validateChecksums(on client: Client) throws {
        try validateChecksums(on: client.ledgerId!)
    }
}

extension Array: ValidateChecksums where Self.Element: ValidateChecksums {
    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try forEach { element in try element.validateChecksums(on: ledgerId) }
    }
}

/// A transaction or query that can be executed on the Hedera network.
///
/// Do *not* implement this protocol.
/// This protocol is semantically sealed (even if Swift does not support *actually* sealing it).
/// Implementing this protocol may break in any way without warning in any future version of this SDK.
internal protocol Request: Encodable, ValidateChecksums {
    associatedtype Response: Decodable

    func executeInternal(_ client: Client, _ timeout: TimeInterval?) async throws -> Response

    static func decodeResponse(_ responseBytes: Data) throws -> Response
}

extension Request {
    internal func executeEncoded(_ client: Client, request: String, signers: [Signer], timeout: TimeInterval?)
        async throws -> Response
    {
        if client.isAutoValidateChecksumsEnabled() {
            try self.validateChecksums(on: client)
        }

        // start an unmanaged continuation to bridge a C callback with Swift async
        let responseBytes: Data = try await withUnmanagedThrowingContinuation { continuation in
            // invoke `hedera_execute`, callback will be invoked on request completion
            let err = hedera_execute(
                client.ptr, request, continuation, makeHederaSignersFromArray(signers: signers), timeout != nil,
                timeout ?? 0.0
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

    /// Execute this request against the provided client of the Hedera network.
    internal func executeInternal(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> Response {
        // encode self as a JSON request to pass to Rust
        let requestBytes = try JSONEncoder().encode(self)

        let request = String(data: requestBytes, encoding: .utf8)!

        return try await executeEncoded(client, request: request, signers: [], timeout: timeout)
    }

    internal static func decodeResponse(_ responseBytes: Data) throws -> Response {
        // decode the response as the generic output type of this query types
        try JSONDecoder().decode(Response.self, from: responseBytes)
    }
}
