import Hedera

@main
public enum Program {
    public static func main() async throws {
        let client = Client.forTestnet()

        let balance = try await AccountBalanceQuery()
            .accountId("0.0.1001")
            .execute(client)

        print("balance = \(balance.hbars)")
    }
}
