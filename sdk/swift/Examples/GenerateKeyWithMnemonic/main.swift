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
        do {
            let mnemonic = Mnemonic.generate24()
            let privateKey = try mnemonic.toPrivateKey()
            let publicKey = privateKey.publicKey

            print("24 word mnemonic: \(mnemonic)")
            print("private key = \(privateKey)")
            print("public key = \(publicKey)")
        }

        do {
            let mnemonic = Mnemonic.generate12()
            let privateKey = try mnemonic.toPrivateKey()
            let publicKey = privateKey.publicKey

            print("12 word mnemonic: \(mnemonic)")
            print("private key = \(privateKey)")
            print("public key = \(publicKey)")
        }
    }
}
