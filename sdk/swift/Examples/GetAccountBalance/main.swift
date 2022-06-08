import Hedera

@main
public enum Program {
    public static func main() async throws {
        let client = Client.forTestnet()

        let response = try await AccountBalanceQuery()
            .accountId("0.0.1001")
            .execute(client)

        print("balance = \(response.balance)")
    }
}
