/// A transaction that can be executed on the Hedera network.
public class Transaction: Request<TransactionResponse> {
    public override func encode(to encoder: Encoder) throws {
        // TODO: encode payment transaction
        // TODO: var container = encoder.container(keyedBy: CodingKeys.self)
    }
}
