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

/// A public key on the Hedera network.
public final class PublicKey: LosslessStringConvertible, ExpressibleByStringLiteral, Codable {
    private let ptr: OpaquePointer

    internal init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    public init?(_ description: String) {
        var key = OpaquePointer.init(bitPattern: 0)
        let err = hedera_public_key_from_string(description, &key)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        ptr = key!
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public required convenience init(from decoder: Decoder) throws {
        self.init(try decoder.singleValueContainer().decode(String.self))!
    }

    deinit {
        hedera_public_key_free(ptr)
    }

    public var description: String {
        let descriptionBytes = hedera_public_key_to_string(ptr)
        let description = String(validatingUTF8: descriptionBytes!)!

        return description
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(String(describing: self))
    }
}
