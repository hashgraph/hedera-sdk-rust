import Hedera
import SwiftDotenv

@main
public enum Program {
    public static func main() async throws {
        let env = try Dotenv.load()
        let client = Client.forTestnet()

        client.setOperator(env.operatorAccountId, env.operatorKey)

        let response = try await AccountDeleteTransaction()
            .transferAccountId("0.0.6189")
            .deleteAccountId("0.0.34952813")
            .execute(client)

        _ = try await response.getReceipt(client)
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
