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

/// Submit an Ethereum transaction.
public final class EthereumTransaction: Transaction {
    public init(
        ethereumData: Data? = nil,
        callDataFileId: FileId? = nil,
        maxGasAllowanceHbar: Hbar = 0
    ) {
        self.ethereumData = ethereumData
        self.callDataFileId = callDataFileId
        self.maxGasAllowanceHbar = maxGasAllowanceHbar

        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_EthereumTransactionBody) throws {
        self.ethereumData = !data.ethereumData.isEmpty ? data.ethereumData : nil
        self.callDataFileId = data.hasCallData ? .fromProtobuf(data.callData) : nil
        self.maxGasAllowanceHbar = .fromTinybars(data.maxGasAllowance)

        try super.init(protobuf: proto)
    }

    /// The raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    public var ethereumData: Data? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    @discardableResult
    public func ethereumData(_ ethereumData: Data) -> Self {
        self.ethereumData = ethereumData

        return self
    }

    /// For large transactions (for example contract create) this should be used to
    /// set the FileId of an HFS file containing the callData
    /// of the ethereumData. The data in the ethereumData will be re-written with
    /// the callData element as a zero length string with the original contents in
    /// the referenced file at time of execution. The ethereumData will need to be
    /// "rehydrated" with the callData for signature validation to pass.
    public var callDataFileId: FileId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets a file ID to find the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    ///
    /// For large transactions (for example contract create) this should be used to
    /// set the FileId of an HFS file containing the callData
    /// of the ethereumData. The data in the ethereumData will be re-written with
    /// the callData element as a zero length string with the original contents in
    /// the referenced file at time of execution. The ethereumData will need to be
    /// "rehydrated" with the callData for signature validation to pass.
    ///
    @discardableResult
    public func callDataFileId(_ callDataFileId: FileId) -> Self {
        self.callDataFileId = callDataFileId

        return self
    }

    /// The maximum amount that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    public var maxGasAllowanceHbar: Hbar {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the maximum amount that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    @discardableResult
    public func maxGasAllowanceHbar(_ maxGasAllowanceHbar: Hbar) -> Self {
        self.maxGasAllowanceHbar = maxGasAllowanceHbar

        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try callDataFileId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_SmartContractServiceAsyncClient(channel: channel).callEthereum(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .ethereumTransaction(toProtobuf())
    }
}

extension EthereumTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_EthereumTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.ethereumData = ethereumData ?? Data()
            callDataFileId?.toProtobufInto(&proto.callData)
            proto.maxGasAllowance = maxGasAllowanceHbar.toTinybars()
        }
    }
}
