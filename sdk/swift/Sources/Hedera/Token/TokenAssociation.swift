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

/// A token <-> account association.
public struct TokenAssociation: Codable {
    /// The token involved in the association.
    public let tokenId: TokenId

    /// The account involved in the association.
    public let accountId: AccountId

    public static func fromBytes(_ bytes: Data) throws -> Self {
        let json: String = try bytes.withUnsafeTypedBytes { pointer in
            var ptr: UnsafeMutablePointer<CChar>? = nil
            let err = hedera_token_association_from_bytes(
                pointer.baseAddress,
                pointer.count,
                &ptr
            )

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return String(hString: ptr!)
        }

        return try JSONDecoder().decode(Self.self, from: json.data(using: .utf8)!)
    }

    private func toBytesInner() throws -> Data {
        let jsonBytes = try JSONEncoder().encode(self)
        let json = String(data: jsonBytes, encoding: .utf8)!
        var buf: UnsafeMutablePointer<UInt8>?
        var bufSize: Int = 0
        let err = hedera_token_association_to_bytes(json, &buf, &bufSize)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return Data(bytesNoCopy: buf!, count: bufSize, deallocator: Data.unsafeCHederaBytesFree)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toBytesInner()
    }
}
