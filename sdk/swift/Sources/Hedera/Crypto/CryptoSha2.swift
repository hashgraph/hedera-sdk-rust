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

extension Crypto {
    internal enum Sha2 {
        case sha256
        case sha512

        internal static func digest(_ kind: Sha2, _ data: Data) -> Data {
            kind.digest(data)
        }

        internal func digest(_ data: Data) -> Data {
            switch self {
            case .sha256:
                return data.withUnsafeTypedBytes { buffer in
                    var output: UnsafeMutablePointer<UInt8>?
                    let count = hedera_crypto_sha2_sha256_digest(buffer.baseAddress, buffer.count, &output)
                    return Data(bytesNoCopy: output!, count: count, deallocator: .unsafeCHederaBytesFree)
                }
            case .sha512:
                return data.withUnsafeTypedBytes { buffer in
                    var output: UnsafeMutablePointer<UInt8>?
                    let count = hedera_crypto_sha2_sha512_digest(buffer.baseAddress, buffer.count, &output)
                    return Data(bytesNoCopy: output!, count: count, deallocator: .unsafeCHederaBytesFree)
                }
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
    }
}
