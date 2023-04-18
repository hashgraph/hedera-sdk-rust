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

public final class EthereumFlow {
    private static let maxEthereumDataSize: Int = 5120

    public var ethereumData: EthereumData?

    @discardableResult
    public func ethereumData(_ data: Data) throws -> Self {
        ethereumData = try .init(rlpBytes: data)

        return self
    }

    public var maxGasAllowance: Hbar?

    @discardableResult
    public func maxGasAllowance(_ maxGasAllowance: Hbar) -> Self {
        self.maxGasAllowance = maxGasAllowance

        return self
    }

    public func execute(_ client: Client, _ timeoutPerTansaction: TimeInterval? = nil) async throws
        -> TransactionResponse
    {
        guard var ethereumData = ethereumData else {
            // todo: replace this with a real error?
            fatalError("ethereumData must be set before calling `EthereumFlow.execute`")
        }

        var ethereumDataBytes = ethereumData.toBytes()

        let ethereumTransaction = EthereumTransaction()

        if let maxGasAllowance = maxGasAllowance {
            ethereumTransaction.maxGasAllowanceHbar(maxGasAllowance)
        }

        if ethereumDataBytes.count <= Self.maxEthereumDataSize {
            return try await ethereumTransaction.ethereumData(ethereumDataBytes).execute(client, timeoutPerTansaction)
        }

        let callData = ethereumData.callData
        ethereumData.callData = Data()

        let fileId = try await Self.createFile(client, callData, timeoutPerTansaction)

        ethereumDataBytes = ethereumData.toBytes()

        ethereumTransaction.callDataFileId(fileId).ethereumData(ethereumDataBytes)

        return try await ethereumTransaction.execute(client, timeoutPerTansaction)
    }

    private static func splitCallData(_ callData: Data) -> (fileCreate: Data, fileAppend: Data?) {
        let fileAppendDefaultChunkSize: Int = 4096

        if let data = callData.splitAt(fileAppendDefaultChunkSize) {
            return (Data(data.0), Data(data.1))
        }

        return (callData, nil)
    }

    private static func createFile(_ client: Client, _ callData: Data, _ timeoutPerTansaction: TimeInterval? = nil)
        async throws -> FileId
    {
        let data = splitCallData(callData)

        let fileId = try await FileCreateTransaction()
            .contents(data.fileCreate)
            .execute(client, timeoutPerTansaction)
            .getReceiptQuery()
            .execute(client, timeoutPerTansaction)
            .fileId!

        if let fileAppendData = data.fileAppend {
            // note: file append waits for receipts so we don't need to ourself
            _ = try await FileAppendTransaction()
                .fileId(fileId)
                .contents(fileAppendData)
                .executeAll(client, timeoutPerTansaction)
        }

        return fileId
    }
}
