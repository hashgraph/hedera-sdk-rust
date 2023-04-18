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

internal final class MnemonicTests: XCTestCase {
    internal func testParse() throws {
        for mnemonic in knownGoodMnemonics {
            XCTAssertEqual(try Mnemonic.fromString(mnemonic).description, mnemonic)
        }
    }

    internal func testInvalidLengthError() {
        // we can't test for up to `usize` length, but we can test several lengths to be modestly sure.
        // it might seem that testing many lengths would be slow.
        // we test:

        // todo: this feels overengineered.
        // every length up to (and including `DENSE_LIMIT`).
        // arbitrarily chosen to be 48.

        let denseLimit = 48
        let denseLengths = Array(0...denseLimit)
        let lengths = denseLengths + Array((0...10).lazy.map { $0 * 12 }.drop { $0 < denseLimit })

        for length in lengths.lazy.filter({ ![12, 22, 24].contains($0) }) {
            // this is a word that's explicitly in the word list,
            // to ensure we aren't accidentally testing that this error happens before "word(s) not in list"

            let words = Array(repeating: "apple", count: length)

            XCTAssertThrowsError(try Mnemonic.fromWords(words: words)) { error in
                guard let error = error as? HError,
                    case .mnemonicParse(let reason, _) = error.kind
                else {
                    XCTFail("Unexpected error: \(error)")
                    return
                }

                XCTAssertEqual(reason, .badLength(length))
            }
        }
    }

    internal func testUnknownWords1() {
        let mnemonic = "obvious favorite remain caution remove laptop base vacant alone fever slush dune"

        for index in 0..<12 {
            var words = mnemonic.split(separator: " ").map(String.init)

            words[index] = "lorum"

            XCTAssertThrowsError(try Mnemonic.fromWords(words: words)) { error in
                guard let error = error as? HError,
                    case .mnemonicParse(let reason, _) = error.kind
                else {
                    XCTFail("Unexpected error: \(error)")
                    return
                }

                XCTAssertEqual(reason, .unknownWords([index]))
            }
        }
    }

    internal func testUnknownWords2() {
        // a 24 word mnemonic containing the following typos:
        // absorb -> adsorb
        // account -> acount
        // acquire -> acquired
        let mnemonic =
            "abandon ability able about above absent adsorb abstract absurd abuse access accident "
            + "acount accuse achieve acid acoustic acquired across act action actor actress actual"

        XCTAssertThrowsError(try Mnemonic.fromString(mnemonic)) { error in
            guard let error = error as? HError,
                case .mnemonicParse(let reason, _) = error.kind
            else {
                XCTFail("Unexpected error: \(error)")
                return
            }

            XCTAssertEqual(reason, .unknownWords([6, 12, 17]))
        }
    }

    internal func testChecksumMismatch1() {
        let mnemonic =
            "abandon ability able about above absent absorb abstract absurd abuse access accident "
            + "account accuse achieve acid acoustic acquire across act action actor actress actual"

        XCTAssertThrowsError(try Mnemonic.fromString(mnemonic)) { error in
            guard let error = error as? HError,
                case .mnemonicParse(let reason, _) = error.kind
            else {
                XCTFail("Unexpected error: \(error)")
                return
            }

            XCTAssertEqual(reason, .checksumMismatch(expected: 0xba, actual: 0x17))
        }
    }

    internal func testChecksumMismatch2() {
        let mnemonic = "abandon ability able about above absent absorb abstract absurd abuse access accident"

        XCTAssertThrowsError(try Mnemonic.fromString(mnemonic)) { error in
            guard let error = error as? HError,
                case .mnemonicParse(let reason, _) = error.kind
            else {
                XCTFail("Unexpected error: \(error)")
                return
            }

            XCTAssertEqual(reason, .checksumMismatch(expected: 0x10, actual: 0xb0))
        }
    }

    internal func testFromEntropy() throws {
        let entropy = [
            Data(hexEncoded: "744b201a7c399733691c2fda5c6f605ceb0c016882cb14f64ea9eb5b6d68298b")!,
            Data(hexEncoded: "e2674c8eb2fcada0c433984da6f52bac56466f914b49bd1a8087ed8b12b15248")!,
            Data(hexEncoded: "b1615de02c5da95e15ee0f646f7c5cb02f41e69c9c71df683c1fc78db9b825c7")!,
            Data(hexEncoded: "4e172857ab9ac2563fee9c829a4b2e9b")!,
        ]

        for (entropy, string) in zip(entropy, knownGoodMnemonics) {
            let mnemonic = Mnemonic.fromEntropyForTesting(entropy: entropy)

            XCTAssertEqual(String(describing: mnemonic), string)
        }
    }

    internal func testMnemonic3() throws {
        let str =
            "obvious favorite remain caution remove laptop base vacant increase video erase pass "
            + "sniff sausage knock grid argue salt romance way alone fever slush dune"

        let mnemonic = try Mnemonic.fromString(str)

        let privateKey = try mnemonic.toLegacyPrivateKey()

        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312"
        )
    }

    internal func testLegacyMnemonic() throws {
        let str =
            "jolly kidnap tom lawn drunk chick optic lust mutter mole bride "
            + "galley dense member sage neural widow decide curb aboard margin manure"

        let mnemonic = try Mnemonic.fromString(str)
        let privateKey = try mnemonic.toLegacyPrivateKey()

        // skip the derives and just test the key.
        // (bugs in `legacy_derive` shouldn't make this function fail.)
        XCTAssertEqual(
            privateKey.description,
            "302e020100300506032b65700422042000c2f59212cb3417f0ee0d38e7bd876810d04f2dd2cb5c2d8f26ff406573f2bd"
        )
    }

    internal func testToPrivateKey() throws {
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
