import XCTest

@testable import Hedera

public final class PublicKeyTests: XCTestCase {
    public func testParseEd25519() throws {
        let publicKey: PublicKey =
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7"

        XCTAssertEqual(
            publicKey.description,
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7")
    }

    public func testParseEcdsa() throws {
        let publicKey: PublicKey =
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7"

        XCTAssertEqual(
            publicKey.description,
            "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7")
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

    public func testEd25519ParseVariants() throws {
        try publicKeyParseVariants(
            key: "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7"
        )
    }

    public func testEcdsaParseVariants() throws {
        try publicKeyParseVariants(
            key: "302f300906072a8648ce3d020103220002703a9370b0443be6ae7c507b0aec81a55e94e4a863b9655360bd65358caa6588"
        )
    }
}
