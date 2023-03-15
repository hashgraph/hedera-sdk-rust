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

internal struct PingQuery {
    internal init(nodeAccountId: AccountId) {
        self.nodeAccountId = nodeAccountId
    }

    private let nodeAccountId: AccountId

    internal func execute(_ client: Client, timeout: TimeInterval? = nil) async throws {
        try await executeAny(client, self, timeout)
    }
}

extension PingQuery: ValidateChecksums {
    func validateChecksums(on ledgerId: LedgerId) throws {
        try nodeAccountId.validateChecksums(on: ledgerId)
    }
}

extension PingQuery: Execute {
    typealias GrpcRequest = Proto_Query

    typealias GrpcResponse = Proto_Response

    typealias Context = ()

    typealias Response = ()

    var nodeAccountIds: [AccountId]? {
        [nodeAccountId]
    }

    var explicitTransactionId: TransactionId? { nil }

    var requiresTransactionId: Bool { false }

    func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (Proto_Query, ()) {
        let header = Proto_QueryHeader.with { $0.responseType = .answerOnly }

        assert(nodeAccountId == self.nodeAccountId)

        let query = Proto_Query.with { proto in 
            proto.query = .cryptogetAccountBalance(.with { proto in 
                proto.accountID = nodeAccountId.toProtobuf()
                proto.header = header
            })
        }

        return (query, ())
    }

    func execute(_ channel: GRPC.GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        print("hello from ping query")
        return try await Proto_CryptoServiceAsyncClient(channel: channel).cryptoGetBalance(request)
    }

    func makeResponse(
        _ response: Proto_Response, _ context: (), _ nodeAccountId: AccountId, _ transactionId: TransactionId?
    ) throws {}

    func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError {
        HError(kind: .queryNoPaymentPreCheckStatus(status: status), description: "query with no payment transaction failed pre-check with status \(status)")
    }

    static func responsePrecheckStatus(_ response: HederaProtobufs.Proto_Response) throws -> Int32 {
        try Int32(response.header().nodeTransactionPrecheckCode.rawValue)
    }
}
