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

import HederaProtobufs

/// Information about a single account that is proxy staking.
public struct ProxyStaker {
    /// The Account ID that is proxy staking.
    public let accountId: AccountId

    /// The number of hbars that are currently proxy staked.
    public let amount: UInt64
}

extension ProxyStaker: TryProtobufCodable {
    internal typealias Protobuf = Proto_ProxyStaker

    internal init(protobuf proto: Protobuf) throws {
        self.init(accountId: try .fromProtobuf(proto.accountID), amount: UInt64(proto.amount))
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.accountID = accountId.toProtobuf()
            proto.amount = Int64(amount)
        }
    }
}
