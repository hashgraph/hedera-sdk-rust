import XCTest

@testable import Hedera

internal final class TransactionIdTests: XCTestCase {
    internal func testFromStringWrongField() {
        XCTAssertNil(TransactionId.init("0.0.31415?1641088801.2"))
    }

    internal func testFromStringWrongField2() {
        XCTAssertNil(TransactionId.init("0.0.31415/1641088801.2"))
    }

    internal func testFromStringOutOfOrder() {
        XCTAssertNil(TransactionId.init("0.0.31415?scheduled/1412@1641088801.2"))
    }

    internal func testFromStringSingleDigitNanos() throws {
        let validStart = Timestamp(fromUnixTimestampNanos: 1_641_088_801 * 1_000_000_000 + 2)

        let expected: TransactionId = TransactionId(
            accountId: "0.0.31415",
            validStart: validStart,
            scheduled: false,
            nonce: nil
        )

        XCTAssertEqual("0.0.31415@1641088801.2", expected)
    }

    internal func testToStringSingleDigitNanos() throws {
        let validStart = Timestamp(fromUnixTimestampNanos: 1_641_088_801 * 1_000_000_000 + 2)

        let transactionId: TransactionId = TransactionId(
            accountId: "0.0.31415",
            validStart: validStart,
            scheduled: false,
            nonce: nil
        )

        XCTAssertEqual(transactionId.description, "0.0.31415@1641088801.2")
    }
}
