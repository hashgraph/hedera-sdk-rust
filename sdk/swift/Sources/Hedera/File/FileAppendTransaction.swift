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

import Foundation
import GRPC
import HederaProtobufs

/// Append the given contents to the end of the specified file.
public final class FileAppendTransaction: ChunkedTransaction {
    /// Create a new `FileAppendTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: [Proto_FileAppendTransactionBody]) throws {
        var iter = data.makeIterator()
        let first = iter.next()!

        self.fileId = first.hasFileID ? .fromProtobuf(first.fileID) : nil
        let chunks = data.count
        var contents: Data = first.contents
        var largestChunkSize = max(first.contents.count, 1)

        for item in iter {
            largestChunkSize = max(largestChunkSize, item.contents.count)
            contents.append(item.contents)
        }

        try super.init(protobuf: proto, data: contents, chunks: chunks, largestChunkSize: largestChunkSize)
    }

    /// The file to which the bytes will be appended.
    public var fileId: FileId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the file to which the bytes will be appended.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The bytes that will be appended to the end of the specified file.
    public var contents: Data {
        get { data }
        set(contents) {
            ensureNotFrozen()
            data = contents
        }
    }

    /// Sets the bytes that will be appended to the end of the specified file.
    @discardableResult
    public func contents(_ contents: Data) -> Self {
        self.contents = contents

        return self
    }

    /// Sets `self.contents` to the UTF-8 encoded bytes of `contents`.
    @discardableResult
    public func contents(_ contents: String) -> Self {
        self.contents = contents.data(using: .utf8)!

        return self
    }

    internal override var waitForReceipt: Bool { true }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_FileServiceAsyncClient(channel: channel).appendContent(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        .fileAppend(
            .with { proto in
                self.fileId?.toProtobufInto(&proto.fileID)
                proto.contents = self.messageChunk(chunkInfo)
            }
        )
    }
}

extension FileAppendTransaction: ToSchedulableTransactionData {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        precondition(self.usedChunks == 1)

        return .fileAppend(
            .with { proto in
                self.fileId?.toProtobufInto(&proto.fileID)
                proto.contents = contents
            }
        )
    }
}
