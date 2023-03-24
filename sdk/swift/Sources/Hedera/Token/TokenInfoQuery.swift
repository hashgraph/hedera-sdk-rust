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

/// Gets information about the Token instance.
public final class TokenInfoQuery: Query<TokenInfo> {
    /// Create a new `TokenInfoQuery`.
    public init(
        tokenId: TokenId? = nil
    ) {
        self.tokenId = tokenId
    }

    /// The token ID for which information is requested.
    public var tokenId: TokenId?

    /// Sets the token ID for which information is requested.
    @discardableResult
    public func tokenId(_ tokenId: TokenId) -> Self {
        self.tokenId = tokenId

        return self
    }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.tokenGetInfo = .with { proto in
                proto.header = header
                tokenId?.toProtobufInto(&proto.token)
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_TokenServiceAsyncClient(channel: channel).getTokenInfo(request)
    }

    internal override func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        guard case .tokenGetInfo(let proto) = response else {
            throw HError.fromProtobuf("unexpected \(response) received, expected `tokenGetInfo`")
        }

        return try .fromProtobuf(proto.tokenInfo)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try tokenId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
