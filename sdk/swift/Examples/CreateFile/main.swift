import Hedera
import SwiftDotenv

@main
public enum Program {
    public static func main() async throws {
        let env = try Dotenv.load()

        // todo: network from name
        let client = Client.forTestnet()

        // Defaults the operator account ID and key such that all generated transactions will be paid for
        // by this account and be signed by this key
        client.setOperator(env.operatorAccountId, env.operatorKey)

        // The file is required to be a byte array,
        // you can easily use the bytes of a file instead.
        let fileContents = "Hedera hashgraph is great!"

        let response = try await FileCreateTransaction()
            .keys([.single(env.operatorKey.getPublicKey())])
            .contents(fileContents.data(using: .utf8)!)
            .maxTransactionFee(2)
            .execute(client)

        let receipt = try await response.getReceipt(client)

        print("file: \(String(describing: receipt.fileId))")
    }
}

extension Environment {
    public var operatorAccountId: AccountId {
        AccountId(self["OPERATOR_ACCOUNT_ID"]!.stringValue)!
    }

    public var operatorKey: PrivateKey {
        PrivateKey(self["OPERATOR_KEY"]!.stringValue)!
    }
}
