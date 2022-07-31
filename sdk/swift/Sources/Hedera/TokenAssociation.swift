/// A token <-> account association.
public struct TokenAssociation: Codable {
    /// The token involved in the association.
    public let tokenId: TokenId

    /// The account involved in the association.
    public let accountId: AccountId
}
