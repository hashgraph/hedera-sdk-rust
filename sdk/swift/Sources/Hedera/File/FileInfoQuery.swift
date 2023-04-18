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

/// Get all the information about a file.
public final class FileInfoQuery: Query<FileInfo> {
    /// Create a new `FileInfoQuery`.
    public init(
        fileId: FileId? = nil
    ) {
        self.fileId = fileId
    }

    /// The file ID for which information is requested.
    public var fileId: FileId?

    /// Sets the file ID for which information is requested.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.fileGetInfo = .with { proto in
                proto.header = header
                fileId?.toProtobufInto(&proto.fileID)
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_FileServiceAsyncClient(channel: channel).getFileInfo(request)
    }

    internal override func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        guard case .fileGetInfo(let proto) = response else {
            throw HError.fromProtobuf("unexpected \(response) received, expected `fileGetInfo`")
        }

        return try .fromProtobuf(proto.fileInfo)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
