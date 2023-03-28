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

public struct LedgerId: LosslessStringConvertible, ExpressibleByStringLiteral, Equatable,
    CustomStringConvertible
{
    public static let mainnet = LedgerId(Data([0]))

    public static let testnet = LedgerId(Data([1]))

    public static let previewnet = LedgerId(Data([2]))

    public static func fromBytes(_ bytes: Data) -> Self {
        Self(bytes)
    }

    public static func fromString(_ description: String) -> Self? {
        Self(description)
    }

    public init(_ bytes: Data) {
        self.bytes = bytes
    }

    public init(stringLiteral value: StringLiteralType) {
        self.init(value)!
    }

    public init?(_ description: String) {
        switch description {
        case "mainnet":
            self = .mainnet
            return
        case "testnet":
            self = .testnet
            return
        case "previewnet":
            self = .previewnet
            return
        default:
            guard let bytes = Data(hexEncoded: description) else {
                return nil
            }

            self.bytes = bytes
        }
    }

    internal let bytes: Data

    public func isMainnet() -> Bool {
        self == .mainnet
    }

    public func isTestnet() -> Bool {
        self == .testnet
    }

    public func isPreviewnet() -> Bool {
        self == .previewnet
    }

    public static func == (lhs: Self, rhs: Self) -> Bool {
        lhs.bytes == rhs.bytes
    }

    public var description: String {
        if isMainnet() {
            return "mainnet"
        }

        if isTestnet() {
            return "testnet"
        }

        if isPreviewnet() {
            return "previewnet"
        }

        return bytes.hexStringEncoded()
    }

    public func toString() -> String {
        description
    }
}

#if compiler(<5.7)
    // Swift 5.7 added the conformance to data, despite to the best of my knowledge, not changing anything in the underlying type.
    extension LedgerId: @unchecked Sendable {}
#else
    extension LedgerId: Sendable {}
#endif
