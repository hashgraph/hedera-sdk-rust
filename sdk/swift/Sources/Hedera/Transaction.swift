/// A transaction that can be executed on the Hedera network.
public class Transaction: Request<TransactionResponse> {
    private enum CodingKeys: String, CodingKey {
        case type = "$type"
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        let typeName = String(describing: type(of: self))
        let requestName = typeName.prefix(1).lowercased() + typeName.dropFirst().dropLast(11)

        try container.encode(requestName, forKey: .type)
    }
}
