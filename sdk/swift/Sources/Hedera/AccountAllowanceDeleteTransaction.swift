/// Deletes one or more non-fungible approved allowances from an owner's account. This operation
/// will remove the allowances granted to one or more specific non-fungible token serial numbers. Each owner account
/// listed as wiping an allowance must sign the transaction. Hbar and fungible token allowances
/// can be removed by setting the amount to zero in `AccountAllowanceApproveTransaction`.
///
public final class AccountAllowanceDeleteTransaction: Transaction {
    private var nftAllowances: [NftRemoveAllowance] = []

    /// Create a new `AccountAllowanceDeleteTransaction`.
    public override init() {
    }

    /// Remove all nft token allowances.
    @discardableResult
    public func deleteAllTokenNftAllowances(_ nftId: NftId, _ ownerAccountId: AccountId) -> Self {
        if var allowance = nftAllowances.first(where: { (allowance) in
            allowance.tokenId == nftId.tokenId && allowance.ownerAccountId == ownerAccountId
        }) {
            allowance.serialNumbers.append(nftId.serialNumber)
        } else {
            nftAllowances.append(
                NftRemoveAllowance(
                    tokenId: nftId.tokenId,
                    ownerAccountId: ownerAccountId,
                    serialNumbers: [nftId.serialNumber]
                ))
        }

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case nftAllowances
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(nftAllowances, forKey: .nftAllowances)

        try super.encode(to: encoder)
    }
}

private struct NftRemoveAllowance: Encodable {
    let tokenId: TokenId
    let ownerAccountId: AccountId
    var serialNumbers: [UInt64]
}
