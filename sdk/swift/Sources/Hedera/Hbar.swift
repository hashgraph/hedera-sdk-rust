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
public enum HbarUnit: UInt64 {
    case tinybar = 1
    case microbar = 100
    case millibar = 100_000
    case hbar = 100_000_000
    case kilobar = 100_000_000_000
    case megabar = 100_000_000_000_000
    case gigabar = 100_000_000_000_000_000
}

public struct Hbar: LosslessStringConvertible, Codable, ExpressibleByIntegerLiteral, ExpressibleByStringLiteral, ExpressibleByFloatLiteral {
    /// A constant value of zero hbars.
    public static let zero: Hbar = 0

    /// A constant value of the maximum number of hbars.
    public static let max: Hbar = 50000000000

    /// A constant value of the minimum number of hbars.
    public static let min: Hbar = -50000000000

    public static func fromTinybars(_ amount: Int64) -> Self {
        Self(tinybars: amount)
    }

    public init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public init(integerLiteral value: IntegerLiteralType) {
        self.init(tinybars: Int64(value))
    }

    public init(floatLiteral value: FloatLiteralType) {
        try! self.init(Decimal(value))
    }

    public init?(_ description: String) {
        let amount = NSDecimalNumber(string: description).decimalValue

        if (amount.isNaN) {
            return nil
        }

        let hbar = try? Self(amount)

        if (hbar == nil) {
            return nil
        }

        self = hbar!
    }

    public init(from decoder: Decoder) throws {
        self.init(tinybars: try decoder.singleValueContainer().decode(Int64.self))
    }

    private init(tinybars: Int64) {
        self.tinybars = tinybars
    }

    /// Create a new Hbar of the specified, possibly fractional value.
    public init(_ amount: Decimal) throws {
        self = try Self(amount, HbarUnit.hbar)
    }

    /// Create a new Hbar of the specified, possibly fractional value.
    public init(_ amount: Decimal, _ unit: HbarUnit) throws {
        let tinybars = amount * Decimal(unit.rawValue);

        if (!(tinybars.isZero || (tinybars.isNormal && tinybars.exponent >= 0))) {
            throw NSError(domain: "Amount and Unit combination results in a fractional value for tinybar.  Ensure tinybar value is a whole number.", code: 0)
        }

        self.tinybars = NSDecimalNumber(decimal: tinybars).int64Value
    }

    private let tinybars: Int64

    /// Convert this hbar value to a different unit.
    public func to(_ unit: HbarUnit) -> Decimal {
        Decimal(tinybars) / Decimal(unit.rawValue)
    }

    /// Convert this hbar value to Tinybars.
    public func toTinybars() -> Int64 {
        tinybars
    }

    public var description: String {
        to(.hbar).description
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        try container.encode(toTinybars())
    }
}
