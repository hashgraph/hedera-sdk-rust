/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

import GRPC
import NIOCore

internal final class MirrorNetwork {
    private enum State {
        case idle(target: GRPC.ConnectionTarget)
        case connected(GRPCChannel)
    }

    private var state: State

    private init(_ state: State) {
        self.state = state
    }

    private static func idle(target: GRPC.ConnectionTarget) -> Self {
        Self(.idle(target: target))
    }

    internal static func mainnet() -> Self {
        .idle(target: .hostAndPort("mainnet-public.mirrornode.hedera.com", 443))
    }

    internal static func testnet() -> Self {
        .idle(target: .hostAndPort("hcs.testnet.mirrornode.hedera.com", 5600))
    }

    internal static func previewnet() -> Self {
        .idle(target: .hostAndPort("hcs.previewnet.mirrornode.hedera.com", 5600))
    }

    internal func channel(_ eventLoop: NIOCore.EventLoopGroup) -> GRPCChannel {
        switch state {
        case .idle(let target):
            let channel = GRPC.ClientConnection(configuration: .default(target: target, eventLoopGroup: eventLoop))
            self.state = .connected(channel)
            return channel
        case .connected(let channel):
            return channel
        }
    }
}
