import Hedera
import SwiftDotenv

@main
public enum Program {
    public static func main() async throws {
        let env = try Dotenv.load()
        let client = Client.forTestnet()

        client.setOperator(env.operatorAccountId, env.operatorKey)

        let id = AccountId(num: 1001)

        let info = try await AccountInfoQuery()
            .accountId(id)
            .execute(client)

        print("balance = \(info.balance)")
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
