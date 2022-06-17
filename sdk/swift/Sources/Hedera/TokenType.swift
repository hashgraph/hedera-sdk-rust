/// Possible token types.
///
/// Apart from fungible and non-fungible, tokens can have either a common or
/// unique representation.
///
/// Only `fungibleCommon` and `nonFungibleUnique` are supported right now. More
/// may be added in the future.
///
public enum TokenType: Codable {
    case fungibleCommon
    case nonFungibleUnique
}
