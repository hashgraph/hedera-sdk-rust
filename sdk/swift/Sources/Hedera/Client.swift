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

import Atomics
import Foundation
import GRPC
import NIOConcurrencyHelpers
import NIOCore

// safety: Atomics just forgot to put the conformance.
private struct AtomicBool: @unchecked Sendable {
    init(_ value: Bool) {
        self.inner = .init(value)
    }

    fileprivate let inner: ManagedAtomic<Bool>
}

// safety: Atomics just forgot to put the conformance.
private struct AtomicInt64: @unchecked Sendable {
    init(_ value: Int64) {
        self.inner = .init(value)
    }

    fileprivate let inner: ManagedAtomic<Int64>
}

/// Managed client for use on the Hedera network.
public final class Client: Sendable {
    internal let eventLoop: NIOCore.EventLoopGroup

    private let mirrorNetwork: MirrorNetwork
    internal let network: Network
    private let operatorInner: NIOLockedValueBox<Operator?>
    private let autoValidateChecksumsInner: AtomicBool
    private let maxTransactionFeeInner: AtomicInt64

    private init(
        network: Network,
        mirrorNetwork: MirrorNetwork,
        ledgerId: LedgerId?,
        _ eventLoop: NIOCore.EventLoopGroup
    ) {
        self.eventLoop = eventLoop
        // self.ptr = ptr
        self.mirrorNetwork = mirrorNetwork
        self.network = network
        self.operatorInner = .init(nil)
        self.ledgerIdInner = .init(ledgerId)
        self.autoValidateChecksumsInner = .init(false)
        self.maxTransactionFeeInner = .init(0)
    }

    /// Note: this operation is O(n)
    private var nodes: [AccountId] {
        network.nodes
    }

    internal var mirrorChannel: GRPCChannel { mirrorNetwork.channel }

    internal func randomNodeIds() -> [AccountId] {
        let nodeIds = self.network.healthyNodeIds()

        let nodeSampleAmount = (nodeIds.count + 2) / 3

        let nodeIdIndecies = randomIndexes(upTo: nodeIds.count, amount: nodeSampleAmount)
        return nodeIdIndecies.map { nodeIds[$0] }
    }

    internal var `operator`: Operator? {
        return operatorInner.withLockedValue { $0 }
    }

    internal var maxTransactionFee: Hbar? {
        let value = maxTransactionFeeInner.inner.load(ordering: .relaxed)

        guard value != 0 else {
            return nil
        }

        return .fromTinybars(value)
    }

    /// Construct a Hedera client pre-configured for mainnet access.
    public static func forMainnet() -> Self {
        let eventLoop = PlatformSupport.makeEventLoopGroup(loopCount: 1)
        return Self(
            network: .mainnet(eventLoop),
            mirrorNetwork: .mainnet(eventLoop),
            ledgerId: .mainnet,
            eventLoop
        )
    }

    /// Construct a Hedera client pre-configured for testnet access.
    public static func forTestnet() -> Self {
        let eventLoop = PlatformSupport.makeEventLoopGroup(loopCount: 1)
        return Self(
            network: .testnet(eventLoop),
            mirrorNetwork: .testnet(eventLoop),
            ledgerId: .testnet,
            eventLoop
        )
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    public static func forPreviewnet() -> Self {
        let eventLoop = PlatformSupport.makeEventLoopGroup(loopCount: 1)
        return Self(
            network: .previewnet(eventLoop),
            mirrorNetwork: .previewnet(eventLoop),
            ledgerId: .previewnet,
            eventLoop
        )
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
        operatorInner.withLockedValue { $0 = .init(accountId: accountId, signer: privateKey) }

        return self
    }

    public func ping(_ nodeAccountId: AccountId) async throws {
        try await PingQuery(nodeAccountId: nodeAccountId).execute(self)
    }

    public func ping(_ nodeAccountId: AccountId, _ timeout: TimeInterval) async throws {
        try await PingQuery(nodeAccountId: nodeAccountId).execute(self, timeout: timeout)
    }

    public func pingAll() async throws {
        try await withThrowingTaskGroup(of: Void.self) { group in
            for node in self.nodes {
                group.addTask {
                    try await self.ping(node)
                }

                try await group.waitForAll()
            }
        }
    }

    public func pingAll(_ timeout: TimeInterval) async throws {
        try await withThrowingTaskGroup(of: Void.self) { group in
            for node in self.nodes {
                group.addTask {
                    try await self.ping(node, timeout)
                }

                try await group.waitForAll()
            }
        }
    }

    private let ledgerIdInner: NIOLockedValueBox<LedgerId?>

    @discardableResult
    public func setLedgerId(_ ledgerId: LedgerId?) -> Self {
        self.ledgerId = ledgerId

        return self
    }

    // note: matches JS
    public var ledgerId: LedgerId? {
        get {
            ledgerIdInner.withLockedValue { $0 }
        }

        set(value) {
            ledgerIdInner.withLockedValue { $0 = value }
        }
    }

    fileprivate var autoValidateChecksums: Bool {
        get { self.autoValidateChecksumsInner.inner.load(ordering: .relaxed) }
        set(value) { self.autoValidateChecksumsInner.inner.store(value, ordering: .relaxed) }
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
        (self.operator?.accountId).map { .generateFrom($0) }
    }
}
