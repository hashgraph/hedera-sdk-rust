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

/// Undelete a file or smart contract that was deleted by SystemDelete.
public final class SystemUndeleteTransaction: Transaction {
    /// Create a new `SystemUndeleteTransaction`.
    public init(
        fileId: FileId? = nil,
        contractId: ContractId? = nil
    ) {
        self.fileId = fileId
        self.contractId = contractId

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_SystemUndeleteTransactionBody) throws {
        switch data.id {
        case .contractID(let contractId):
            self.contractId = try .fromProtobuf(contractId)
        case .fileID(let fileId):
            self.fileId = .fromProtobuf(fileId)
        case nil:
            break
        }

        try super.init(protobuf: proto)
    }

    /// The file ID to undelete.
    public var fileId: FileId? {
        willSet {
            ensureNotFrozen(fieldName: "fileId")
        }
    }

    /// Sets the file ID to undelete.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The contract ID to undelete.
    public var contractId: ContractId? {
        willSet {
            ensureNotFrozen(fieldName: "contractId")
        }
    }

    /// Sets the contract ID to undelete.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try contractId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        if fileId != nil {
            return try await Proto_FileServiceAsyncClient(channel: channel).systemUndelete(request)
        }

        if contractId != nil {
            return try await Proto_SmartContractServiceAsyncClient(channel: channel).systemUndelete(request)
        }

        fatalError("\(type(of: self)) has no `fileId`/`contractId`")
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .systemUndelete(
            .with { proto in
                if let fileId = fileId {
                    proto.fileID = fileId.toProtobuf()
                }

                if let contractId = contractId {
                    proto.contractID = contractId.toProtobuf()
                }
            }
        )
    }
}

extension SystemUndeleteTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_SystemUndeleteTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            if let fileId = fileId {
                proto.fileID = fileId.toProtobuf()
            }

            if let contractId = contractId {
                proto.contractID = contractId.toProtobuf()
            }
        }
    }
}

extension SystemUndeleteTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .systemUndelete(toProtobuf())
    }
}
