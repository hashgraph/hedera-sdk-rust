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

/// A transaction that can be executed on the Hedera network.
public class Transaction: Request {
    private var signers: [Signer] = []

    public typealias Response = TransactionResponse

    private enum CodingKeys: String, CodingKey {
        case type = "$type"
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

    public func execute(_ client: Client) async throws -> TransactionResponse {
        // encode self as a JSON request to pass to Rust
        let requestBytes = try JSONEncoder().encode(self)

        let request = String(data: requestBytes, encoding: .utf8)!

        return try await self.executeEncoded(client, request: request, signers: self.signers)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        let typeName = String(describing: type(of: self))
        let requestName = typeName.prefix(1).lowercased() + typeName.dropFirst().dropLast(11)

        try container.encode(requestName, forKey: .type)
    }
}
