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
import CryptoKit
import Foundation

extension Crypto {
    internal enum AesError: Error {
        case bufferTooSmall(available: Int, needed: Int)
        case alignment
        case decode
        case other(Int32)
    }

    internal enum Aes {
    }
}

extension Crypto.Aes {
    internal static func aes128CbcPadDecrypt(key: Data, iv: Data, message: Data) throws -> Data {
        precondition(key.count == 16, "bug: key size \(key.count) incorrect for algorithm")
        precondition(iv.count == 16, "bug: iv size incorrect for algorithm")

        // we have to do the very fun dance of trying a second time if the buffer is too small
        do {
            return try aes128CbcPadDecryptOnce(key: key, iv: iv, message: message, outputCapacity: message.count)
        } catch Crypto.AesError.bufferTooSmall(available: _, let needed) {
            return try aes128CbcPadDecryptOnce(key: key, iv: iv, message: message, outputCapacity: needed)
        }
    }

    private static func aes128CbcPadDecryptOnce(
        key: Data,
        iv: Data,
        message: Data,
        outputCapacity: Int
    ) throws -> Data {
        var output = UnsafeMutableBufferPointer<UInt8>.allocate(capacity: message.count)
        output.initialize(repeating: 0)

        defer {
            output.deallocate()
        }

        let data = try aes128CbcPadDecryptInner(
            key: key,
            iv: iv,
            message: message,
            output: &output
        )

        return data
    }

    private static func aes128CbcPadDecryptInner(
        key: Data,
        iv: Data,
        message: Data,
        output: inout UnsafeMutableBufferPointer<UInt8>
    ) throws -> Data {
        try key.withUnsafeBytes { key in
            try iv.withUnsafeBytes { iv in
                try message.withUnsafeBytes { message in
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
                        output.baseAddress,
                        output.count,
                        &dataOutMoved
                    )

                    switch Int(status) {
                    case kCCSuccess:
                        let tmp = output[..<dataOutMoved]
                        return Data(tmp)

                    case kCCBufferTooSmall:
                        throw Crypto.AesError.bufferTooSmall(available: output.count, needed: dataOutMoved)
                    case kCCAlignmentError:
                        throw Crypto.AesError.alignment
                    case kCCDecodeError:
                        throw Crypto.AesError.decode
                    default:
                        throw Crypto.AesError.other(status)
                    }
                }
            }
        }
    }
}
