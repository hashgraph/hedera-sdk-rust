/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

import Foundation

/// Common units of hbar.
/// For the most part they follow SI prefix conventions.
public enum HbarUnit: UInt64, LosslessStringConvertible, ExpressibleByStringLiteral {
    case tinybar = 1
    case microbar = 100
    case millibar = 100_000
    case hbar = 100_000_000
    case kilobar = 100_000_000_000
    case megabar = 100_000_000_000_000
    case gigabar = 100_000_000_000_000_000

    public func getSymbol() -> String {
        description
    }

    public var description: String {
        switch self {
        case .tinybar:
            return "tℏ"
        case .microbar:
            return "µℏ"
        case .millibar:
            return "mℏ"
        case .hbar:
            return "ℏ"
        case .kilobar:
            return "kℏ"
        case .megabar:
            return "Mℏ"
        case .gigabar:
            return "Gℏ"
        }
    }

    public init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    fileprivate init<S: StringProtocol>(parsing description: S) throws {
        switch description {
        case "tℏ":
            self = .tinybar
        case "µℏ":
            self = .microbar
        case "mℏ":
            self = .millibar
        case "ℏ":
            self = .hbar
        case "kℏ":
            self = .kilobar
        case "Mℏ":
            self = .megabar
        case "Gℏ":
            self = .gigabar
        default:
            throw HError(kind: .basicParse, description: "unit must be a valid hbar unit")
        }
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public func tinybar() -> UInt64 {
        self.rawValue
    }
}

public struct Hbar: LosslessStringConvertible, Codable, ExpressibleByIntegerLiteral,
    ExpressibleByStringLiteral, ExpressibleByFloatLiteral, Equatable
{
    /// A constant value of zero hbars.
    public static let zero: Hbar = 0

    /// A constant value of the maximum number of hbars.
    public static let max: Hbar = 50_000_000_000

    /// A constant value of the minimum number of hbars.
    public static let min: Hbar = -50_000_000_000

    /// Create a new Hbar of the specified, possibly fractional value.
    public init(_ amount: Decimal, _ unit: HbarUnit = .hbar) throws {
        guard amount.isFinite else {
            throw HError(kind: .basicParse, description: "amount must be a finite decimal number")
        }

        let tinybars = amount * Decimal(unit.rawValue)

        guard tinybars.isZero || (tinybars.isNormal && tinybars.exponent >= 0) else {
            throw HError(
                kind: .basicParse,
                description:
                    "amount and unit combination results in a fractional value for tinybar, ensure tinybar value is a whole number"
            )
        }

        self.tinybars = NSDecimalNumber(decimal: tinybars).int64Value
    }

    public init(stringLiteral value: StringLiteralType) {
        // swiftlint:disable force_try
        try! self.init(parsing: value)
    }

    public init(integerLiteral value: IntegerLiteralType) {
        // swiftlint:disable force_try
        try! self.init(Decimal(value))
    }

    public init(floatLiteral value: FloatLiteralType) {
        // swiftlint:disable force_try
        try! self.init(Decimal(value))
    }

    public static func fromString(_ description: String) throws -> Self {
        return try Self(parsing: description)
    }

    private init<S: StringProtocol>(parsing description: S) throws {
        let (rawAmount, rawUnit) = description.splitOnce(on: " ") ?? (description[...], nil)

        let unit = try rawUnit.map { try HbarUnit(parsing: $0) } ?? .hbar

        guard let amount = Decimal(string: String(rawAmount)) else {
            throw HError(kind: .basicParse, description: "amount not parsable as a decimal")
        }

        try self.init(amount, unit)
    }

    public init?(_ description: String) {
        try? self.init(parsing: description)
    }

    public static func from(_ amount: Decimal, _ unit: HbarUnit = .hbar) throws -> Self {
        try Self(amount, unit)
    }

    public static func fromTinybars(_ amount: Int64) -> Self {
        Self(tinybars: amount)
    }

    public init(from decoder: Decoder) throws {
        self.init(tinybars: try decoder.singleValueContainer().decode(Int64.self))
    }

    private init(tinybars: Int64) {
        self.tinybars = tinybars
    }

    private let tinybars: Int64

    public func getValue() -> Decimal {
        to(.hbar)
    }

    public func negated() -> Self {
        Self(tinybars: -tinybars)
    }

    /// Convert this hbar value to a different unit.
    public func to(_ unit: HbarUnit) -> Decimal {
        Decimal(tinybars) / Decimal(unit.rawValue)
    }

    /// Convert this hbar value to Tinybars.
    public func toTinybars() -> Int64 {
        tinybars
    }

    public func toString(_ unit: HbarUnit? = nil) -> String {
        let unit = unit ?? (abs(tinybars) < 10_000 ? .tinybar : .hbar)

        return "\(to(unit)) \(unit)"
    }

    public var description: String {
        toString()
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(toTinybars())
    }
}
