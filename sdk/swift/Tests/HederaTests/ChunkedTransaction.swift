import XCTest

@testable import Hedera

internal final class ChunkedTransactionTests: XCTestCase {
    internal func testToFromBytes() throws {
        let client = Client.forTestnet()
        client.setOperator(0, .generateEd25519())

        let transactionId = TransactionId(accountId: 0, validStart: Timestamp(from: Date()), scheduled: false)

        let bytes = try TopicMessageSubmitTransaction()
            .topicId(314)
            .message("Hello, world!".data(using: .utf8)!)
            .chunkSize(8)
            .maxChunks(2)
            .transactionId(transactionId)
            .freezeWith(client)
            .toBytes()

        let transaction = try Transaction.fromBytes(bytes)

        guard let transaction = transaction as? TopicMessageSubmitTransaction else {
            XCTFail("Transaction wasn't a TopicMessageSubmitTransaction (it was actually \(type(of: transaction))")
            return
        }

        XCTAssertEqual(transaction.topicId, 314)
        XCTAssertEqual(transaction.message, "Hello, world!".data(using: .utf8)!)
        XCTAssertEqual(transaction.transactionId, transactionId)

    }
}
