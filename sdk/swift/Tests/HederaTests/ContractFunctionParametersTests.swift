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

import NumberKit
import XCTest

@testable import Hedera

internal final class ContractFunctionParametersTests: XCTestCase {
    // swiftlint:disable:next function_body_length
    internal func testMiscArgs() {
        let result = ContractFunctionParameters()
            .addUint8(0xfa)
            .addInt8(0x2c)
            .addInt8(-71)
            .addUint32(0xc57e_0ebb)
            .addInt32(-623_939_995)
            .addUint64(0x23da_a8fc_a9b4_fa73)
            .addInt64(-0x5)
            .addUint256(
                BigInt(
                    from: "6b38e4c1ba0c2a8a05c599df7894f1d5e99411932ec8e2b473464b79a9d69aff".uppercased(),
                    base: BigInt.hexBase)!
            )
            .addInt256(BigInt(-0x7))
            .addUint8Array([1, 1, 2, 3, 5, 8])
            .addInt8Array([-1, 1, -2])
            .addUint32Array([0xca00_c819, 0x2aa2_3a28, 0x8a00_f0fb, 0x7752_d5c3])
            .addInt32Array([-0xd, 0xe, 0xf, -0x10])
            .addUint64Array([0x11, 0x12, 0x13, 0x14])
            .addInt64Array([-0x15, 0x16, 0x17, -0x18])
            .addUint256Array([0xc0ffee, 0xdeca_ff00])
            .addInt256Array([BigInt(-0x1a)])
            .addBytes32(Data(hexEncoded: "e0561423151ef9ea3b4befec265a7fbc9dfd75fe2cbdfbded979e7846eedfc9e")!)
            .addBytes32Array([
                Data(hexEncoded: "e0561423151ef9ea3b4befec265a7fbc9dfd75fe2cbdfbded979e7846eedfc9e")!,
                Data(hexEncoded: "acd1dcf8a5c85e531e9c5330f041b0154df8aaff892d1b6d9b1605674b03fca6")!,
                Data(hexEncoded: "5684ddf4a1bf500b1b23edc66d0fc05b453a3f2e65618af0a4e64fe83904eede")!,
            ])
            .toBytes("foo")

        XCTAssertEqual(
            result.hexStringEncoded(),
            """
            ccd0cb21\
            00000000000000000000000000000000000000000000000000000000000000fa\
            000000000000000000000000000000000000000000000000000000000000002c\
            ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb9\
            00000000000000000000000000000000000000000000000000000000c57e0ebb\
            ffffffffffffffffffffffffffffffffffffffffffffffffffffffffdacf6e65\
            00000000000000000000000000000000000000000000000023daa8fca9b4fa73\
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb\
            6b38e4c1ba0c2a8a05c599df7894f1d5e99411932ec8e2b473464b79a9d69aff\
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9\
            0000000000000000000000000000000000000000000000000000000000000260\
            0000000000000000000000000000000000000000000000000000000000000340\
            00000000000000000000000000000000000000000000000000000000000003c0\
            0000000000000000000000000000000000000000000000000000000000000460\
            0000000000000000000000000000000000000000000000000000000000000500\
            00000000000000000000000000000000000000000000000000000000000005a0\
            0000000000000000000000000000000000000000000000000000000000000640\
            00000000000000000000000000000000000000000000000000000000000006a0\
            e0561423151ef9ea3b4befec265a7fbc9dfd75fe2cbdfbded979e7846eedfc9e\
            00000000000000000000000000000000000000000000000000000000000006e0\
            0000000000000000000000000000000000000000000000000000000000000006\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000002\
            0000000000000000000000000000000000000000000000000000000000000003\
            0000000000000000000000000000000000000000000000000000000000000005\
            0000000000000000000000000000000000000000000000000000000000000008\
            0000000000000000000000000000000000000000000000000000000000000003\
            ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
            0000000000000000000000000000000000000000000000000000000000000001\
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe\
            0000000000000000000000000000000000000000000000000000000000000004\
            00000000000000000000000000000000000000000000000000000000ca00c819\
            000000000000000000000000000000000000000000000000000000002aa23a28\
            000000000000000000000000000000000000000000000000000000008a00f0fb\
            000000000000000000000000000000000000000000000000000000007752d5c3\
            0000000000000000000000000000000000000000000000000000000000000004\
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff3\
            000000000000000000000000000000000000000000000000000000000000000e\
            000000000000000000000000000000000000000000000000000000000000000f\
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0\
            0000000000000000000000000000000000000000000000000000000000000004\
            0000000000000000000000000000000000000000000000000000000000000011\
            0000000000000000000000000000000000000000000000000000000000000012\
            0000000000000000000000000000000000000000000000000000000000000013\
            0000000000000000000000000000000000000000000000000000000000000014\
            0000000000000000000000000000000000000000000000000000000000000004\
            ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeb\
            0000000000000000000000000000000000000000000000000000000000000016\
            0000000000000000000000000000000000000000000000000000000000000017\
            ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8\
            0000000000000000000000000000000000000000000000000000000000000002\
            0000000000000000000000000000000000000000000000000000000000c0ffee\
            00000000000000000000000000000000000000000000000000000000decaff00\
            0000000000000000000000000000000000000000000000000000000000000001\
            ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe6\
            0000000000000000000000000000000000000000000000000000000000000003\
            e0561423151ef9ea3b4befec265a7fbc9dfd75fe2cbdfbded979e7846eedfc9e\
            acd1dcf8a5c85e531e9c5330f041b0154df8aaff892d1b6d9b1605674b03fca6\
            5684ddf4a1bf500b1b23edc66d0fc05b453a3f2e65618af0a4e64fe83904eede
            """
        )
    }

    internal func testAddressParams() {
        let result = ContractFunctionParameters()
            .addAddress("0x4a17b15b0cb6bbaed6863ec4b876f67963784082")
            .addAddress("0x26b78c151e57b95db43a8a66787d1a8e3e618b55")
            .addAddressArray(
                [
                    "355dbd7f82623cf3b9bdaf93904465c337475d8d",
                    "0x33ca2a5913b59f7004e4a4940c4d307898ee6181",
                ]
            )
            .toBytes("foo")

        XCTAssertEqual(
            result.hexStringEncoded(),
            """
            7d48c86d\
            0000000000000000000000004a17b15b0cb6bbaed6863ec4b876f67963784082\
            00000000000000000000000026b78c151e57b95db43a8a66787d1a8e3e618b55\
            0000000000000000000000000000000000000000000000000000000000000060\
            0000000000000000000000000000000000000000000000000000000000000002\
            000000000000000000000000355dbd7f82623cf3b9bdaf93904465c337475d8d\
            00000000000000000000000033ca2a5913b59f7004e4a4940c4d307898ee6181
            """
        )
    }

    internal func testFunctionParams() {
        let result = ContractFunctionParameters()
            .addFunction("3a29e764b2a8ed36cff67b613fa086db02fdd1b7", Data([1, 2, 3, 4]))
            .addFunction("3a29e764b2a8ed36cff67b613fa086db02fdd1b7", ContractFunctionSelector(Data([1, 2, 3, 4])))
            .addFunction(
                "0x56de35467b2a4e5f219058ad0d3a66688ca12d3b", ContractFunctionSelector("randomFunction").addBool()
            )
            .toBytes("foo")

        XCTAssertEqual(
            result.hexStringEncoded(),
            """
            e78b3940\
            3a29e764b2a8ed36cff67b613fa086db02fdd1b7010203040000000000000000\
            3a29e764b2a8ed36cff67b613fa086db02fdd1b7010203040000000000000000\
            56de35467b2a4e5f219058ad0d3a66688ca12d3b63441d820000000000000000
            """
        )
    }
}
