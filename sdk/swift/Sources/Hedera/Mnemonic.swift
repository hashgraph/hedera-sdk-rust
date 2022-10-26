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

public final class Mnemonic: LosslessStringConvertible, ExpressibleByStringLiteral {
    internal let ptr: OpaquePointer

    public var words: [String] {
        description.components(separatedBy: " ")
    }

    public var isLegacy: Bool {
        hedera_mnemonic_is_legacy(ptr)
    }

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    public static func fromWords(_ words: [String]) throws -> Self {
        // this is kinda backwards, but, it's easier than dealing with a `***char`
        try Self.fromString(words.joined(separator: " "))
    }

    public static func fromString(_ description: String) throws -> Self {
        var ptr = OpaquePointer.init(bitPattern: 0)

        let err = hedera_mnemonic_from_string(description, &ptr)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return Self.init(ptr!)
    }

    public init?(_ description: String) {
        var ptr = OpaquePointer.init(bitPattern: 0)

        let err = hedera_mnemonic_from_string(description, &ptr)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        self.ptr = ptr!
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public static func generate24() -> Self {
        let ptr = hedera_mnemonic_generate_24()
        return Self.init(ptr!)
    }

    public static func generate12() -> Self {
        let ptr = hedera_mnemonic_generate_24()
        return Self.init(ptr!)
    }

    public func toPrivateKey(passphrase: String = "") throws -> PrivateKey {
        var ptr = OpaquePointer.init(bitPattern: 0)

        let err = hedera_mnemonic_to_private_key(self.ptr, passphrase, &ptr)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return PrivateKey.unsafeFromPtr(ptr!)
    }

    public func toLegacyPrivateKey() throws -> PrivateKey {
        var ptr = OpaquePointer.init(bitPattern: 0)

        let err = hedera_mnemonic_to_legacy_private_key(self.ptr, &ptr)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return PrivateKey.unsafeFromPtr(ptr!)
    }

    public func toString() -> String {
        description
    }

    public var description: String {
        let descriptionBytes = hedera_mnemonic_to_string(ptr)
        return String(hString: descriptionBytes!)!
    }

    deinit {
        hedera_mnemonic_free(ptr)
    }
}
