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
    internal enum Pkcs5 {}
}

extension Crypto.Pkcs5 {
    internal static func pbkdf2(
        variant: Crypto.Hmac,
        password: Data,
        salt: Data,
        rounds: UInt32,
        keySize: Int
    ) -> Data {

        let hmac: CHedera.HederaHmacVariant

        switch variant {
        case .sha2(.sha256): hmac = HEDERA_HMAC_VARIANT_SHA2_SHA256
        case .sha2(.sha512): hmac = HEDERA_HMAC_VARIANT_SHA2_SHA512
        case .sha3(.keccak256): hmac = HEDERA_HMAC_VARIANT_SHA3_KECCAK256
        }

        return password.withUnsafeTypedBytes { password in
            salt.withUnsafeTypedBytes { salt in
                var output = Data(repeating: 0, count: keySize)

                output.withUnsafeMutableTypedBytes { key in
                    hedera_crypto_pbkdf2_hmac(
                        hmac,
                        password.baseAddress,
                        password.count,
                        salt.baseAddress,
                        salt.count,
                        rounds,
                        key.baseAddress,
                        key.count
                    )
                }

                return output
            }
        }
    }
}
