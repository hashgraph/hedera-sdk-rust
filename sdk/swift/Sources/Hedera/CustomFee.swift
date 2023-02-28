import Foundation
import NumberKit

// swiftlint:disable file_length

/// A transfer fee to assess during a `TransferTransaction` that transfers units of
/// the token to which the fee is attached.
public protocol CustomFee {
    /// The account to receive the custom fee.
    var feeCollectorAccountId: AccountId? { get set }

    /// True if all collectors are exempt from fees, false otherwise.
    var allCollectorsAreExempt: Bool { get set }

    /// Sets the account to recieve the custom fee.
    @discardableResult
    mutating func feeCollectorAccountId(_ feeCollectorAccountId: AccountId) -> Self

    /// Set to `true` if all collectors should be exempt from fees, or to false otherwise.
    @discardableResult
    mutating func allCollectorsAreExempt(_ allCollectorsAreExempt: Bool) -> Self
}

extension CustomFee {
    public mutating func feeCollectorAccountId(_ feeCollectorAccountId: AccountId) -> Self {
        self.feeCollectorAccountId = feeCollectorAccountId

        return self
    }

    public mutating func allCollectorsAreExempt(_ allCollectorsAreExempt: Bool) -> Self {
        self.allCollectorsAreExempt = true

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
    private enum CodingKeys: String, CodingKey {
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

    public var allCollectorsAreExempt: Bool {
        get {
            switch self {
            case .fixed(let fee):
                return fee.allCollectorsAreExempt
            case .fractional(let fee):
                return fee.allCollectorsAreExempt
            case .royalty(let fee):
                return fee.allCollectorsAreExempt
            }

        }
        set(newValue) {
            switch self {
            case .fixed(var fee):
                fee.allCollectorsAreExempt = newValue
            case .fractional(var fee):
                fee.allCollectorsAreExempt = newValue
            case .royalty(var fee):
                fee.allCollectorsAreExempt = newValue
            }
        }
    }
}

extension AnyCustomFee: ValidateChecksums {
    internal func validateChecksums(on ledgerId: LedgerId) throws {
        switch self {
        case .fixed(let fee):
            try fee.validateChecksums(on: ledgerId)
        case .fractional(let fee):
            try fee.validateChecksums(on: ledgerId)
        case .royalty(let fee):
            try fee.validateChecksums(on: ledgerId)
        }
    }
}

/// A fixed number of units (hbar or token) to assess as a fee during a `TransferTransaction` that transfers
/// units of the token to which this fixed fee is attached.
public struct FixedFee: CustomFee, Codable, ValidateChecksums {
    public var feeCollectorAccountId: AccountId?

    public var allCollectorsAreExempt: Bool

    /// Create a new `CustomFixedFee`.
    public init(
        amount: UInt64 = 0,
        denominatingTokenId: TokenId? = nil,
        feeCollectorAccountId: AccountId? = nil,
        allCollectorsAreExempt: Bool = false
    ) {
        self.amount = amount
        self.denominatingTokenId = denominatingTokenId
        self.feeCollectorAccountId = feeCollectorAccountId
        self.allCollectorsAreExempt = allCollectorsAreExempt
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

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try denominatingTokenId?.validateChecksums(on: ledgerId)
        try feeCollectorAccountId?.validateChecksums(on: ledgerId)
    }
}

/// A fraction of the transferred units of a token to assess as a fee.
///
/// The amount assessed will never be less than the given `minimumAmount`, and never greater
/// than the given `maximumAmount`.
///
/// The denomination is always in units of the token to which this fractional fee is attached.
///
public struct FractionalFee: CustomFee, Codable, ValidateChecksums {
    public var feeCollectorAccountId: AccountId?

    public var allCollectorsAreExempt: Bool

    /// Create a new `CustomFixedFee`.
    public init(
        amount: Rational<UInt64> = "1/1",
        minimumAmount: UInt64 = 0,
        maximumAmount: UInt64 = 0,
        netOfTransfers: Bool = false,
        feeCollectorAccountId: AccountId? = nil,
        allCollectorsAreExempt: Bool = false
    ) {
        self.denominator = amount.denominator
        self.numerator = amount.numerator
        self.minimumAmount = minimumAmount
        self.maximumAmount = maximumAmount
        self.netOfTransfers = netOfTransfers
        self.feeCollectorAccountId = feeCollectorAccountId
        self.allCollectorsAreExempt = allCollectorsAreExempt
    }

    /// The fraction of the transferred units to assess as a fee.
    public var amount: Rational<UInt64> {
        get {
            Rational(numerator, denominator)
        }
        set(new) {
            numerator = new.numerator
            denominator = new.denominator
        }
    }

    /// Denominator of `amount`
    public var denominator: UInt64

    /// Numerator of `amount`
    public var numerator: UInt64

    /// Sets the fraction of the transferred units to assess as a fee.
    @discardableResult
    public mutating func amount(_ amount: Rational<UInt64>) -> Self {
        self.amount = amount

        return self
    }

    /// Sets the denominator of `amount`
    ///
    /// - Parameters:
    ///   - denominator: the new denominator to use.
    ///
    /// - Returns: `self`.
    @discardableResult
    public mutating func denominator(_ denominator: UInt64) -> Self {
        self.denominator = denominator

        return self
    }

    /// Sets the numerator of `amount`
    ///
    /// - Parameters:
    ///   - numerator: the new numerator to use.
    ///
    /// - Returns: `self`.
    @discardableResult
    public mutating func numerator(_ numerator: UInt64) -> Self {
        self.numerator = numerator

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

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try feeCollectorAccountId?.validateChecksums(on: ledgerId)
    }
}

/// A fee to assess during a `TransferTransaction` that changes ownership of an NFT.
///
/// Defines the fraction of the fungible value exchanged for an NFT that the ledger
/// should collect as a royalty.
public struct RoyaltyFee: CustomFee, Codable {
    public var feeCollectorAccountId: AccountId?

    public var allCollectorsAreExempt: Bool

    /// Create a new `CustomRoyaltyFee`.
    public init(
        exchangeValue: Rational<UInt64> = "1/1",
        fallbackFee: FixedFee? = nil,
        feeCollectorAccountId: AccountId? = nil,
        allCollectorsAreExempt: Bool = false
    ) {
        self.init(
            numerator: exchangeValue.numerator, denominator: exchangeValue.denominator, fallbackFee: fallbackFee,
            feeCollectorAccountId: feeCollectorAccountId, allCollectorsAreExempt: allCollectorsAreExempt
        )
    }

    /// Create a new `CustomRoyaltyFee`
    public init(
        numerator: UInt64 = 1,
        denominator: UInt64 = 1,
        fallbackFee: FixedFee? = nil,
        feeCollectorAccountId: AccountId? = nil,
        allCollectorsAreExempt: Bool = false
    ) {
        self.numerator = numerator
        self.denominator = denominator
        self.fallbackFee = fallbackFee
        self.feeCollectorAccountId = feeCollectorAccountId
        self.allCollectorsAreExempt = allCollectorsAreExempt
    }

    /// The fraction of fungible value exchanged for an NFT to collect as royalty.
    public var exchangeValue: Rational<UInt64> {
        get {
            Rational(numerator, denominator)
        }
        set(new) {
            numerator = new.numerator
            denominator = new.denominator
        }
    }

    /// Denominator of `exchangeValue`
    public var denominator: UInt64

    /// Numerator of `exchangeValue`
    public var numerator: UInt64

    /// Sets the fraction of fungible value exchanged for an NFT to collect as royalty.
    @discardableResult
    public mutating func exchangeValue(_ exchangeValue: Rational<UInt64>) -> Self {
        self.exchangeValue = exchangeValue

        return self
    }

    /// Sets the denominator of `exchangeValue`
    ///
    /// - Parameters:
    ///   - denominator: the new denominator to use.
    ///
    /// - Returns: `self`.
    @discardableResult
    public mutating func denominator(_ denominator: UInt64) -> Self {
        self.denominator = denominator

        return self
    }

    /// Sets the numerator of `exchangeValue`
    ///
    /// - Parameters:
    ///   - numerator: the new numerator to use.
    ///
    /// - Returns: `self`.
    @discardableResult
    public mutating func numerator(_ numerator: UInt64) -> Self {
        self.numerator = numerator

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

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try fallbackFee?.validateChecksums(on: ledgerId)
        try feeCollectorAccountId?.validateChecksums(on: ledgerId)
    }
}
