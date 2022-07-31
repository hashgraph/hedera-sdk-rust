/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
public final class TransferTransaction: Transaction {
    private var transfers: [Transfer] = []
    private var tokenTransfers: [TokenTransfer] = []

    /// Create a new `TransferTransaction`.
    public override init() {
    }

    /// Add a non-approved hbar transfer to the transaction.
    @discardableResult
    public func hbarTransfer(_ accountId: AccountId, _ amount: Int64) -> Self {
        doHbarTransfer(accountId, amount, false)
    }

    /// Add an approved hbar transfer to the transaction.
    @discardableResult
    public func approvedHbarTransfer(_ accountId: AccountId, _ amount: Int64) -> Self {
        doHbarTransfer(accountId, amount, true)
    }

    /// Add a non-approved token transfer to the transaction.
    @discardableResult
    public func tokenTransfer(_ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64) -> Self {
        doTokenTransfer(tokenId, accountId, amount, false, nil)
    }

    /// Add an approved token transfer to the transaction.
    @discardableResult
    public func approvedTokenTransfer(_ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64) -> Self {
        doTokenTransfer(tokenId, accountId, amount, true, nil)
    }

    /// Add a non-approved token transfer with decimals to the transaction.
    @discardableResult
    public func tokenTransferWithDecimals(
        _ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64, _ expectedDecimals: UInt32
    ) -> Self {
        doTokenTransfer(tokenId, accountId, amount, false, expectedDecimals)
    }

    /// Add an approved token transfer with decimals to the transaction.
    @discardableResult
    public func approvedTokenTransferWithDecimals(
        _ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64, _ expectedDecimals: UInt32
    ) -> Self {
        doTokenTransfer(tokenId, accountId, amount, false, expectedDecimals)
    }

    /// Add a non-approved nft transfer to the transaction.
    @discardableResult
    public func nftTransfer(_ nftId: NftId, _ senderAccountId: AccountId, _ receiverAccountId: AccountId)
        -> Self
    {
        doNftTransfer(nftId, senderAccountId, receiverAccountId, false)
    }

    /// Add an approved nft transfer to the transaction.
    @discardableResult
    public func approvedNftTransfer(
        _ nftId: NftId, _ senderAccountId: AccountId, _ receiverAccountId: AccountId
    ) -> Self {
        doNftTransfer(nftId, senderAccountId, receiverAccountId, true)
    }

    private func doHbarTransfer(
        _ accountId: AccountId,
        _ amount: Int64,
        _ approved: Bool
    ) -> Self {
        transfers.append(Transfer(accountId: accountId, amount: amount, isApproval: approved))

        return self
    }

    private func doTokenTransfer(
        _ tokenId: TokenId,
        _ accountId: AccountId,
        _ amount: Int64,
        _ approved: Bool,
        _ expectedDecimals: UInt32?
    ) -> Self {
        let transfer = Transfer(accountId: accountId, amount: amount, isApproval: approved)

        if var tt = tokenTransfers.first(where: { (tt) in tt.tokenId == tokenId }) {
            tt.expectedDecimals = expectedDecimals
            tt.transfers.append(transfer)
        } else {
            tokenTransfers.append(
                TokenTransfer(
                    tokenId: tokenId,
                    transfers: [transfer],
                    nftTransfers: [],
                    expectedDecimals: expectedDecimals
                ))
        }

        return self
    }

    private func doNftTransfer(
        _ nftId: NftId,
        _ senderAccountId: AccountId,
        _ receiverAccountId: AccountId,
        _ approved: Bool
    ) -> Self {
        let transfer = NftTransfer(
            senderAccountId: senderAccountId,
            receiverAccountId: receiverAccountId,
            serialNumber: nftId.serialNumber,
            isApproval: approved
        )

        if var tt = tokenTransfers.first(where: { (tt) in tt.tokenId == nftId.tokenId }) {
            tt.nftTransfers.append(transfer)
        } else {
            tokenTransfers.append(
                TokenTransfer(
                    tokenId: nftId.tokenId,
                    transfers: [],
                    nftTransfers: [transfer],
                    expectedDecimals: nil
                ))
        }

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case transfers
        case tokenTransfers
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(transfers, forKey: .transfers)
        try container.encode(tokenTransfers, forKey: .tokenTransfers)

        try super.encode(to: encoder)
    }
}

private struct Transfer: Encodable {
    let accountId: AccountId
    let amount: Int64
    let isApproval: Bool
}

private struct TokenTransfer: Encodable {
    let tokenId: TokenId
    var transfers: [Transfer]
    var nftTransfers: [NftTransfer]
    var expectedDecimals: UInt32?
}

private struct NftTransfer: Encodable {
    let senderAccountId: AccountId
    let receiverAccountId: AccountId
    let serialNumber: UInt64
    let isApproval: Bool
}
