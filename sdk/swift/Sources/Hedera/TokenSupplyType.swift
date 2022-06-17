/// Possible token supply types.
/// Can be used to restrict supply to a set maximum.
/// Defaults to `infinite`.
public enum TokenSupplyType: Codable {
    case infinite
    case finite
}
