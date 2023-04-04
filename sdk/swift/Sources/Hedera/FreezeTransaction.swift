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

/// Set the freezing period in which the platform will stop creating
/// events and accepting transactions.
public final class FreezeTransaction: Transaction {
    /// Create a new `FreezeTransaction`.
    public init(
        startTime: Timestamp? = nil,
        fileId: FileId? = nil,
        fileHash: Data? = nil,
        freezeType: FreezeType = .unknown
    ) {
        self.startTime = startTime
        self.fileId = fileId
        self.fileHash = fileHash
        self.freezeType = freezeType

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_FreezeTransactionBody) throws {
        startTime = data.hasStartTime ? .fromProtobuf(data.startTime) : nil
        fileId = data.hasUpdateFile ? .fromProtobuf(data.updateFile) : nil
        fileHash = !data.fileHash.isEmpty ? data.fileHash : nil
        freezeType = try .fromProtobuf(data.freezeType)

        try super.init(protobuf: proto)
    }

    /// The start time.
    public var startTime: Timestamp? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the start time.
    @discardableResult
    public func startTime(_ startTime: Timestamp) -> Self {
        self.startTime = startTime

        return self
    }

    /// The file ID.
    public var fileId: FileId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the file ID.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The file hash.
    public var fileHash: Data? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the file hash.
    @discardableResult
    public func fileHash(_ fileHash: Data) -> Self {
        self.fileHash = fileHash

        return self
    }

    /// The freeze type.
    public var freezeType: FreezeType {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the freeze type.
    @discardableResult
    public func freezeType(_ freezeType: FreezeType) -> Self {
        self.freezeType = freezeType

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_FreezeServiceAsyncClient(channel: channel).freeze(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .freeze(toProtobuf())
    }
}

extension FreezeTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_FreezeTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            if let fileId = fileId {
                proto.updateFile = fileId.toProtobuf()
            }

            if let startTime = startTime {
                proto.startTime = startTime.toProtobuf()
            }

            if let fileHash = fileHash {
                proto.fileHash = fileHash
            }

            proto.freezeType = freezeType.toProtobuf()
        }
    }
}

extension FreezeTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .freeze(toProtobuf())
    }
}
