import Hedera

@main
struct Program {
    static func main() async throws {
        let client = Client.forTestnet()

        client.setPayerAccountId(AccountId(num: 6189))
        client.addDefaultSigner(PrivateKey("7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0")!)

        let id = AccountId(num: 1001)

        let info = try await AccountInfoQuery()
            .accountId(id)
            .execute(client)

        print("balance = \(info.balance)")
    }
}
