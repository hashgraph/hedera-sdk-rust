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

/// Delete a file or smart contract - can only be done with a Hedera admin.
public final class SystemDeleteTransaction: Transaction {
    /// Create a new `SystemDeleteTransaction`.
    public init(
        fileId: FileId? = nil,
        contractId: ContractId? = nil,
        expirationTime: Timestamp? = nil
    ) {
        self.fileId = fileId
        self.contractId = contractId
        self.expirationTime = expirationTime

        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        fileId = try container.decodeIfPresent(.fileId)
        contractId = try container.decodeIfPresent(.contractId)
        expirationTime = try container.decodeIfPresent(.expirationTime)

        try super.init(from: decoder)
    }

    /// The file ID which should be deleted.
    public var fileId: FileId?

    /// Sets the file ID which should be deleted.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The contract ID which should be deleted.
    public var contractId: ContractId?

    /// Sets the contract ID which should be deleted.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId

        return self
    }

    /// The timestamp at which the "deleted" file should
    /// truly be permanently deleted.
    public var expirationTime: Timestamp?

    /// Sets the timestamp at which the "deleted" file should
    /// truly be permanently deleted.
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
        case contractId
        case expirationTime
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(fileId, forKey: .fileId)
        try container.encodeIfPresent(contractId, forKey: .contractId)
        try container.encodeIfPresent(expirationTime, forKey: .expirationTime)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try contractId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal static func fromProtobufData(_ proto: Proto_SystemDeleteTransactionBody) throws -> Self {
        let fileId: FileId?
        let contractId: ContractId?

        switch proto.id {
        case .fileID(let id):
            (fileId, contractId) = (.fromProtobuf(id), nil)
        case .contractID(let id):
            (fileId, contractId) = (nil, try .fromProtobuf(id))
        case nil:
            (fileId, contractId) = (nil, nil)
        }

        return Self(
            fileId: fileId,
            contractId: contractId,
            expirationTime: proto.hasExpirationTime
                ? Timestamp(seconds: UInt64(proto.expirationTime.seconds), subSecondNanos: 0) : nil
        )
    }

    internal override func execute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        if let _ = fileId {
            return try await Proto_FileServiceAsyncClient(channel: channel).systemDelete(request)
        }

        if let _ = contractId {
            return try await Proto_SmartContractServiceAsyncClient(channel: channel).systemDelete(request)
        }

        fatalError("\(type(of: self)) has no `fileId`/`contractId`")
    }

    internal override func toTransactionDataProtobuf(_ nodeAccountId: AccountId, _ transactionId: TransactionId)
        -> Proto_TransactionBody.OneOf_Data
    {
        .systemDelete(
            .with { proto in
                if let fileId = fileId {
                    proto.fileID = fileId.toProtobuf()
                }

                if let contractId = contractId {
                    proto.contractID = contractId.toProtobuf()
                }

                if let expirationTime = expirationTime {
                    proto.expirationTime = .with { $0.seconds = Int64(expirationTime.seconds) }
                }
            }
        )
    }
}
