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

public final class ContractFunctionSelectorTests: XCTestCase {
    func testMiscParams() {
        let result = ContractFunctionSelector("foo")
            .addUint8()
            .addInt8()
            .addUint32()
            .addInt32()
            .addUint64()
            .addInt64()
            .addUint256()
            .addInt256()
            .addUint8Array()
            .addInt8Array()
            .addUint32Array()
            .addInt32Array()
            .addUint64Array()
            .addInt64Array()
            .addUint256Array()
            .addInt256Array()
            .finish()

        XCTAssertEqual(result.hexStringEncoded(), "11bcd903")
    }

    func testAddress() {
        let result = ContractFunctionSelector("foo").addAddress().addAddress().addAddressArray().finish()

        XCTAssertEqual(result.hexStringEncoded(), "7d48c86d")
    }

    func testFunction() {
        let result = ContractFunctionSelector("foo").addFunction().addFunction().finish()

        XCTAssertEqual(result.hexStringEncoded(), "c99c40cd")
    }
}
