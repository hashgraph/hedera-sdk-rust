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

import CHedera
import Foundation
import GRPC
import NIOCore
import SwiftProtobuf

/// Managed client for use on the Hedera network.
public final class Client {
    internal let eventLoop: NIOCore.EventLoopGroup = PlatformSupport.makeEventLoopGroup(loopCount: 1)
    internal let network: Network
    internal let mirrorNetwork: MirrorNetwork

    private init(network: Network, mirrorNetwork: MirrorNetwork, ledgerId: LedgerId) {
        self.network = network
        self.mirrorNetwork = mirrorNetwork
        self.ledgerId = ledgerId
    }

    public var ledgerId: LedgerId?
    internal var `operator`: Operator?
    internal var autoValidateChecksums: Bool = false
    internal var maxTransactionFee: Hbar? = nil

    internal func randomNodeIds() -> [AccountId] {
        let nodeIds = network.healthyNodeIds()

        let nodeSampleAmount = (nodeIds.count + 2) / 3
        let nodeIdIndecies = randomIndexes(upTo: nodeIds.count, amount: nodeSampleAmount)

        return nodeIdIndecies.map { nodeIds[$0] }
    }

    /// Construct a Hedera client pre-configured for mainnet access.
    public static func forMainnet() -> Self {
        Self(network: .mainnet(), mirrorNetwork: .mainnet(), ledgerId: .mainnet)
    }

    /// Construct a Hedera client pre-configured for testnet access.
    public static func forTestnet() -> Self {
        Self(network: .testnet(), mirrorNetwork: .testnet(), ledgerId: .mainnet)
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    public static func forPreviewnet() -> Self {
        Self(network: .previewnet(), mirrorNetwork: .previewnet(), ledgerId: .mainnet)
    }

    // wish I could write `init(for name: String)`
    public static func forName(_ name: String) throws -> Self {
        switch name {
        case "mainnet":
            return .forMainnet()

        case "testnet":
            return .forTestnet()

        case "previewnet":
            return .forPreviewnet()

        default:
            throw HError(kind: .basicParse, description: "Unknown network name \(name)")
        }
    }

    /// Sets the account that will, by default, be paying for transactions and queries built with
    /// this client.
    @discardableResult
    public func setOperator(_ accountId: AccountId, _ privateKey: PrivateKey) -> Self {
        `operator` = Operator(accountId: accountId, signer: privateKey)

        return self
    }

    public func ping(_ nodeAccountId: AccountId) async throws {
        _ = try await AccountBalanceQuery(accountId: nodeAccountId).nodeAccountIds([nodeAccountId]).execute(self)
    }

    public func ping(_ nodeAccountId: AccountId, _ timeout: TimeInterval) async throws {
        _ = try await AccountBalanceQuery(accountId: nodeAccountId).nodeAccountIds([nodeAccountId]).execute(
            self, timeout)
    }

    public func pingAll() async throws {
        try await withThrowingTaskGroup(of: Void.self) { group in
            for node in self.network.nodes {
                group.addTask {
                    try await self.ping(node)
                }

                try await group.waitForAll()
            }
        }
    }

    public func pingAll(_ timeout: TimeInterval) async throws {
        try await withThrowingTaskGroup(of: Void.self) { group in
            for node in self.network.nodes {
                group.addTask {
                    try await self.ping(node, timeout)
                }

                try await group.waitForAll()
            }
        }
    }

    @discardableResult
    public func setLedgerId(_ ledgerId: LedgerId?) -> Self {
        self.ledgerId = ledgerId

        return self
    }

    @discardableResult
    public func setAutoValidateChecksums(_ autoValidateChecksums: Bool) -> Self {
        self.autoValidateChecksums = autoValidateChecksums
        
        return self
    }

    public func isAutoValidateChecksumsEnabled() -> Bool {
        autoValidateChecksums
    }

    internal func generateTransactionId() -> TransactionId? {
        self.operator?.generateTransactionId()
    }
}
