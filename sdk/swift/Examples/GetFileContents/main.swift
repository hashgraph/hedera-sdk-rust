import Hedera
import SwiftDotenv

@main
public enum Program {
    public static func main() async throws {
        let env = try Dotenv.load()
        let client = Client.forTestnet()

        client.setOperator(env.operatorAccountId, env.operatorKey)

        let response = try await FileContentsQuery()
            .fileId("0.0.34945328")
            .execute(client)

        let text = String(data: response.contents, encoding: .utf8)!

        print("contents = \(text)")
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
