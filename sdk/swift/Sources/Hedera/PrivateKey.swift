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

/// A private key on the Hedera network.
public final class PrivateKey: LosslessStringConvertible, ExpressibleByStringLiteral {
    internal let ptr: OpaquePointer

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    public init?(_ description: String) {
        var key = OpaquePointer.init(bitPattern: 0)
        let err = hedera_private_key_from_string(description, &key)

        if err != HEDERA_ERROR_OK {
            return nil
        }

        ptr = key!
    }

    public required convenience init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    deinit {
        hedera_private_key_free(ptr)
    }

    /// Generates a new Ed25519 private key.
    public static func generateEd25519() -> Self {
        self.init(hedera_private_key_generate_ed25519())
    }

    /// Generates a new ECDSA(secp256k1) private key.
    public static func generateEcdsaSecp256k1() -> Self {
        self.init(hedera_private_key_generate_ecdsa_secp256k1())
    }

    /// Gets the public key which corresponds to this private key.
    public var publicKey: PublicKey {
        PublicKey(hedera_private_key_get_public_key(ptr))
    }

    public var description: String {
        let descriptionBytes = hedera_private_key_to_string(ptr)
        let description = String(validatingUTF8: descriptionBytes!)!

        return description
    }
}
