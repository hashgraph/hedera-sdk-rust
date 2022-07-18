import Hedera
import SwiftDotenv

@main
public enum Program {
    public static func main() async throws {
        let env = try Dotenv.load()
        let client = Client.forTestnet()

        client.setOperator(env.operatorAccountId, env.operatorKey)

        let newKey = PrivateKey.generateEd25519()

        print("private key = \(newKey)")
        print("public key = \(newKey.publicKey)")

        let response = try await AccountCreateTransaction()
            .key(.single(newKey.publicKey))
            .initialBalance(500_000_000)
            .execute(client)

        let receipt = try await response.getReceipt(client)
        let newAccountId = receipt.accountId!

        print("account address = \(newAccountId)")
    }
}

extension Environment {
    var operatorAccountId: AccountId {
        AccountId(self["OPERATOR_ACCOUNT_ID"]!.stringValue)!
    }

    var operatorKey: PrivateKey {
        PrivateKey(self["OPERATOR_KEY"]!.stringValue)!
    }
}
