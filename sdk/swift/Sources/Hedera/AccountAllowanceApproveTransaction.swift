/// Creates one or more hbar/token approved allowances **relative to the owner account specified in the allowances of
/// this transaction**.
///
/// Each allowance grants a spender the right to transfer a pre-determined amount of the owner's
/// hbar/token to any other account of the spender's choice. If the owner is not specified in any
/// allowance, the payer of transaction is considered to be the owner for that particular allowance.
///
/// Setting the amount to zero will remove the respective allowance for the spender.
///
public final class AccountAllowanceApproveTransaction: Transaction {
    private var hbarAllowances: [HbarAllowance] = []
    private var tokenAllowances: [TokenAllowance] = []
    private var nftAllowances: [NftAllowance] = []

    /// Create a new `AccountAllowanceApproveTransaction`.
    public override init() {
    }

    /// Approves the hbar allowance.
    @discardableResult
    public func approveHbarAllowance(
        _ ownerAccountId: AccountId,
        _ spenderAccountId: AccountId,
        _ amount: UInt64
    ) -> Self {
        hbarAllowances.append(
            HbarAllowance(
                ownerAccountId: ownerAccountId,
                spenderAccountId: spenderAccountId,
                amount: amount))

        return self
    }

    /// Approves the token allowance.
    @discardableResult
    public func approveTokenAllowance(
        _ tokenId: TokenId,
        _ ownerAccountId: AccountId,
        _ spenderAccountId: AccountId,
        _ amount: UInt64
    ) -> Self {
        tokenAllowances.append(
            TokenAllowance(
                tokenId: tokenId,
                ownerAccountId: ownerAccountId,
                spenderAccountId: spenderAccountId,
                amount: amount))

        return self
    }

    /// Approves the token NFT allowance.
    @discardableResult
    public func approveTokenNftAllowance(
        _ nftId: NftId,
        _ ownerAccountId: AccountId,
        _ spenderAccountId: AccountId
    ) -> Self {
        if var allowance = nftAllowances.first(where: { (allowance) in
            allowance.tokenId == nftId.tokenId && allowance.spenderAccountId == spenderAccountId
                && allowance.ownerAccountId == ownerAccountId && allowance.approvedForAll == nil
        }) {
            allowance.serialNumbers.append(nftId.serialNumber)
        } else {
            nftAllowances.append(
                NftAllowance(
                    tokenId: nftId.tokenId,
                    ownerAccountId: ownerAccountId,
                    spenderAccountId: spenderAccountId,
                    serialNumbers: [nftId.serialNumber],
                    approvedForAll: nil,
                    delegatingSpenderAccountId: nil
                ))
        }

        return self
    }

    /// Approve the NFT allowance on all serial numbers (present and future).
    @discardableResult
    public func approveTokenNftAllowanceAllSerials(
        _ tokenId: TokenId,
        _ ownerAccountId: AccountId,
        _ spenderAccountId: AccountId
    ) -> Self {

        nftAllowances.append(
            NftAllowance(
                tokenId: tokenId,
                ownerAccountId: ownerAccountId,
                spenderAccountId: spenderAccountId,
                serialNumbers: [],
                approvedForAll: true,
                delegatingSpenderAccountId: nil
            ))

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case hbarAllowances
        case tokenAllowances
        case nftAllowances
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(hbarAllowances, forKey: .hbarAllowances)
        try container.encode(tokenAllowances, forKey: .tokenAllowances)
        try container.encode(nftAllowances, forKey: .nftAllowances)

        try super.encode(to: encoder)
    }
}

private struct HbarAllowance: Codable {
    let ownerAccountId: AccountId
    let spenderAccountId: AccountId
    let amount: UInt64
}

private struct TokenAllowance: Codable {
    let tokenId: TokenId
    let ownerAccountId: AccountId
    let spenderAccountId: AccountId
    let amount: UInt64
}

private struct NftAllowance: Codable {
    let tokenId: TokenId
    let ownerAccountId: AccountId
    let spenderAccountId: AccountId
    var serialNumbers: [UInt64]
    let approvedForAll: Bool?
    let delegatingSpenderAccountId: AccountId?
}
