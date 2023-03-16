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

/// Gets info on an NFT for a given TokenID and serial number.
public final class TokenNftInfoQuery: Query<TokenNftInfo> {
    /// Create a new `TokenNftInfoQuery`.
    public init(
        nftId: NftId? = nil
    ) {
        self.nftId = nftId
    }

    /// The nft ID for which information is requested.
    public var nftId: NftId?

    /// Sets the nft ID for which information is requested.
    @discardableResult
    public func nftId(_ nftId: NftId) -> Self {
        self.nftId = nftId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case nftId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(nftId, forKey: .nftId)

        try super.encode(to: encoder)
    }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.tokenGetNftInfo = .with { proto in
                proto.header = header
                nftId?.toProtobufInto(&proto.nftID)
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_TokenServiceAsyncClient(channel: channel).getTokenNftInfo(request)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try nftId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
