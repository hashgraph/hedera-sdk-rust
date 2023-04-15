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

import CommonCrypto
import Foundation

extension Crypto {
    internal enum Aes {

    }
}

extension Crypto.Aes {
    internal func aes128CbcPadDecrypt(key: Data, iv: Data, message: Data) throws -> Data {
        // hack: replace with "not in place"
        var message = Data(message)

        try aes128CbcPadDecryptInPlace(
            key: key,
            iv: iv,
            message: &message
        )

        return message
    }

    private func aes128CbcPadDecryptInPlace(key: Data, iv: Data, message: inout Data) throws {
        precondition(key.count == 16, "bug: key size \(key.count) incorrect for algorithm")
        precondition(iv.count == 16, "bug: iv size incorrect for algorithm")

        key.withUnsafeBytes { key in
            iv.withUnsafeBytes { iv in
                message.withUnsafeMutableBytes { message in

                    var dataOutMoved: Int = 0

                    let status = CCCrypt(
                        CCOperation(kCCDecrypt),
                        CCAlgorithm(kCCAlgorithmAES),
                        CCOptions(kCCOptionPKCS7Padding),
                        key.baseAddress,
                        key.count,
                        iv.baseAddress,
                        message.baseAddress,
                        message.count,
                        message.baseAddress,
                        message.count,
                        &dataOutMoved
                    )
                }
            }
        }
    }
}
