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

    internal func testToEvmAddress() throws {
        let publicKey = try PrivateKey.fromStringEcdsa(
            "debae3ca62ab3157110dba79c8de26540dc320ee9be73a77d70ba175643a3500"
        ).publicKey

        let evmAddress = publicKey.toEvmAddress()

        XCTAssertEqual(evmAddress, "0xd8eb8db03c699faa3f47adcdcd2ae91773b10f8b")
    }

    internal func testToEvmAddress2() throws {
        let publicKey = try PublicKey.fromStringEcdsa(
            "029469a657510f3bf199a0e29b21e11e7039d8883f3547d59c3568f9c89f704cbc")

        let evmAddress = publicKey.toEvmAddress()

        XCTAssertEqual(evmAddress, "0xbbaa6bdfe888ae1fc8e7c8cee82081fa79ba8834")
    }

    internal func testEd25519Verify() throws {
        let publicKey = try PublicKey.fromStringDer(
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7"
        )

        let signature = Data(
            hexEncoded: "9d04bfed7baa97c80d29a6ae48c0d896ce8463a7ea0c16197d55a563c73996ef"
                + "062b2adf507f416c108422c0310fc6fb21886e11ce3de3e951d7a56049743f07"
        )!

        XCTAssertNoThrow(try publicKey.verify("hello, world".data(using: .utf8)!, signature))
    }

    internal func testEcdsaVerify() throws {
        let publicKey = try PublicKey.fromStringDer(
            "302d300706052b8104000a03220002703a9370b0443be6ae7c507b0aec81a55e94e4a863b9655360bd65358caa6588"
        )

        let signature = Data(
            hexEncoded: "f3a13a555f1f8cd6532716b8f388bd4e9d8ed0b252743e923114c0c6cbfe414c"
                + "086e3717a6502c3edff6130d34df252fb94b6f662d0cd27e2110903320563851"
        )!

        XCTAssertNoThrow(try publicKey.verify("hello world".data(using: .utf8)!, signature))
    }

    /// We should error if we try to verify a signature with the wrong public key (or the right public key with the wrong signature, same thing)
    ///
    /// This in particular tests attempting to verify a ecdsa-secp256k1 signature for an ed25519 public key.
    internal func testEd25519VerifyErrorEcdsa() throws {
        let publicKey = try PublicKey.fromStringDer(
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7"
        )

        let signature = Data(
            hexEncoded: "f3a13a555f1f8cd6532716b8f388bd4e9d8ed0b252743e923114c0c6cbfe414c"
                + "086e3717a6502c3edff6130d34df252fb94b6f662d0cd27e2110903320563851"
        )!

        XCTAssertThrowsError(try publicKey.verify("hello, world".data(using: .utf8)!, signature))
    }

    /// We should error if we try to verify a signature with the wrong public key (or the right public key with the wrong signature, same thing)
    ///
    /// This in particular tests attempting to verify a ed25519 signature for an ecdsa-secp256k1 public key.
    internal func testEcdsaVerifyErrorEd25519() throws {
        let publicKey = try PublicKey.fromStringDer(
            "302d300706052b8104000a03220002703a9370b0443be6ae7c507b0aec81a55e94e4a863b9655360bd65358caa6588"
        )

        let signature = Data(
            hexEncoded: "9d04bfed7baa97c80d29a6ae48c0d896ce8463a7ea0c16197d55a563c73996ef"
                + "062b2adf507f416c108422c0310fc6fb21886e11ce3de3e951d7a56049743f07"
        )!

        XCTAssertThrowsError(try publicKey.verify("hello world".data(using: .utf8)!, signature))
    }
}
