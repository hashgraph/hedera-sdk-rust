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

internal struct MirrorNetwork {
    private enum Targets {
        static let mainnet: GRPC.ConnectionTarget = .hostAndPort("mainnet-public.mirrornode.hedera.com", 443)
        static let testnet: GRPC.ConnectionTarget = .hostAndPort("hcs.testnet.mirrornode.hedera.com", 5600)
        static let previewnet: GRPC.ConnectionTarget = .hostAndPort("hcs.previewnet.mirrornode.hedera.com", 5600)
    }

    internal let channel: GRPC.ClientConnection

    private init(channel: GRPC.ClientConnection) {
        self.channel = channel
    }

    private init(target: GRPC.ConnectionTarget, eventLoop: EventLoopGroup) {
        self.init(
            channel: GRPC.ClientConnection(configuration: .default(target: target, eventLoopGroup: eventLoop))
        )
    }

    internal static func mainnet(_ eventLoop: NIOCore.EventLoopGroup) -> Self {
        Self(target: Targets.mainnet, eventLoop: eventLoop)
    }

    internal static func testnet(_ eventLoop: NIOCore.EventLoopGroup) -> Self {
        Self(target: Targets.testnet, eventLoop: eventLoop)
    }

    internal static func previewnet(_ eventLoop: NIOCore.EventLoopGroup) -> Self {
        Self(target: Targets.previewnet, eventLoop: eventLoop)
    }
}
