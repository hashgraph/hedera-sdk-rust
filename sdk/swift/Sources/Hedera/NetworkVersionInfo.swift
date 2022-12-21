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

public struct NetworkVersionInfo: Codable {
    /// Version of the protobuf schema in use by the network.
    public let protobufVersion: SemanticVersion

    /// Version of the Hedera services in use by the network.
    public let servicesVersion: SemanticVersion

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension NetworkVersionInfo: ProtobufCodable {
    internal typealias Protobuf = Proto_NetworkGetVersionInfoResponse

    internal init(fromProtobuf proto: Protobuf) {
        self.protobufVersion = SemanticVersion.fromProtobuf(proto.hapiProtoVersion)
        self.servicesVersion = SemanticVersion.fromProtobuf(proto.hederaServicesVersion)
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.hapiProtoVersion = protobufVersion.toProtobuf()
            proto.hederaServicesVersion = servicesVersion.toProtobuf()
        }
    }
}
