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

/// Delete the given file.
///
/// After deletion, it will be marked as deleted and will have no contents.
/// Information about it will continue to exist until it expires.
///
public final class FileDeleteTransaction: Transaction {
    /// Create a new `FileDeleteTransaction`.
    public required init(
        fileId: FileId? = nil
    ) {
        self.fileId = fileId

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_FileDeleteTransactionBody) throws {
        fileId = data.hasFileID ? .fromProtobuf(data.fileID) : nil

        try super.init(protobuf: proto)
    }

    /// The file to delete. It will be marked as deleted until it expires.
    /// Then it will disappear.
    public var fileId: FileId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the file to delete. It will be marked as deleted until it expires.
    /// Then it will disappear.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_FileServiceAsyncClient(channel: channel).deleteFile(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .fileDelete(toProtobuf())
    }
}

extension FileDeleteTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_FileDeleteTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            fileId?.toProtobufInto(&proto.fileID)
        }
    }
}

extension FileDeleteTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .fileDelete(toProtobuf())
    }
}
