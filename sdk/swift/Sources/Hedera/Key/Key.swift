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

public enum Key: Equatable {
    case single(PublicKey)
    case contractId(ContractId)
    case delegatableContractId(ContractId)
    case keyList(KeyList)
}

extension Key: Codable {
    private enum CodingKeys: String, CodingKey {
        case type = "$type"
        case value = "$content"
    }

    private var kind: Kind {
        switch self {
        case .single:
            return .single
        case .contractId:
            return .contractId
        case .delegatableContractId:
            return .delegatableContractId
        case .keyList:
            return .keyList
        }
    }

    private enum Kind: Codable {
        case single
        case contractId
        case delegatableContractId
        case keyList
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(kind, forKey: .type)

        switch self {
        case .single(let publicKey):
            try container.encode(publicKey, forKey: .value)

        case .contractId(let contractId):
            try container.encode(contractId, forKey: .value)

        case .delegatableContractId(let contractId):
            try container.encode(contractId, forKey: .value)

        case .keyList(let keyList):
            try container.encode(keyList, forKey: .value)
        }
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        let kind = try container.decode(Kind.self, forKey: .type)

        switch kind {

        case .single:
            self = try .single(container.decode(.value))

        case .contractId:
            self = try .contractId(container.decode(.value))

        case .delegatableContractId:
            self = try .delegatableContractId(container.decode(.value))

        case .keyList:
            self = try .keyList(container.decode(.value))
        }
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension Key: ToJsonBytes {
    internal static var cToBytes: ToJsonBytesFunc { hedera_key_to_bytes }
}
