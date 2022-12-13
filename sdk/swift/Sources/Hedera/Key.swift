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
import HederaProtobufs

public enum Key: Equatable {
    case single(PublicKey)
    case contractId(ContractId)
    case delegatableContractId(ContractId)
    case keyList(KeyList)

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension Key: Codable {
    private enum CodingKeys: CodingKey {
        case single
        case contractId
        case delegatableContractId
        case keyList
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

        case .keyList(let keyList):
            try container.encode(keyList, forKey: .keyList)
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
        } else if let keyList = try container.decodeIfPresent(KeyList.self, forKey: .keyList) {
            self = .keyList(keyList)
        } else {
            fatalError("(BUG) unexpected variant for Key")
        }
    }
}
extension Key: TryProtobufCodable {
    internal typealias Protobuf = Proto_Key

    internal init(protobuf proto: Protobuf) throws {
        guard let key = proto.key else {
            throw HError.fromProtobuf("unexpected empty key in Key")
        }

        switch key {
        case .contractID(let contractId):
            self = .contractId(try .fromProtobuf(contractId))
        case .ed25519(let ed25519Bytes):
            self = .single(try .fromBytesEd25519(ed25519Bytes))
        case .rsa3072:
            throw HError.fromProtobuf("unsupported key kind: Rsa3072")
        case .ecdsa384:
            throw HError.fromProtobuf("unsupported key kind: Rsa384")
        case .thresholdKey(let thresholdKey):
            self = .keyList(try .fromProtobuf(thresholdKey))
        case .keyList(let keyList):
            self = .keyList(try .fromProtobuf(keyList))
        case .ecdsaSecp256K1(let ecdsaBytes):
            self = .single(try .fromBytesEcdsa(ecdsaBytes))
        case .delegatableContractID(let contractId):
            self = .delegatableContractId(try .fromProtobuf(contractId))
        }
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            // this is make sure we set the property by having a `let` constant that *must* be assigned to
            // (we get a compiler error otherwise)
            let key: Protobuf.OneOf_Key
            switch self {
            case .single(let single):
                let bytes = single.toBytesRaw()
                key = single.isEd25519() ? .ed25519(bytes) : .ecdsaSecp256K1(bytes)
            case .contractId(let contractId):
                key = .contractID(contractId.toProtobuf())
            case .delegatableContractId(let delegatableContractId):
                key = .delegatableContractID(delegatableContractId.toProtobuf())
            case .keyList(let keyList):
                key = keyList.toProtobufKey()
            }

            proto.key = key
        }
    }
}
