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

import CryptoSwift
import Foundation

extension Crypto {
    internal enum Sha3 {
        case keccak256

        internal static func digest(_ kind: Sha3, _ data: Data) -> Data {
            kind.digest(data)
        }

        internal func digest(_ data: Data) -> Data {
            switch self {
            case .keccak256:
                return Data(CryptoSwift.SHA3(variant: .keccak256).calculate(for: Array(data)))
            }
        }

        /// Hash data using the `keccak256` algorithm.
        ///
        /// - Parameter data: the data to be hashed.
        ///
        /// - Returns: the hash of `data`.
        internal static func keccak256(_ data: Data) -> Data {
            digest(.keccak256, data)
        }
    }
}
