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

/// The total fees charged for a transaction, consisting of 3 parts:
/// The node fee, the network fee, and the service fee.
public struct FeeData {
    public init(node: FeeComponents, network: FeeComponents, service: FeeComponents, kind: FeeDataType) {
        self.node = node
        self.network = network
        self.service = service
        self.kind = kind
    }

    /// Fee charged by the node for this functionality.
    public var node: FeeComponents

    /// Fee charged by Hedera for network operations.
    public var network: FeeComponents

    /// Fee charged by Hedera for providing the service.
    public var service: FeeComponents

    /// A subtype distinguishing between different types of fee data
    /// correlating to the same hedera functionality.
    public var kind: FeeDataType

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self(protobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension FeeData: TryProtobufCodable {
    internal typealias Protobuf = Proto_FeeData

    internal init(protobuf proto: Protobuf) throws {
        self.init(
            node: .fromProtobuf(proto.nodedata),
            network: .fromProtobuf(proto.networkdata),
            service: .fromProtobuf(proto.servicedata),
            kind: try .fromProtobuf(proto.subType)
        )
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.nodedata = node.toProtobuf()
            proto.networkdata = network.toProtobuf()
            proto.servicedata = service.toProtobuf()
            proto.subType = kind.toProtobuf()
        }
    }
}
