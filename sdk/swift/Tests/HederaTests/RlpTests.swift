import Foundation
import XCTest

@testable import Hedera

// Frustratingly, there is no usable Rlp library for swift, so we had to write our own (Internal use only)
// hence:
// adapted from https://github.com/ethereum/tests/blob/d25a79ae508daeb60bee0bf819ac7e884fc494d7/RLPTests/rlptest.json
// which is licensed under the MIT license.
private struct Test {
    fileprivate init(_ a: Input, _ b: Output) {
        self.a = a
        self.b = b
    }

    fileprivate enum Input: ExpressibleByArrayLiteral, ExpressibleByStringLiteral, Equatable {
        init(arrayLiteral elements: Self...) {
            self = .array(elements)
        }

        init(stringLiteral value: String) {
            self = .value(value)
        }

        case array([Self])
        case value(String)
    }

    fileprivate struct Output: ExpressibleByStringLiteral {
        fileprivate let data: Data
        fileprivate init(stringLiteral value: String) {
            data = Data(hexEncoded: value.stripPrefix("0x") ?? value[...])!
        }
    }

    fileprivate let a: Input
    fileprivate let b: Output
}

extension Test.Input: RlpDecodable, RlpEncodable {
    fileprivate init(rlp: AnyRlp) throws {
        if rlp.isList {
            self = .array(try .init(rlp: rlp))
        } else {
            self = .value(String(decoding: try Data(rlp: rlp), as: UTF8.self))
        }
    }

    func encode(to encoder: inout Rlp.Encoder) {
        switch self {
        case .value(let value):
            encoder.append(value.data(using: .utf8)!)
        case .array(let list):
            encoder.append(list)
        }
    }

}

private enum RlpTestCases: Hashable {
    fileprivate static let emptyString = Test("", "0x80")
    fileprivate static let bytestring00 = Test("\u{0000}", "0x00")
    fileprivate static let bytestring01 = Test("\u{0001}", "0x01")
    fileprivate static let bytestring7F = Test("\u{007F}", "0x7f")
    fileprivate static let shortstring = Test("dog", "0x83646f67")
    fileprivate static let shortstring2 = Test(
        "Lorem ipsum dolor sit amet, consectetur adipisicing eli",
        "0xb74c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e7365637465747572206164697069736963696e6720656c69"
    )

    fileprivate static let longstring = Test(
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit",
        "0xb8384c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e7365637465747572206164697069736963696e6720656c6974"
    )

    fileprivate static let longstring2 = Test(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur mauris magna, suscipit sed vehicula non, iaculis faucibus tortor. Proin suscipit ultricies malesuada. Duis tortor elit, dictum quis tristique eu, ultrices at risus. Morbi a est imperdiet mi ullamcorper aliquet suscipit nec lorem. Aenean quis leo mollis, vulputate elit varius, consequat enim. Nulla ultrices turpis justo, et posuere urna consectetur nec. Proin non convallis metus. Donec tempor ipsum in mauris congue sollicitudin. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Suspendisse convallis sem vel massa faucibus, eget lacinia lacus tempor. Nulla quis ultricies purus. Proin auctor rhoncus nibh condimentum mollis. Aliquam consequat enim at metus luctus, a eleifend purus egestas. Curabitur at nibh metus. Nam bibendum, neque at auctor tristique, lorem libero aliquet arcu, non interdum tellus lectus sit amet eros. Cras rhoncus, metus ac ornare cursus, dolor justo ultrices metus, at ullamcorper volutpat",
        "0xb904004c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742e20437572616269747572206d6175726973206d61676e612c20737573636970697420736564207665686963756c61206e6f6e2c20696163756c697320666175636962757320746f72746f722e2050726f696e20737573636970697420756c74726963696573206d616c6573756164612e204475697320746f72746f7220656c69742c2064696374756d2071756973207472697374697175652065752c20756c7472696365732061742072697375732e204d6f72626920612065737420696d70657264696574206d6920756c6c616d636f7270657220616c6971756574207375736369706974206e6563206c6f72656d2e2041656e65616e2071756973206c656f206d6f6c6c69732c2076756c70757461746520656c6974207661726975732c20636f6e73657175617420656e696d2e204e756c6c6120756c74726963657320747572706973206a7573746f2c20657420706f73756572652075726e6120636f6e7365637465747572206e65632e2050726f696e206e6f6e20636f6e76616c6c6973206d657475732e20446f6e65632074656d706f7220697073756d20696e206d617572697320636f6e67756520736f6c6c696369747564696e2e20566573746962756c756d20616e746520697073756d207072696d697320696e206661756369627573206f726369206c756374757320657420756c74726963657320706f737565726520637562696c69612043757261653b2053757370656e646973736520636f6e76616c6c69732073656d2076656c206d617373612066617563696275732c2065676574206c6163696e6961206c616375732074656d706f722e204e756c6c61207175697320756c747269636965732070757275732e2050726f696e20617563746f722072686f6e637573206e69626820636f6e64696d656e74756d206d6f6c6c69732e20416c697175616d20636f6e73657175617420656e696d206174206d65747573206c75637475732c206120656c656966656e6420707572757320656765737461732e20437572616269747572206174206e696268206d657475732e204e616d20626962656e64756d2c206e6571756520617420617563746f72207472697374697175652c206c6f72656d206c696265726f20616c697175657420617263752c206e6f6e20696e74657264756d2074656c6c7573206c65637475732073697420616d65742065726f732e20437261732072686f6e6375732c206d65747573206163206f726e617265206375727375732c20646f6c6f72206a7573746f20756c747269636573206d657475732c20617420756c6c616d636f7270657220766f6c7574706174"
    )
    // no integer support currently.
    // "zero": Test(_, ""),
    // "smallint": Test(_, ""),
    // "smallint2": Test(_, ""),
    // "smallint3": Test(_, ""),
    // "smallint4": Test(_, ""),
    // "mediumint1": Test(_, ""),
    // "mediumint2": Test(_, ""),
    // "mediumint3": Test(_, ""),
    // "mediumint4": Test(_, ""),
    // "mediumint5": Test(_, ""),
    fileprivate static let emptylist = Test([], "0xc0")
    fileprivate static let stringlist = Test(["dog", "god", "cat"], "0xcc83646f6783676f6483636174")
    // no integer support currently.
    // "multilist": Test(_, ""),
    fileprivate static let shortListMax1 = Test(
        ["asdf", "qwer", "zxcv", "asdf", "qwer", "zxcv", "asdf", "qwer", "zxcv", "asdf", "qwer"],
        "0xf784617364668471776572847a78637684617364668471776572847a78637684617364668471776572847a78637684617364668471776572"
    )

    fileprivate static let longList1 = Test(
        [
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
        ],

        "0xf840cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376"
    )

    fileprivate static let longList2 = Test(
        [
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
            ["asdf", "qwer", "zxcv"],
        ],

        "0xf90200cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376cf84617364668471776572847a786376"
    )
    fileprivate static let listsoflists = Test([[[], []], []], "0xc4c2c0c0c0")
    fileprivate static let listsoflists2 = Test([[], [[]], [[], [[]]]], "0xc7c0c1c0c3c0c1c0")
    fileprivate static let dictTest1 = Test(
        [
            ["key1", "val1"],
            ["key2", "val2"],
            ["key3", "val3"],
            ["key4", "val4"],
        ],
        "0xecca846b6579318476616c31ca846b6579328476616c32ca846b6579338476616c33ca846b6579348476616c34"
    )
    // no integer support
    // "bigint": Test(_, ""),
}

public final class RlpTests: XCTestCase {
    private static func decode(_ input: Data) throws -> Test.Input {
        return try .init(rlp: AnyRlp(raw: input))
    }

    private func encodeTest(_ test: Test) {
        XCTAssertEqual(test.a.rlpEncoded().hexStringEncoded(), test.b.data.hexStringEncoded())
    }

    private func decodeTest(_ test: Test) throws {
        let decoded = try Self.decode(test.b.data)

        XCTAssertEqual(decoded, test.a)
    }

    internal func testEncodeEmptyString() {
        encodeTest(RlpTestCases.emptyString)
    }

    internal func testEncodeByteString00() {
        encodeTest(RlpTestCases.bytestring00)
    }

    internal func testEncodeByteString01() {
        encodeTest(RlpTestCases.bytestring01)
    }

    internal func testEncodeByteString7F() {
        encodeTest(RlpTestCases.bytestring7F)
    }

    internal func testEncodeShortString() {
        encodeTest(RlpTestCases.shortstring)
    }

    internal func testEncodeShortString2() {
        encodeTest(RlpTestCases.shortstring2)
    }

    internal func testEncodeLongString() {
        encodeTest(RlpTestCases.longstring)
    }

    internal func testEncodeLongString2() {
        encodeTest(RlpTestCases.longstring2)
    }

    internal func testEncodeEmptyList() {
        encodeTest(RlpTestCases.emptylist)
    }

    internal func testEncodeStringList() {
        encodeTest(RlpTestCases.stringlist)
    }

    internal func testEncodeShortListMax1() {
        encodeTest(RlpTestCases.shortListMax1)
    }

    internal func testEncodeLongList1() {
        encodeTest(RlpTestCases.longList1)
    }

    internal func testEncodeLongList2() {
        encodeTest(RlpTestCases.longList2)
    }

    internal func testEncodeListsOfLists() {
        encodeTest(RlpTestCases.listsoflists)
    }

    internal func testEncodeListsOfLists2() {
        encodeTest(RlpTestCases.listsoflists2)
    }

    internal func testEncodeDictTest1() {
        encodeTest(RlpTestCases.dictTest1)
    }

    internal func testDecodeEmptyString() throws {
        try decodeTest(RlpTestCases.emptyString)
    }

    internal func testDecodeByteString00() throws {
        try decodeTest(RlpTestCases.bytestring00)
    }

    internal func testDecodeByteString01() throws {
        try decodeTest(RlpTestCases.bytestring01)
    }

    internal func testDecodeByteString7F() throws {
        try decodeTest(RlpTestCases.bytestring7F)
    }

    internal func testDecodeShortString() throws {
        try decodeTest(RlpTestCases.shortstring)
    }

    internal func testDecodeShortString2() throws {
        try decodeTest(RlpTestCases.shortstring2)
    }

    internal func testDecodeLongString() throws {
        try decodeTest(RlpTestCases.longstring)
    }

    internal func testDecodeLongString2() throws {
        try decodeTest(RlpTestCases.longstring2)
    }

    internal func testDecodeEmptyList() throws {
        try decodeTest(RlpTestCases.emptylist)
    }

    internal func testDecodeStringList() throws {
        try decodeTest(RlpTestCases.stringlist)
    }

    internal func testDecodeShortListMax1() throws {
        try decodeTest(RlpTestCases.shortListMax1)
    }

    internal func testDecodeLongList1() throws {
        try decodeTest(RlpTestCases.longList1)
    }

    internal func testDecodeLongList2() throws {
        try decodeTest(RlpTestCases.longList2)
    }

    internal func testDecodeListsOfLists() throws {
        try decodeTest(RlpTestCases.listsoflists)
    }

    internal func testDecodeListsOfLists2() throws {
        try decodeTest(RlpTestCases.listsoflists2)
    }

    internal func testDecodeDictTest1() throws {
        try decodeTest(RlpTestCases.dictTest1)
    }
}
