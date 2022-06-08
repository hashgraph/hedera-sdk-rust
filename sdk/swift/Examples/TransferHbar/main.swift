import Foundation
import Hedera

@main
public enum Program {
    public static func main() async throws {
        let client = Client.forTestnet()

        client.setPayerAccountId("0.0.6189")
        client.addDefaultSigner(PrivateKey("7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0")!)

        let transactionResponse = try await TransferTransaction()
            .hbarTransfer(account: "0.0.1001", amount: 20)
            .hbarTransfer(account: "0.0.6189", amount: -20)
            .execute(client)

        // either of these values can be used to lookup transactions in an explorer such as
        //  Kabuto or DragonGlass; the transaction ID is generally more useful as it also contains a rough
        //  estimation of when the transaction was created (+/- 8 seconds) and the account that paid for
        //  transaction
        print("transaction id: \(transactionResponse.transactionId)")
        print("transaction hash: \(transactionResponse.transactionHash)")
    }
}
