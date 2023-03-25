/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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
import SwiftProtobuf

internal protocol Execute {
    associatedtype GrpcRequest: SwiftProtobuf.Message
    associatedtype GrpcResponse: SwiftProtobuf.Message
    associatedtype Context
    associatedtype Response

    /// The _explicit_ nodes that this request will be submitted to.
    var nodeAccountIds: [AccountId]? { get }

    var explicitTransactionId: TransactionId? { get }

    var requiresTransactionId: Bool { get }

    /// Check whether to retry for a given pre-check status.
    func shouldRetryPrecheck(forStatus status: Status) -> Bool

    /// Check whether we should retry an otherwise successful response.
    func shouldRetry(forResponse response: GrpcResponse) -> Bool

    /// Create a new request for execution.
    ///
    /// A created request is cached per node until any request returns
    /// `TransactionExpired`; in which case, the request cache is cleared.
    func makeRequest(_ transactionId: TransactionId?, _ nodeAccountId: AccountId) throws -> (GrpcRequest, Context)

    func execute(_ channel: GRPCChannel, _ request: GrpcRequest) async throws -> GrpcResponse

    /// Create a response from the GRPC response and the saved transaction
    /// and node account ID from the successful request.
    func makeResponse(
        _ response: GrpcResponse,
        _ context: Context,
        _ nodeAccountId: AccountId,
        _ transactionId: TransactionId?
    ) throws -> Response

    func makeErrorPrecheck(_ status: Status, _ transactionId: TransactionId?) -> HError

    /// Gets pre-check status from the GRPC response.
    static func responsePrecheckStatus(_ response: GrpcResponse) throws -> Int32
}

extension Execute {
    internal func shouldRetryPrecheck(forStatus status: Status) -> Bool {
        false
    }

    internal func shouldRetry(forResponse response: GrpcResponse) -> Bool {
        false
    }
}

// swiftlint:disable:next function_body_length
internal func executeAny<E: Execute & ValidateChecksums>(_ client: Client, _ executable: E, _ timeout: TimeInterval?)
    async throws -> E.Response
{
    let timeout = timeout ?? LegacyExponentialBackoff.defaultMaxElapsedTime

    var backoff = LegacyExponentialBackoff(maxElapsedTime: .limited(timeout))
    var lastError: HError?

    if client.isAutoValidateChecksumsEnabled() {
        try executable.validateChecksums(on: client)
    }

    let explicitTransactionId = executable.explicitTransactionId
    var transactionId =
        executable.requiresTransactionId ? (explicitTransactionId ?? client.generateTransactionId()) : nil

    let explicitNodeIndexes = try executable.nodeAccountIds.map { try client.network.nodeIndexesForIds($0) }

    while true {
        let randomNodeIndexes = randomNodeIndexes(client: client, explicitNodeIndexes: explicitNodeIndexes)
        inner: for await nodeIndex in randomNodeIndexes {
            let (nodeAccountId, channel) = client.network.channel(for: nodeIndex)
            let (request, context) = try executable.makeRequest(transactionId, nodeAccountId)
            let response: E.GrpcResponse

            do {
                response = try await executable.execute(channel, request)
            } catch let error as GRPCStatus {
                switch error.code {
                case .unavailable, .resourceExhausted:
                    // NOTE: this is an "unhealthy" node
                    // todo: mark unhealthy
                    // try the next node in our allowed list, immediately
                    lastError = HError(
                        kind: .grpcStatus(status: Int32(error.code.rawValue)),
                        description: error.description
                    )
                    continue inner

                case let code:
                    throw HError(
                        kind: .grpcStatus(status: Int32(code.rawValue)),
                        description: error.description
                    )
                }
            }

            let rawPrecheckStatus = try E.responsePrecheckStatus(response)
            let precheckStatus = Status(rawValue: rawPrecheckStatus)

            switch precheckStatus {
            case .ok where executable.shouldRetry(forResponse: response):
                lastError = executable.makeErrorPrecheck(precheckStatus, transactionId)
                break inner

            case .ok:
                return try executable.makeResponse(response, context, nodeAccountId, transactionId)

            case .busy, .platformNotActive:
                // NOTE: this is a "busy" node
                // try the next node in our allowed list, immediately
                lastError = executable.makeErrorPrecheck(precheckStatus, transactionId)

            case .transactionExpired where explicitTransactionId == nil:
                // the transaction that was generated has since expired
                // re-generate the transaction ID and try again, immediately
                lastError = executable.makeErrorPrecheck(precheckStatus, transactionId)
                transactionId = client.generateTransactionId()
                continue inner

            case .UNRECOGNIZED(let value):
                throw HError(
                    kind: .responseStatusUnrecognized,
                    description: "response status \(value) unrecognized"
                )

            case let status where executable.shouldRetryPrecheck(forStatus: precheckStatus):
                // conditional retry on pre-check should back-off and try again
                lastError = executable.makeErrorPrecheck(status, transactionId)
                break inner

            default:
                throw executable.makeErrorPrecheck(precheckStatus, transactionId)
            }

            guard let timeout = backoff.next() else {
                throw HError.timedOut(String(describing: lastError))
            }

            try await Task.sleep(nanoseconds: UInt64(timeout * 1e9))
        }
    }
}

internal func randomIndexes(upTo count: Int, amount: Int) -> [Int] {
    var elements = Array(0..<count)

    var output: [Int] = []

    for _ in 0..<amount {
        let index = Int.random(in: 0..<elements.count)
        let item = elements.remove(at: index)
        output.append(item)
    }

    return output
}

// ugh
private struct NodeIndexesGeneratorMap: AsyncSequence, AsyncIteratorProtocol {
    fileprivate typealias Element = Int
    fileprivate typealias AsyncIterator = Self

    fileprivate func makeAsyncIterator() -> AsyncIterator {
        self
    }

    fileprivate init(indecies: [Int], passthrough: Bool, client: Client) {
        // `popLast` is generally faster, sooo...
        self.source = indecies.reversed()
        self.passthrough = passthrough
        self.client = client
    }

    fileprivate var source: [Int]
    fileprivate let passthrough: Bool
    fileprivate let client: Client

    mutating func next() async -> Int? {
        func recursePing(client: Client, nodeIndex: Int) async -> Bool {
            do {
                try await (client.ping(client.network.nodes[nodeIndex]))
                return true
            } catch {
                return false
            }
        }

        guard let current = source.popLast() else {
            return nil
        }

        if passthrough {
            return current
        }

        if client.network.nodeRecentlyPinged(current, now: .now) {
            return current
        }

        if await recursePing(client: client, nodeIndex: current) {
            return current
        }

        return nil
    }
}

// this is made complicated by the fact that we *might* have to ping nodes (and we really want to not do that if at all possible)
private func randomNodeIndexes(client: Client, explicitNodeIndexes: [Int]?) -> NodeIndexesGeneratorMap {
    let nodeIndexes = explicitNodeIndexes ?? client.network.healthyNodeIndexes()

    let nodeSampleAmount = (explicitNodeIndexes != nil) ? nodeIndexes.count : (nodeIndexes.count + 2) / 3

    let randomNodeIndexes = randomIndexes(upTo: nodeIndexes.count, amount: nodeSampleAmount).map { nodeIndexes[$0] }

    return NodeIndexesGeneratorMap(indecies: randomNodeIndexes, passthrough: explicitNodeIndexes != nil, client: client)
}
