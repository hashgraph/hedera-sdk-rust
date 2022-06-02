import Hedera

@main
public enum Program {
    public static func main() async throws {
        let client = Client.forTestnet()

        let id = AccountId(num: 1001)

        let response = try await AccountBalanceQuery()
            .accountId(id)
            .execute(client)

        print("balance = \(response.balance)")
    }
}
