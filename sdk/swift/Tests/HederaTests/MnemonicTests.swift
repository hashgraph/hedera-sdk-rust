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

private let knownGoodMnemonics: [String] = [
    "inmate flip alley wear offer often piece magnet surge toddler submit right radio absent pear floor belt raven "
        + "price stove replace reduce plate home",
    "tiny denial casual grass skull spare awkward indoor ethics dash enough flavor good daughter early "
        + "hard rug staff capable swallow raise flavor empty angle",
    "ramp april job flavor surround pyramid fish sea good know blame gate village viable include mixed term "
        + "draft among monitor swear swing novel track",
    "evoke rich bicycle fire promote climb zero squeeze little spoil slight damage",
]

public final class MnemonicTests: XCTestCase {
    public func testParse() throws {
        for mnemonic in knownGoodMnemonics {
            XCTAssertEqual(try Mnemonic.fromString(mnemonic).description, mnemonic)
        }
    }

    public func testMnemonic3() throws {
        let str =
            "obvious favorite remain caution " + "remove laptop base vacant " + "increase video erase pass "
            + "sniff sausage knock grid " + "argue salt romance way " + "alone fever slush dune"

        let mnemonic = try Mnemonic.fromString(str)

        let privateKey = try mnemonic.toLegacyPrivateKey()

        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312"
        )
    }

    public func testLegacyMnemonic() throws {
        let str =
            "jolly kidnap tom lawn drunk chick optic lust mutter mole bride "
            + "galley dense member sage neural widow decide curb aboard margin manure"

        let mnemonic = try Mnemonic.fromString(str)
        let privateKey = try mnemonic.toLegacyPrivateKey()

        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b65700422042000c2f59212cb3417f0ee0d38e7bd876810d04f2dd2cb5c2d8f26ff406573f2bd"
        )
    }

    public func testToPrivateKey() throws {
        let str =
            "inmate flip alley wear offer often " + "piece magnet surge toddler submit right "
            + "radio absent pear floor belt raven " + "price stove replace reduce plate home"

        let mnemonic = try Mnemonic.fromString(str)

        let key = try mnemonic.toPrivateKey()

        XCTAssertEqual(
            key.description,
            "302e020100300506032b657004220420853f15aecd22706b105da1d709b4ac05b4906170c2b9c7495dff9af49e1391da"
        )
    }
}
