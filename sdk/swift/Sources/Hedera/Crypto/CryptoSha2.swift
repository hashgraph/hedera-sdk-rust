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

import CryptoKit
import Foundation

extension Crypto {
    internal enum Sha2 {
        case sha256
        case sha384
        case sha512

        internal static func digest(_ kind: Sha2, _ data: Data) -> Data {
            kind.digest(data)
        }

        internal func digest(_ data: Data) -> Data {
            switch self {
            case .sha256:
                return Data(CryptoKit.SHA256.hash(data: data))
            case .sha384:
                return Data(CryptoKit.SHA384.hash(data: data))
            case .sha512:
                return Data(CryptoKit.SHA512.hash(data: data))
            }
        }

        /// Hash data using the `sha256` algorithm.
        ///
        /// - Parameter data: the data to be hashed.
        ///
        /// - Returns: the hash of `data`.
        internal static func sha256(_ data: Data) -> Data {
            digest(.sha256, data)
        }

        /// Hash data using the `sha384` algorithm.
        ///
        /// - Parameter data: the data to be hashed.
        ///
        /// - Returns: the hash of `data`.
        internal static func sha384(_ data: Data) -> Data {
            digest(.sha384, data)
        }
    }
}
