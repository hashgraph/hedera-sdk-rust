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

import GRPC
import HederaProtobufs

/// Get information about the versions of protobuf and hedera.
///
public final class NetworkVersionInfoQuery: Query<NetworkVersionInfo> {
    /// Create a new `NetworkVersionInfoQuery`.
    public override init() {
    }

    internal override var requiresPayment: Bool { false }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.networkGetVersionInfo = .with { proto in
                proto.header = header
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_NetworkServiceAsyncClient(channel: channel).getVersionInfo(request)
    }

    internal override func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        guard case .networkGetVersionInfo(let proto) = response else {
            throw HError.fromProtobuf("unexpected \(response) received, expected `networkGetVersionInfo`")
        }

        return .fromProtobuf(proto)
    }
}
