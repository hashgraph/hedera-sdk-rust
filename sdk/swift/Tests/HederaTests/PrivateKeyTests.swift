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

import XCTest

@testable import Hedera

private func keyWithChain(key: PrivateKey, chainCode: String) -> PrivateKey {
    key.withChainCode(chainCode: Data(hexEncoded: chainCode)!)
}

internal final class PrivateKeyTests: XCTestCase {
    internal func testParseEd25519() throws {
        let privateKey: PrivateKey =
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312"

        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312")
    }

    internal func testParseEcdsa() throws {
        let privateKey: PrivateKey =
            "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048"

        XCTAssertEqual(
            privateKey.description,
            "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048")
    }

    internal func testEd25519Sign() throws {
        let message = "hello, world".data(using: .utf8)!
        let privateKey: PrivateKey =
            "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10"

        let signature = privateKey.sign(message)

        // note that CryptoKit randomizes the signature, *sigh*, so the only thing we *can* test is that the signature verifies.

        XCTAssertNoThrow(try privateKey.publicKey.verify(message, signature))
    }

    internal func testEcdsaSign() throws {
        let privateKey: PrivateKey =
            "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048"

        let signature = privateKey.sign("hello world".data(using: .utf8)!)

        XCTAssertEqual(
            signature.hexStringEncoded(),
            "f3a13a555f1f8cd6532716b8f388bd4e9d8ed0b252743e923114c0c6cbfe414c086e3717a6502c3edff6130d34df252fb94b6f662d0cd27e2110903320563851"
        )
    }

    internal func testEd25519LegacyDerive() throws {
        let privateKey: PrivateKey =
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312"

        let privateKey0 = try privateKey.legacyDerive(0)

        XCTAssertEqual(
            privateKey0.description,
            "302e020100300506032b6570042204202b7345f302a10c2a6d55bf8b7af40f125ec41d780957826006d30776f0c441fb")

        let privateKeyNeg1 = try privateKey.legacyDerive(-1)

        XCTAssertEqual(
            privateKeyNeg1.description,
            "302e020100300506032b657004220420caffc03fdb9853e6a91a5b3c57a5c0031d164ce1c464dea88f3114786b5199e5")
    }

    internal func testEd25519LegacyDerive2() throws {
        let privateKey: PrivateKey =
            "302e020100300506032b65700422042000c2f59212cb3417f0ee0d38e7bd876810d04f2dd2cb5c2d8f26ff406573f2bd"

        let privateKeyMhw = try privateKey.legacyDerive(0xff_ffff_ffff)

        XCTAssertEqual(
            privateKeyMhw.description,
            "302e020100300506032b6570042204206890dc311754ce9d3fc36bdf83301aa1c8f2556e035a6d0d13c2cccdbbab1242")
    }

    // "iosKey"
    internal func testEd25519Derive1() throws {
        let key = keyWithChain(
            key: "302e020100300506032b657004220420a6b9548d7e123ad4c8bc6fee58301e9b96360000df9d03785c07b620569e7728",
            chainCode: "cde7f535264f1db4e2ded409396f8c72f8075cc43757bd5a205c97699ea40271"
        )

        let child = try key.derive(0)

        XCTAssertEqual(
            child.prettyPrint(),
            #"""
            PrivateKey.ed25519(
                key: 5f66a51931e8c99089472e0d70516b6272b94dd772b967f8221e1077f966dbda,
                chainCode: Optional("0e5c869c1cf9daecd03edb2d49cf2621412578a352578a4bb7ef4eef2942b7c9")
            )
            """#
        )
    }

    // "androidKey"
    internal func testEd25519Derive2() throws {
        let key = keyWithChain(
            key:
                "302e020100300506032b65700422042097dbce1988ef8caf5cf0fd13a5374969e2be5f50650abd19314db6b32f96f18e",
            chainCode: "b7b406314eb2224f172c1907fe39f807e306655e81f2b3bc4766486f42ef1433"
        )

        let child = try key.derive(0)

        XCTAssertEqual(
            child.prettyPrint(),
            #"""
            PrivateKey.ed25519(
                key: c284c25b3a1458b59423bc289e83703b125c8eefec4d5aa1b393c2beb9f2bae6,
                chainCode: Optional("a7a1c2d115a988e51efc12c23692188a4796b312a4a700d6c703e4de4cf1a7f6")
            )
            """#
        )
    }

    internal func testEd25519FromPem() throws {
        let pemString = """
            -----BEGIN PRIVATE KEY-----
            MC4CAQAwBQYDK2VwBCIEINtIS4KOZLLY8SzjwKDpOguMznrxu485yXcyOUSCU44Q
            -----END PRIVATE KEY-----
            """
        let privateKey = try PrivateKey.fromPem(pemString)

        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10")
    }

    internal func testEd25519FromPemWithPassword() throws {
        let pemString =
            """
            -----BEGIN ENCRYPTED PRIVATE KEY-----
            MIGbMFcGCSqGSIb3DQEFDTBKMCkGCSqGSIb3DQEFDDAcBAjeB6TNNQX+1gICCAAw
            DAYIKoZIhvcNAgkFADAdBglghkgBZQMEAQIEENfMacg1/Txd/LhKkxZtJe0EQEVL
            mez3xb+sfUIF3TKEIDJtw7H0xBNlbAfLxTV11pofiar0z1/WRBHFFUuGIYSiKjlU
            V9RQhAnemO84zcZfTYs=
            -----END ENCRYPTED PRIVATE KEY-----
            """

        let privateKey = try PrivateKey.fromPem(pemString, "test")

        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b6570042204208d8df406a762e36dfbf6dda2239f38a266db369e09bca6a8569e9e79b4826152"
        )
    }

    internal func testEcdsaFromPem() throws {
        let pemString = """
            -----BEGIN PRIVATE KEY-----
            MDACAQAwBwYFK4EEAAoEIgQgh3bGuDGhthrBDawDBKKEPeRxb1SxkZu5GiaF0P4/
            MEg=
            -----END PRIVATE KEY-----
            """

        let privateKey = try PrivateKey.fromPem(pemString)

        XCTAssertEqual(
            privateKey.description,
            "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048")
    }

    internal func testEd25519FromPemInvalidTypeLabel() {
        // extra `S` in the type label
        let pemString = """
            -----BEGIN PRIVATE KEYS-----
            MC4CAQAwBQYDK2VwBCIEINtIS4KOZLLY8SzjwKDpOguMznrxu485yXcyOUSCU44Q
            -----END PRIVATE KEY-----
            """

        XCTAssertThrowsError(try PrivateKey.fromPem(pemString)) { error in
            // we're going to complain very loudly anyway if it's the wrong error type.
            // swiftlint:disable:next force_cast
            XCTAssertEqual((error as! HError).kind, HError.ErrorKind.keyParse)
        }
    }
}
