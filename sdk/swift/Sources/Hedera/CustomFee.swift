import NumberKit

/// Holder for any concrete `CustomFee` for use in decoding.
public enum AnyCustomFee: Codable {
    case fixed(CustomFixedFee)
    case fractional(CustomFractionalFee)
    case royalty(CustomRoyaltyFee)

    public init(_ fee: CustomFee) {
        switch fee {
        case let fixedFee as CustomFixedFee:
            self = .fixed(fixedFee)

        case let fractionalFee as CustomFractionalFee:
            self = .fractional(fractionalFee)

        case let royaltyFee as CustomRoyaltyFee:
            self = .royalty(royaltyFee)

        default:
            fatalError("(BUG) unreachable: unexpected derived class of CustomFee")
        }
    }

    public init(_ fixed: CustomFixedFee) {
        self = .fixed(fixed)
    }

    public init(_ fractional: CustomFractionalFee) {
        self = .fractional(fractional)
    }

    public init(_ royalty: CustomRoyaltyFee) {
        self = .royalty(royalty)
    }

    private enum CodingKeys: String, CodingKey {
        case type = "$type"
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        let type = try container.decode(String.self, forKey: .type)

        switch type {
        case "fixed":
            self = .fixed(try CustomFixedFee(from: decoder))

        case "royalty":
            self = .royalty(try CustomRoyaltyFee(from: decoder))

        case "fractional":
            self = .fractional(try CustomFractionalFee(from: decoder))

        default:
            fatalError("unexpected variant for CustomFee: \(type)")
        }
    }

    public func encode(to encoder: Encoder) throws {
        switch self {
        case .fixed(let fee):
            try fee.encode(to: encoder)

        case .fractional(let fee):
            try fee.encode(to: encoder)

        case .royalty(let fee):
            try fee.encode(to: encoder)
        }
    }
}

/// A transfer fee to assess during a `TransferTransaction` that transfers units of
/// the token to which the fee is attached.
public class CustomFee: Encodable {
    /// Create a new `CustomFee`.
    internal init(
        feeCollectorAccountId: AccountAddress? = nil
    ) {
        self.feeCollectorAccountId = feeCollectorAccountId
    }

    internal init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        feeCollectorAccountId = try container.decode(AccountAddress.self, forKey: .feeCollectorAccountId)
    }

    /// The account to receive the custom fee.
    public var feeCollectorAccountId: AccountAddress?

    /// Sets the account to receive the custom fee.
    @discardableResult
    public func feeCollectorAccountId(_ feeCollectorAccountId: AccountAddress) -> Self {
        self.feeCollectorAccountId = feeCollectorAccountId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case type = "$type"
        case feeCollectorAccountId
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        let typeName = String(describing: type(of: self))
        let customFeeName = String(typeName.lowercased().dropFirst(5).dropLast(3))

        try container.encode(customFeeName, forKey: .type)
        try container.encodeIfPresent(feeCollectorAccountId, forKey: .feeCollectorAccountId)
    }
}

/// A fixed number of units (hbar or token) to assess as a fee during a `TransferTransaction` that transfers
/// units of the token to which this fixed fee is attached.
public final class CustomFixedFee: CustomFee, Decodable {
    /// Create a new `CustomFixedFee`.
    public init(
        amount: UInt64 = 0,
        denominatingTokenId: TokenId? = nil,
        feeCollectorAccountId: AccountAddress? = nil
    ) {
        self.amount = amount
        self.denominatingTokenId = denominatingTokenId

        super.init(feeCollectorAccountId: feeCollectorAccountId)
    }

    public override init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        amount = try container.decode(UInt64.self, forKey: .amount)
        denominatingTokenId = try container.decodeIfPresent(TokenId.self, forKey: .denominatingTokenId)

        try super.init(from: decoder)
    }

    /// The number of units to assess as a fee.
    ///
    /// If the `denominatingTokenId` is unset, this value is in HBAR and must be set in **tinybars**.
    ///
    public var amount: UInt64

    /// Sets the number of units to assess as a fee.
    @discardableResult
    public func amount(_ amount: UInt64) -> Self {
        self.amount = amount

        return self
    }

    /// The denomination of the fee.
    ///
    /// Taken as HBAR if left unset.
    /// When used in a `TokenCreateTransaction`, taken as the newly created token ID if set to `0.0.0`.
    ///
    public var denominatingTokenId: TokenId?

    /// Sets the denomination of the fee.
    @discardableResult
    public func denominatingTokenId(_ denominatingTokenId: TokenId) -> Self {
        self.denominatingTokenId = denominatingTokenId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case amount
        case denominatingTokenId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(amount, forKey: .amount)
        try container.encodeIfPresent(denominatingTokenId, forKey: .denominatingTokenId)

        try super.encode(to: encoder)
    }
}

/// A fraction of the transferred units of a token to assess as a fee.
///
/// The amount assessed will never be less than the given `minimumAmount`, and never greater
/// than the given `maximumAmount`.
///
/// The denomination is always in units of the token to which this fractional fee is attached.
///
public final class CustomFractionalFee: CustomFee, Decodable {
    /// Create a new `CustomFractionalFee`.
    public init(
        amount: Rational<UInt64> = "1/1",
        minimumAmount: UInt64 = 0,
        maximumAmount: UInt64 = 0,
        netOfTransfers: Bool = false,
        feeCollectorAccountId: AccountAddress? = nil
    ) {
        self.amount = amount
        self.minimumAmount = minimumAmount
        self.maximumAmount = maximumAmount
        self.netOfTransfers = netOfTransfers

        super.init(feeCollectorAccountId: feeCollectorAccountId)
    }

    public override init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        amount = try container.decode(Rational<UInt64>.self, forKey: .amount)
        minimumAmount = try container.decode(UInt64.self, forKey: .minimumAmount)
        maximumAmount = try container.decode(UInt64.self, forKey: .maximumAmount)
        netOfTransfers = try container.decode(Bool.self, forKey: .netOfTransfers)

        try super.init(from: decoder)
    }

    /// The fraction of the transferred units to assess as a fee.
    public var amount: Rational<UInt64>

    /// Sets the fraction of the transferred units to assess as a fee.
    @discardableResult
    public func amount(_ amount: Rational<UInt64>) -> Self {
        self.amount = amount

        return self
    }

    /// The minimum amount to assess.
    public var minimumAmount: UInt64

    /// Sets the minimum amount to assess.
    @discardableResult
    public func minimumAmount(_ minimumAmount: UInt64) -> Self {
        self.minimumAmount = minimumAmount

        return self
    }

    /// The maximum amount to assess.
    public var maximumAmount: UInt64

    /// Sets the maximum amount to assess.
    @discardableResult
    public func maximumAmount(_ maximumAmount: UInt64) -> Self {
        self.maximumAmount = maximumAmount

        return self
    }

    /// Whether the fee assessment should be in addition to the transfer amount or not.
    ///
    /// If true, assesses the fee to the sender, so the receiver gets the full amount from the token
    /// transfer list, and the sender is charged an additional fee.
    ///
    /// If false, the receiver does NOT get the full amount, but only what is left over after
    /// paying the fractional fee.
    ///
    public var netOfTransfers: Bool

    /// Sets whether the fee assessment should be in addition to the transfer amount or not.
    @discardableResult
    public func netOfTransfers(_ netOfTransfers: Bool) -> Self {
        self.netOfTransfers = netOfTransfers

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case amount
        case minimumAmount
        case maximumAmount
        case netOfTransfers
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(amount.description, forKey: .amount)
        try container.encode(minimumAmount, forKey: .minimumAmount)
        try container.encode(maximumAmount, forKey: .maximumAmount)
        try container.encode(netOfTransfers, forKey: .netOfTransfers)

        try super.encode(to: encoder)
    }
}

/// A fee to assess during a `TransferTransaction` that changes ownership of an NFT.
///
/// Defines the fraction of the fungible value exchanged for an NFT that the ledger
/// should collect as a royalty.
///
public final class CustomRoyaltyFee: CustomFee, Decodable {
    /// Create a new `CustomRoyaltyFee`.
    public init(
        exchangeValue: Rational<UInt64> = "1/1",
        fallbackAmount: UInt64? = nil,
        fallbackDenominatingTokenId: TokenId? = nil,
        feeCollectorAccountId: AccountAddress? = nil
    ) {
        self.exchangeValue = exchangeValue
        self.fallbackAmount = fallbackAmount
        self.fallbackDenominatingTokenId = fallbackDenominatingTokenId

        super.init(feeCollectorAccountId: feeCollectorAccountId)
    }

    public override init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        exchangeValue = try container.decode(Rational<UInt64>.self, forKey: .exchangeValue)
        fallbackAmount = try container.decodeIfPresent(UInt64.self, forKey: .fallbackAmount)
        fallbackDenominatingTokenId = try container.decodeIfPresent(TokenId.self, forKey: .fallbackDenominatingTokenId)

        try super.init(from: decoder)
    }

    /// The fraction of fungible value exchanged for an NFT to collect as royalty.
    public var exchangeValue: Rational<UInt64>

    /// If present, the fixed fee amount to assess to the NFT receiver when no fungible value is exchanged
    /// with the sender.
    public var fallbackAmount: UInt64?

    /// The denomination of the fallback fee amount.
    public var fallbackDenominatingTokenId: TokenId?

    private enum CodingKeys: String, CodingKey {
        case exchangeValue
        case fallbackAmount
        case fallbackDenominatingTokenId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(exchangeValue.description, forKey: .exchangeValue)
        try container.encodeIfPresent(fallbackAmount, forKey: .fallbackAmount)
        try container.encodeIfPresent(fallbackDenominatingTokenId, forKey: .fallbackDenominatingTokenId)

        try super.encode(to: encoder)
    }
}
