import Hedera

@main
struct Program {
    static func main() async throws {
        // Generate a Ed25519 key
        // This is the current recommended default for Hedera

        var keyPrivate = PrivateKey.generateEd25519()
        var keyPublic = keyPrivate.publicKey

        print("ed25519 private = \(keyPrivate)")
        print("ed25519 public = \(keyPublic)")

        // Generate a ECDSA(secp256k1) key
        // This is recommended for better compatibility with Ethereum

        keyPrivate = PrivateKey.generateEcdsaSecp256k1()
        keyPublic = keyPrivate.publicKey

        print("ecdsa(secp256k1) private = \(keyPrivate)")
        print("ecdsa(secp256k1) public = \(keyPublic)")
    }
}
