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

/// The total fees charged for a transaction, consisting of 3 parts:
/// The node fee, the network fee, and the service fee.
public struct FeeData: Codable {
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
        try Self.fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension FeeData: ToFromJsonBytes {
    internal static var cFromBytes: FromJsonBytesFunc { hedera_fee_data_from_bytes }
    internal static var cToBytes: ToJsonBytesFunc { hedera_fee_data_to_bytes }
}
