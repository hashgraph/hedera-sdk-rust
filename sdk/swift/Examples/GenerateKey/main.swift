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

import Foundation
import Hedera

@main
public enum Program {
    public static func main() async throws {
        // Generate a Ed25519 key
        // This is the current recommended default for Hedera

        var keyPrivate = PrivateKey.generateEd25519()
        var keyPublic = keyPrivate.getPublicKey()

        print("ed25519 private = \(keyPrivate)")
        print("ed25519 public = \(keyPublic)")

        // Generate a ECDSA(secp256k1) key
        // This is recommended for better compatibility with Ethereum
        keyPrivate = PrivateKey.generateEcdsa()
        keyPublic = keyPrivate.getPublicKey()

        print("ecdsa(secp256k1) private = \(keyPrivate)")
        print("ecdsa(secp256k1) public = \(keyPublic)")
    }
}
