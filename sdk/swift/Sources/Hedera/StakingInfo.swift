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
import HederaProtobufs

public struct StakingInfo: Codable {
    internal init(
        declineStakingReward: Bool,
        stakePeriodStart: Timestamp?,
        pendingReward: Hbar,
        stakedToMe: Hbar,
        stakedAccountId: AccountId?,
        stakedNodeId: UInt64?
    ) {
        self.declineStakingReward = declineStakingReward
        self.stakePeriodStart = stakePeriodStart
        self.pendingReward = pendingReward
        self.stakedToMe = stakedToMe
        self.stakedAccountId = stakedAccountId
        self.stakedNodeId = stakedNodeId
    }

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
        try Self(fromProtobufBytes: bytes)
    }

    public func toBytes() -> Data {
        toProtobufBytes()
    }
}

extension StakingInfo: TryProtobufCodable {
    internal typealias Protobuf = Proto_StakingInfo

    internal init(fromProtobuf proto: Protobuf) throws {
        let stakePeriodStart = proto.hasStakePeriodStart ? proto.stakePeriodStart : nil
        let stakedAccountId: Proto_AccountID?
        let stakedNodeId: UInt64?
        switch proto.stakedID {
        case .none:
            stakedAccountId = nil
            stakedNodeId = nil
        case .some(.stakedNodeID(let nodeId)):
            stakedAccountId = nil
            stakedNodeId = UInt64(nodeId)
        case .some(.stakedAccountID(let accountId)):
            stakedAccountId = accountId
            stakedNodeId = nil
        }

        self.init(
            declineStakingReward: proto.declineReward,
            stakePeriodStart: .fromProtobuf(stakePeriodStart),
            pendingReward: Hbar.fromTinybars(proto.pendingReward),
            stakedToMe: Hbar.fromTinybars(proto.stakedToMe),
            stakedAccountId: try .fromProtobuf(stakedAccountId),
            stakedNodeId: stakedNodeId
        )
    }

    func toProtobuf() -> Protobuf {
        .with { proto in
            proto.declineReward = declineStakingReward
            stakePeriodStart?.toProtobufInto(&proto.stakePeriodStart)
            proto.pendingReward = pendingReward.toTinybars()
            proto.stakedToMe = stakedToMe.toTinybars()

            stakedAccountId?.toProtobufInto(&proto.stakedAccountID)

            // node ID wins, so it goes last.
            if let nodeId = stakedNodeId {
                proto.stakedNodeID = Int64(nodeId)
            }
        }
    }
}
