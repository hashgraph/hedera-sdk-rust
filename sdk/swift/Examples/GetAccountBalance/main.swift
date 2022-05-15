import Hedera

@main
struct Program {
    static func main() async throws {
        let client = Client.forTestnet()

        let id = AccountId(num: 10012)

        let ab = try await AccountBalanceQuery()
            .accountId(id)
            .execute(client)

        print("balance = \(ab.balance)")
    }
}
