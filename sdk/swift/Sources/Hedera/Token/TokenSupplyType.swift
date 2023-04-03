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

import HederaProtobufs
import SwiftProtobuf

/// Possible token supply types.
/// Can be used to restrict supply to a set maximum.
/// Defaults to `infinite`.
public enum TokenSupplyType {
    case infinite
    case finite
}

extension TokenSupplyType: TryProtobufCodable {
    internal typealias Protobuf = Proto_TokenSupplyType

    internal init(protobuf proto: HederaProtobufs.Proto_TokenSupplyType) throws {
        switch proto {
        case .infinite:
            self = .infinite
        case .finite:
            self = .finite
        case .UNRECOGNIZED(let value):
            throw HError.fromProtobuf("unrecognized TokenSupplyType: `\(value)`")
        }
    }

    internal func toProtobuf() -> HederaProtobufs.Proto_TokenSupplyType {
        switch self {
        case .infinite:
            return .infinite
        case .finite:
            return .finite
        }
    }
}
