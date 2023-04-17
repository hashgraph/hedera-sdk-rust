import XCTest

@testable import Hedera

internal final class PublicKeyTests: XCTestCase {
    internal func testParseEd25519() throws {
        let publicKey: PublicKey =
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7"

        XCTAssertEqual(
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7",
            publicKey.description
        )
    }

    internal func testParseEcdsa() throws {
        let publicKey: PublicKey =
            "302d300706052b8104000a03220002703a9370b0443be6ae7c507b0aec81a55e94e4a863b9655360bd65358caa6588"

        XCTAssertEqual(
            "302d300706052b8104000a03220002703a9370b0443be6ae7c507b0aec81a55e94e4a863b9655360bd65358caa6588",
            publicKey.description
        )
    }

    private func publicKeyParseVariants(key: String) throws {
        for variant in 0..<4 {
            let prefix = variant & 1 == 1
            let uppercase = (variant >> 1) & 1 == 1
            let prefixStr = prefix ? "0x" : ""
            let keyCased = uppercase ? key.uppercased() : key.lowercased()
            let publicKey = try PublicKey.fromString("\(prefixStr)\(keyCased)")

            XCTAssertEqual(key, publicKey.description)
        }
    }

    internal func testEd25519ParseVariants() throws {
        try publicKeyParseVariants(
            key: "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7"
        )
    }

    internal func testEcdsaParseVariants() throws {
        try publicKeyParseVariants(
            key: "302d300706052b8104000a03220002703a9370b0443be6ae7c507b0aec81a55e94e4a863b9655360bd65358caa6588"
        )
    }
}
