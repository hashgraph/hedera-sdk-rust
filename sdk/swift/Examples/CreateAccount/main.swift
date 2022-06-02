import Hedera

@main
public enum Program {
    public static func main() async throws {
        let client = Client.forTestnet()

        client.setPayerAccountId(AccountId(num: 6189))
        client.addDefaultSigner(PrivateKey("7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0")!)

        let newKey = PrivateKey.generateEd25519()

        print("private key = \(newKey)")
        print("public key = \(newKey.publicKey)")

        let response = try await AccountCreateTransaction()
            .key(.single(newKey.publicKey))
            .initialBalance(500_000_000)
            .execute(client)

        let receipt = try await response.getSuccessfulReceipt(client)
        let newAccountId = receipt.accountId!

        print("account address = \(newAccountId)")
    }
}
