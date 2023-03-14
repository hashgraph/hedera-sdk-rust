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
import HederaProtobufs

/// Possible `FeeData` subtypes.
public enum FeeDataType {
    /// The resource prices have no special scope.
    case `default`

    /// The resource prices are scoped to an operation on a fungible token.
    case tokenFungibleCommon

    /// The resource prices are scoped to an operation on a non-fungible token.
    case tokenNonFungibleUnique

    /// The resource prices are scoped to an operation on a fungible token with a custom fee schedule.
    case tokenFungibleCommonWithCustomFees

    /// The resource prices are scoped to an operation on a non-fungible token with a custom fee schedule.
    case tokenNonFungibleUniqueWithCustomFees

    /// The resource prices are scoped to a `ScheduleCreateTransaction`
    /// containing a `ContractExecuteTransaction`.
    case scheduleCreateContractCall
}

extension FeeDataType: TryProtobufCodable {
    internal typealias Protobuf = Proto_SubType

    internal init(protobuf proto: Proto_SubType) throws {
        switch proto {
        case .default: self = .default
        case .tokenFungibleCommon: self = .tokenFungibleCommon
        case .tokenNonFungibleUnique: self = .tokenNonFungibleUnique
        case .tokenFungibleCommonWithCustomFees: self = .tokenFungibleCommonWithCustomFees
        case .tokenNonFungibleUniqueWithCustomFees: self = .tokenNonFungibleUniqueWithCustomFees
        case .scheduleCreateContractCall: self = .scheduleCreateContractCall
        case .UNRECOGNIZED(let code):
            throw HError.fromProtobuf("unrecognized FeeDataType `\(code)`")
        }
    }

    internal func toProtobuf() -> Protobuf {
        switch self {
        case .default: return .default
        case .tokenFungibleCommon: return .tokenFungibleCommon
        case .tokenNonFungibleUnique: return .tokenNonFungibleUnique
        case .tokenFungibleCommonWithCustomFees: return .tokenFungibleCommonWithCustomFees
        case .tokenNonFungibleUniqueWithCustomFees: return .tokenNonFungibleUniqueWithCustomFees
        case .scheduleCreateContractCall: return .scheduleCreateContractCall
        }
    }
}
