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

public final class PrivateKeyTests: XCTestCase {
    public func testParseEd25519() throws {
        let privateKey: PrivateKey =
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312"

        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312")
    }

    public func testParseEcdsa() throws {
        let privateKey: PrivateKey =
            "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048"

        XCTAssertEqual(
            privateKey.description,
            "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048")
    }

    public func testEd25519LegacyDerive() throws {
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

    public func testEd25519LegacyDerive2() throws {
        let privateKey: PrivateKey =
            "302e020100300506032b65700422042000c2f59212cb3417f0ee0d38e7bd876810d04f2dd2cb5c2d8f26ff406573f2bd"

        let privateKeyMhw = try privateKey.legacyDerive(0xff_ffff_ffff)

        XCTAssertEqual(
            privateKeyMhw.description,
            "302e020100300506032b6570042204206890dc311754ce9d3fc36bdf83301aa1c8f2556e035a6d0d13c2cccdbbab1242")
    }

    public func testEd25519FromPem() throws {
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

    public func testEcdsaFromPem() throws {
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

    public func testEd25519FromPemInvalidTypeLabel() {
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
