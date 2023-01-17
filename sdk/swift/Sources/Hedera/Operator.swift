/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

internal struct Operator: Codable {
    internal init(accountId: AccountId, signer: PrivateKey) {
        self.accountId = accountId
        self.signer = signer
    }

    internal let accountId: AccountId
    internal let signer: PrivateKey

    internal enum CodingKeys: CodingKey {
        case accountId
        case signer
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        accountId = try container.decode(AccountId.self, forKey: .accountId)
        let signer = try container.decode(String.self, forKey: .signer)
        self.signer = try .fromStringDer(signer)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(accountId, forKey: .accountId)
        try container.encode(signer.toStringDer(), forKey: .signer)
    }
}
