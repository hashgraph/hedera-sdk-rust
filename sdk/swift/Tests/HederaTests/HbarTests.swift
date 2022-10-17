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
import XCTest
@testable import Hedera

final class HbarTests: XCTestCase {
    func testInit() throws {
        let fifty: Hbar = 50

        XCTAssertEqual(fifty, Hbar(50))
        XCTAssertEqual(fifty, Hbar(50.0))
        XCTAssertEqual(fifty, 50.0)

        XCTAssertEqual(fifty, "50")
        XCTAssertEqual(fifty, "50.0")
        XCTAssertEqual(fifty, Hbar("50"))
        XCTAssertEqual(fifty, Hbar("50.0"))

        XCTAssertEqual(fifty, try Hbar.from(50))
        XCTAssertEqual(fifty, try Hbar.from(50.0))
        XCTAssertEqual(fifty, try Hbar.fromString("50"))
        XCTAssertEqual(fifty, try Hbar.fromString("50.0"))
        XCTAssertEqual(fifty, Hbar.fromTinybars(5_000_000_000))
    }

    func testFractionalTinybarThrowsError() {
        // todo: test the exact error.
        XCTAssertThrowsError(try Hbar(0.1, .tinybar))
    }

    func testNanHbarThrowsError() {
        // todo: test the exact error.
        XCTAssertThrowsError(try Hbar(Decimal.quietNaN))
    }

    func testInitUnit() throws {
        let fifty_tinybar: Hbar = 0.0000005

        XCTAssertEqual(fifty_tinybar, try Hbar(50, .tinybar))
        XCTAssertEqual(fifty_tinybar, try Hbar(50.0, .tinybar))
        XCTAssertEqual(fifty_tinybar, try Hbar(0.5, .microbar))
        XCTAssertEqual(fifty_tinybar, try Hbar(5e-4, .millibar))
        XCTAssertEqual(fifty_tinybar, try Hbar(5e-7, .hbar))
        XCTAssertEqual(fifty_tinybar, try Hbar(5e-10, .kilobar))
        XCTAssertEqual(fifty_tinybar, "50 tℏ")
        XCTAssertEqual(fifty_tinybar, "50.0 tℏ")
        XCTAssertEqual(fifty_tinybar, "0.5 µℏ")
        XCTAssertEqual(fifty_tinybar, "0.0005 mℏ")
        XCTAssertEqual(fifty_tinybar, "0.0000005 ℏ")
        XCTAssertEqual(fifty_tinybar, "0.0000000005 kℏ")
        XCTAssertEqual(fifty_tinybar, "0.0000000000005 Mℏ")
        XCTAssertEqual(fifty_tinybar, "0.0000000000000005 Gℏ")

        XCTAssertEqual(fifty_tinybar, try Hbar.from(50, .tinybar))
        XCTAssertEqual(fifty_tinybar, try Hbar.from(50.0, .tinybar))
        XCTAssertEqual(fifty_tinybar, try Hbar.from(0.5, .microbar))
        XCTAssertEqual(fifty_tinybar, try Hbar.from(5e-4, .millibar))
        XCTAssertEqual(fifty_tinybar, try Hbar.from(5e-7, .hbar))
        XCTAssertEqual(fifty_tinybar, try Hbar.from(5e-10, .kilobar))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("50 tℏ"))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("50.0 tℏ"))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("0.5 µℏ"))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("0.0005 mℏ"))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("0.0000005 ℏ"))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("0.0000000005 kℏ"))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("0.0000000000005 Mℏ"))
        XCTAssertEqual(fifty_tinybar, try Hbar.fromString("0.0000000000000005 Gℏ"))
    }

    func testTo() {
        let twenty_two_kilobars: Hbar = 22_000

        XCTAssertEqual(twenty_two_kilobars.getValue(), 22_000)
        XCTAssertEqual(twenty_two_kilobars.to(.tinybar), 2_200_000_000_000)
        XCTAssertEqual(twenty_two_kilobars.to(.microbar), 22_000_000_000)
        XCTAssertEqual(twenty_two_kilobars.to(.millibar), 22_000_000)
        XCTAssertEqual(twenty_two_kilobars.to(.hbar), 22_000)
        XCTAssertEqual(twenty_two_kilobars.to(.kilobar), 22)
        XCTAssertEqual(twenty_two_kilobars.to(.megabar), Decimal(string: "0.022"))
        XCTAssertEqual(twenty_two_kilobars.to(.gigabar), Decimal(string: "0.000022"))
    }

    func testNegated() {
        XCTAssertEqual(Hbar(2).negated(), -2)
    }

    // what better way to ensure the right thing gets printed than to test that for all values of <inner range>.
    // it isn't practical to test all ~2^64 values `Hbar` can hold.
    // In fact, this test test's less than 1% of 1% of 1%... of all values.
    func testDescription() {
        let innerRange = -9999...9999
        for i in innerRange {
            let hbar = Hbar.fromTinybars(Int64(i))
            let expected = "\(i) tℏ"
            XCTAssertEqual(hbar.toString(), expected)
            XCTAssertEqual(hbar.description, expected)
        }

        for i in -20000...20_000 {
            guard !innerRange.contains(i) else { continue }
            let hbar = Hbar.fromTinybars(Int64(i))

            let expected = "\(hbar.to(.hbar)) ℏ"
            XCTAssertEqual(hbar.toString(), expected)
            XCTAssertEqual(hbar.description, expected)
        }
    }

    func testToStringWithUnit() {
        let fifty: Hbar = 50

        XCTAssertEqual(fifty.toString(.tinybar), "5000000000 tℏ")
        XCTAssertEqual(fifty.toString(.microbar), "50000000 µℏ")
        XCTAssertEqual(fifty.toString(.millibar), "50000 mℏ")
        XCTAssertEqual(fifty.toString(.hbar), "50 ℏ")
        XCTAssertEqual(fifty.toString(.kilobar), "0.05 kℏ")
        XCTAssertEqual(fifty.toString(.megabar), "0.00005 Mℏ")
        XCTAssertEqual(fifty.toString(.gigabar), "0.00000005 Gℏ")
    }

    func testEncodingHasNoUnit() throws {
       let encoded = String(data: try JSONEncoder().encode(Hbar(21)), encoding: .utf8)!

        XCTAssertEqual(encoded, "2100000000")
    }
}
