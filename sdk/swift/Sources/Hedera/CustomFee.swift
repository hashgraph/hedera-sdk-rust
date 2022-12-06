import Foundation
import NumberKit

/// A transfer fee to assess during a `TransferTransaction` that transfers units of
/// the token to which the fee is attached.
public protocol CustomFee {
    /// The account to receive the custom fee.
    var feeCollectorAccountId: AccountId? { get set }

    /// Sets the account to recieve the custom fee.
    @discardableResult
    mutating func feeCollectorAccountId(_ feeCollectorAccountId: AccountId) -> Self
}

extension CustomFee {
    public mutating func feeCollectorAccountId(_ feeCollectorAccountId: AccountId) -> Self {
        self.feeCollectorAccountId = feeCollectorAccountId

        return self
    }
}

/// A transfer fee to assess during a `TransferTransaction` that transfers units of
/// the token to which the fee is attached.
public enum AnyCustomFee {
    case fixed(FixedFee)
    case fractional(FractionalFee)
    case royalty(RoyaltyFee)
}

extension AnyCustomFee: Codable {
    enum CodingKeys: String, CodingKey {
        case type = "$type"
    }

    public init(from decoder: Swift.Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        let type = try container.decode(String.self, forKey: .type)

        // note: intentionally use *this* decoder rather than a nested decoder
        switch type {
        case "fixed":
            self = .fixed(try FixedFee(from: decoder))
        case "fractional":
            self = .fractional(try FractionalFee(from: decoder))

        case "royalty":
            self = .royalty(try RoyaltyFee(from: decoder))
        default:
            fatalError("unexpected custom fee kind: \(type)")
        }
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        switch self {
        case .fixed(let fee):
            try container.encode("fixed", forKey: .type)
            try fee.encode(to: encoder)
        case .fractional(let fee):
            try container.encode("fractional", forKey: .type)
            try fee.encode(to: encoder)

        case .royalty(let fee):
            try container.encode("royalty", forKey: .type)
            try fee.encode(to: encoder)
        }
    }
}

extension AnyCustomFee: CustomFee {
    public var feeCollectorAccountId: AccountId? {
        get {
            switch self {
            case .fixed(let fee):
                return fee.feeCollectorAccountId
            case .fractional(let fee):
                return fee.feeCollectorAccountId
            case .royalty(let fee):
                return fee.feeCollectorAccountId
            }

        }
        set(newValue) {
            switch self {
            case .fixed(var fee):
                fee.feeCollectorAccountId = newValue
            case .fractional(var fee):
                fee.feeCollectorAccountId = newValue
            case .royalty(var fee):
                fee.feeCollectorAccountId = newValue
            }
        }
    }
}

/// A fixed number of units (hbar or token) to assess as a fee during a `TransferTransaction` that transfers
/// units of the token to which this fixed fee is attached.
public struct FixedFee: CustomFee, Codable {
    public var feeCollectorAccountId: AccountId?

    /// Create a new `CustomFixedFee`.
    public init(
        amount: UInt64 = 0,
        denominatingTokenId: TokenId? = nil,
        feeCollectorAccountId: AccountId? = nil
    ) {
        self.amount = amount
        self.denominatingTokenId = denominatingTokenId
        self.feeCollectorAccountId = feeCollectorAccountId
    }

    /// The number of units to assess as a fee.
    ///
    /// If the `denominatingTokenId` is unset, this value is in HBAR and must be set in **tinybars**.
    public var amount: UInt64

    /// Sets the number of units to assess as a fee.
    @discardableResult
    public mutating func amount(_ amount: UInt64) -> Self {
        self.amount = amount

        return self
    }

    /// The denomination of the fee.
    ///
    /// Taken as HBAR if left unset.
    /// When used in a `TokenCreateTransaction`, taken as the newly created token ID if set to `0.0.0`.
    public var denominatingTokenId: TokenId?

    /// Sets the denomination of the fee.
    @discardableResult
    public mutating func denominatingTokenId(_ denominatingTokenId: TokenId) -> Self {
        self.denominatingTokenId = denominatingTokenId

        return self
    }
}

/// A fraction of the transferred units of a token to assess as a fee.
///
/// The amount assessed will never be less than the given `minimumAmount`, and never greater
/// than the given `maximumAmount`.
///
/// The denomination is always in units of the token to which this fractional fee is attached.
///
public struct FractionalFee: CustomFee, Codable {
    public var feeCollectorAccountId: AccountId?

    /// Create a new `CustomFixedFee`.
    public init(
        amount: Rational<UInt64> = "1/1",
        minimumAmount: UInt64 = 0,
        maximumAmount: UInt64 = 0,
        netOfTransfers: Bool = false,
        feeCollectorAccountId: AccountId? = nil
    ) {
        self.amount = amount
        self.minimumAmount = minimumAmount
        self.maximumAmount = maximumAmount
        self.netOfTransfers = netOfTransfers
        self.feeCollectorAccountId = feeCollectorAccountId
    }

    /// The fraction of the transferred units to assess as a fee.
    public var amount: Rational<UInt64>

    /// Sets the fraction of the transferred units to assess as a fee.
    @discardableResult
    public mutating func amount(_ amount: Rational<UInt64>) -> Self {
        self.amount = amount

        return self
    }

    /// The minimum amount to assess.
    public var minimumAmount: UInt64

    /// Sets the minimum amount to assess.
    @discardableResult
    public mutating func minimumAmount(_ minimumAmount: UInt64) -> Self {
        self.minimumAmount = minimumAmount

        return self
    }

    /// The maximum amount to assess.
    public var maximumAmount: UInt64

    /// Sets the maximum amount to assess.
    @discardableResult
    public mutating func maximumAmount(_ maximumAmount: UInt64) -> Self {
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
    public var netOfTransfers: Bool

    /// Sets whether the fee assessment should be in addition to the transfer amount or not.
    @discardableResult
    public mutating func netOfTransfers(_ netOfTransfers: Bool) -> Self {
        self.netOfTransfers = netOfTransfers

        return self
    }

    enum CodingKeys: String, CodingKey {
        case amount
        case minimumAmount
        case maximumAmount
        case netOfTransfers
        case feeCollectorAccountId
    }

    public init(from decoder: Swift.Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        amount = Rational(from: try container.decode(String.self, forKey: .amount))!
        minimumAmount = try container.decode(UInt64.self, forKey: .minimumAmount)
        maximumAmount = try container.decode(UInt64.self, forKey: .maximumAmount)
        netOfTransfers = try container.decode(Bool.self, forKey: .netOfTransfers)
        feeCollectorAccountId = try container.decodeIfPresent(AccountId.self, forKey: .feeCollectorAccountId)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(String(describing: amount), forKey: .amount)
        try container.encode(minimumAmount, forKey: .minimumAmount)
        try container.encode(maximumAmount, forKey: .maximumAmount)
        try container.encode(netOfTransfers, forKey: .netOfTransfers)
    }
}

/// A fee to assess during a `TransferTransaction` that changes ownership of an NFT.
///
/// Defines the fraction of the fungible value exchanged for an NFT that the ledger
/// should collect as a royalty.
public struct RoyaltyFee: CustomFee, Codable {
    public var feeCollectorAccountId: AccountId?

    /// Create a new `CustomRoyaltyFee`.
    public init(
        exchangeValue: Rational<UInt64> = "1/1",
        fallbackFee: FixedFee? = nil
    ) {
        self.exchangeValue = exchangeValue
        self.fallbackFee = fallbackFee
    }

    /// The fraction of fungible value exchanged for an NFT to collect as royalty.
    public var exchangeValue: Rational<UInt64>

    /// Sets the fraction of fungible value exchanged for an NFT to collect as royalty.
    @discardableResult
    public mutating func exchangeValue(_ exchangeValue: Rational<UInt64>) -> Self {
        self.exchangeValue = exchangeValue

        return self
    }

    /// If present, the fixed fee to assess to the NFT receiver when no fungible value is exchanged
    /// with the sender.
    public var fallbackFee: FixedFee?

    /// Set the fixed fee to assess to the NFT receiver when no fungible value is exchanged
    /// with the sender.
    @discardableResult
    public mutating func fallbackFee(_ fallbackFee: FixedFee) -> Self {
        self.fallbackFee = fallbackFee

        return self
    }

    enum CodingKeys: String, CodingKey {
        case feeCollectorAccountId
        case exchangeValue
        case fallbackFee
    }

    public init(from decoder: Swift.Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.exchangeValue = Rational(from: try container.decode(String.self, forKey: .exchangeValue))!
        fallbackFee = try container.decodeIfPresent(FixedFee.self, forKey: .fallbackFee)
        feeCollectorAccountId = try container.decodeIfPresent(AccountId.self, forKey: .feeCollectorAccountId)
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(String(describing: self.exchangeValue), forKey: .exchangeValue)
        try container.encodeIfPresent(fallbackFee, forKey: .fallbackFee)
        try container.encodeIfPresent(feeCollectorAccountId, forKey: .feeCollectorAccountId)
    }
}
