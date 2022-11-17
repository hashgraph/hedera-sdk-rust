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

import CHedera
import Foundation

public struct StakingInfo: Codable {
    /// If true, the contract declines receiving a staking reward. The default value is false.
    public let declineStakingReward: Bool

    /// The staking period during which either the staking settings for this account or contract changed
    /// (such as starting staking or changing staked_node_id)
    /// or the most recent reward was earned, whichever is later.
    /// If this account or contract is not currently staked to a node, then this field is not set.
    public let stakePeriodStart: Timestamp?

    /// The amount in Hbar that will be received in the next reward situation.
    public let pendingReward: Hbar

    /// The total of balance of all accounts staked to this account or contract.
    public let stakedToMe: Hbar

    /// The account to which this account or contract is staking.
    public let stakedAccountId: AccountId?

    /// The ID of the node this account or contract is staked to.
    public let stakedNodeId: UInt64?

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        declineStakingReward = try container.decode(Bool.self, forKey: .declineStakingReward)
        stakePeriodStart = try container.decodeIfPresent(Timestamp.self, forKey: .stakePeriodStart)
        pendingReward = try container.decode(Hbar.self, forKey: .pendingReward)
        stakedToMe = try container.decode(Hbar.self, forKey: .stakedToMe)
        stakedAccountId = try container.decodeIfPresent(AccountId.self, forKey: .stakedAccountId)
        stakedNodeId = try container.decodeIfPresent(UInt64.self, forKey: .stakedNodeId)
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        let json: String = try bytes.withUnsafeTypedBytes { pointer in
            var ptr: UnsafeMutablePointer<CChar>? = nil
            let err = hedera_staking_info_from_bytes(
                pointer.baseAddress,
                pointer.count,
                &ptr
            )

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return String(hString: ptr!)
        }

        return try JSONDecoder().decode(Self.self, from: json.data(using: .utf8)!)
    }

    private func toBytesInner() throws -> Data {
        let jsonBytes = try JSONEncoder().encode(self)
        let json = String(data: jsonBytes, encoding: .utf8)!
        var buf: UnsafeMutablePointer<UInt8>?
        var bufSize: Int = 0
        let err = hedera_staking_info_to_bytes(json, &buf, &bufSize)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return Data(bytesNoCopy: buf!, count: bufSize, deallocator: Data.unsafeCHederaBytesFree)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toBytesInner()
    }
}
