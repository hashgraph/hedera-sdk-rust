import Hedera

@main
public enum Program {
    public static func main() async throws {
        let client = Client.forTestnet()

        client.setPayerAccountId(AccountId(num: 6189))
        client.addDefaultSigner(PrivateKey("7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0")!)

        let response = try await FileContentsQuery()
            .fileId("0.0.34945328")
            .execute(client)

        let text = String(data: response.contents, encoding: .utf8)!

        print("contents = \(text)")
    }
}
