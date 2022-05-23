/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
public final class TransferTransaction: Transaction {
    private var hbarTransfers: [HbarTransfer] = []

    public override init() {
    }

    @discardableResult
    public func hbarTransfer(account: AccountIdOrAlias, amount: Int64) -> Self {
        hbarTransfers.append(HbarTransfer(account: account, amount: amount))

        return self
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        var transfer = container.nestedContainer(keyedBy: TransferKeys.self, forKey: .transfer)

        try transfer.encode(hbarTransfers, forKey: .tinybarTransfers)

        try super.encode(to: encoder)
    }
}

private struct HbarTransfer: Encodable {
    let account: AccountIdOrAlias
    let amount: Int64
}

private enum CodingKeys: String, CodingKey {
    case transfer
}

private enum TransferKeys: String, CodingKey {
    case tinybarTransfers
    // TODO: case tokenTransfers
    // TODO: case nftTransfers
}
