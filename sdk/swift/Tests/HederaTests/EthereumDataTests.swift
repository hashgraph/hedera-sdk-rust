/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

private let rawTxType0: Data = Data(
    hexEncoded:
        "f864012f83018000947e3a9eaf9bcc39e2ffa38eb30bf7a93feacbc181808276"
        + "53820277a0f9fbff985d374be4a55f296915002eec11ac96f1ce2df183adf992"
        + "baa9390b2fa00c1e867cc960d9c74ec2e6a662b7908ec4c8cc9f3091e886bcef"
        + "beb2290fb792"
)!

private let rawTxType2: Data = Data(
    hexEncoded:
        "02f87082012a022f2f83018000947e3a9eaf9bcc39e2ffa38eb30bf7a93feacb"
        + "c181880de0b6b3a764000083123456c001a0df48f2efd10421811de2bfb125ab"
        + "75b2d3c44139c4642837fb1fccce911fd479a01aaf7ae92bee896651dfc9d99a"
        + "e422a296bf5d9f1ca49b2d96d82b79eb112d66"
)!

public final class EthereumDataTests: XCTestCase {
    internal func testLegacyFromBytes() throws {
        let data = try EthereumData(rlpBytes: rawTxType0)

        guard case .legacy(let data) = data else {
            XCTFail("expected legacy ethereum data")
            return
        }

        XCTAssertEqual("01", data.nonce.hexStringEncoded())
        XCTAssertEqual("2f", data.gasPrice.hexStringEncoded())
        XCTAssertEqual("018000", data.gasLimit.hexStringEncoded())
        XCTAssertEqual("7e3a9eaf9bcc39e2ffa38eb30bf7a93feacbc181", data.to.hexStringEncoded())
        XCTAssertEqual("", data.value.hexStringEncoded())
        XCTAssertEqual("0277", data.v.hexStringEncoded())
        XCTAssertEqual("7653", data.callData.hexStringEncoded())
        XCTAssertEqual("f9fbff985d374be4a55f296915002eec11ac96f1ce2df183adf992baa9390b2f", data.r.hexStringEncoded())
        XCTAssertEqual("0c1e867cc960d9c74ec2e6a662b7908ec4c8cc9f3091e886bcefbeb2290fb792", data.s.hexStringEncoded())
    }

    internal func testLegacyRoundtrip() throws {
        let data = try EthereumData(rlpBytes: rawTxType0)

        XCTAssertEqual(rawTxType0.hexStringEncoded(), data.toBytes().hexStringEncoded())
    }

    internal func testEip1559FromBytes() throws {
        let data = try EthereumData(rlpBytes: rawTxType2)

        guard case .eip1559(let data) = data else {
            XCTFail("expected eip1559 ethereum data")
            return
        }

        XCTAssertEqual("012a", data.chainId.hexStringEncoded())
        XCTAssertEqual("02", data.nonce.hexStringEncoded())
        XCTAssertEqual("2f", data.maxPriorityGas.hexStringEncoded())
        XCTAssertEqual("2f", data.maxGas.hexStringEncoded())
        XCTAssertEqual("018000", data.gasLimit.hexStringEncoded())
        XCTAssertEqual("7e3a9eaf9bcc39e2ffa38eb30bf7a93feacbc181", data.to.hexStringEncoded())
        XCTAssertEqual("0de0b6b3a7640000", data.value.hexStringEncoded())
        XCTAssertEqual("123456", data.callData.hexStringEncoded())
        XCTAssertEqual([], data.accessList.map { $0.hexStringEncoded() })
        XCTAssertEqual("01", data.recoveryId.hexStringEncoded())
        XCTAssertEqual("df48f2efd10421811de2bfb125ab75b2d3c44139c4642837fb1fccce911fd479", data.r.hexStringEncoded())
        XCTAssertEqual("1aaf7ae92bee896651dfc9d99ae422a296bf5d9f1ca49b2d96d82b79eb112d66", data.s.hexStringEncoded())
    }

    internal func testEip1559Roundtrip() throws {
        let data = try EthereumData(rlpBytes: rawTxType2)

        XCTAssertEqual(rawTxType2.hexStringEncoded(), data.toBytes().hexStringEncoded())
    }
}
