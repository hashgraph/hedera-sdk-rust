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
import Network

public struct SocketAddressV4: LosslessStringConvertible, Decodable {
    // name is is to match the other SDKs.
    // swiftlint:disable:next identifier_name
    public var ip: IPv4Address
    public var port: UInt16

    public init?(_ description: String) {
        let parts = description.components(separatedBy: ":")
        guard parts.count == 2 else {
            return nil
        }

        // name is is to match the field
        // swiftlint:disable:next identifier_name
        guard let ip = IPv4Address(parts[0]) else {
            return nil
        }

        guard let port = UInt16(parts[1]) else {
            return nil
        }

        self.ip = ip
        self.port = port
    }

    public init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    public var description: String {
        "\(ip):\(port)"
    }
}

public struct NodeAddress: Decodable {
    /// A non-sequential, unique, static identifier for the node
    public var nodeId: UInt64

    /// The node's X509 RSA public key used to sign stream files.
    public var rsaPublicKey: Data

    /// The account to be paid for queries and transactions sent to this node.
    public var nodeAccountId: AccountId

    /// Hash of the node's TLS certificate.
    ///
    /// Precisely, this field is a string of
    /// hexadecimal characters which, translated to binary, are the SHA-384 hash of
    /// the UTF-8 NFKD encoding of the node's TLS cert in PEM format.
    ///
    /// Its value can be used to verify the node's certificate it presents during TLS negotiations.
    public var tlsCertificateHash: Data

    /// A node's service IP addresses and ports.
    public var serviceEndpoints: [SocketAddressV4]

    /// A description of the node, up to 100 bytes.
    public var description: String
}
