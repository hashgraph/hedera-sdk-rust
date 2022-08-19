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

// TODO: KeyList
// TODO: ThresholdKey
public enum Key {
    case single(PublicKey)
    case contractId(ContractId)
    case delegatableContractId(ContractId)
}

extension Key: Codable {
    private enum CodingKeys: CodingKey {
        case single
        case contractId
        case delegatableContractId
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        switch self {
        case .single(let publicKey):
            try container.encode(publicKey, forKey: .single)

        case .contractId(let contractId):
            try container.encode(contractId, forKey: .contractId)

        case .delegatableContractId(let contractId):
            try container.encode(contractId, forKey: .delegatableContractId)
        }
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        if let single = try container.decodeIfPresent(PublicKey.self, forKey: .single) {
            self = .single(single)
        } else if let contractId = try container.decodeIfPresent(ContractId.self, forKey: .contractId) {
            self = .contractId(contractId)
        } else if let contractId = try container.decodeIfPresent(ContractId.self, forKey: .delegatableContractId) {
            self = .delegatableContractId(contractId)
        } else {
            fatalError("(BUG) unexpected variant for Key")
        }
    }
}
