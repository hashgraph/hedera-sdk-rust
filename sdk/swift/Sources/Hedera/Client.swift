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

/// Managed client for use on the Hedera network.
public final class Client: Sendable {
    internal let eventLoop: NIOCore.EventLoopGroup

    internal let ptr: OpaquePointer
    private let mirrorNetwork: MirrorNetwork
    internal let network: Network

    private init(
        unsafeFromPtr ptr: OpaquePointer, network: Network, mirrorNetwork: MirrorNetwork,
        _ eventLoop: NIOCore.EventLoopGroup
    ) {
        self.eventLoop = eventLoop
        self.ptr = ptr
        self.mirrorNetwork = mirrorNetwork
        self.network = network
    }

    /// Note: this operation is O(n)
    private var nodes: [AccountId] {
        var ids: UnsafeMutablePointer<HederaAccountId>?

        let len = hedera_client_get_nodes(ptr, &ids)

        let nodes = UnsafeMutableBufferPointer(start: ids, count: len).map { AccountId(unsafeFromCHedera: $0) }

        hedera_account_id_array_free(ids, len)

        return nodes
    }

    internal var mirrorChannel: GRPCChannel { mirrorNetwork.channel }

    internal func randomNodeIds() -> [AccountId] {
        var ids: UnsafeMutablePointer<HederaAccountId>?

        let len = hedera_client_get_random_node_ids(ptr, &ids)

        let nodes = UnsafeMutableBufferPointer(start: ids, count: len).map { AccountId(unsafeFromCHedera: $0) }

        hedera_account_id_array_free(ids, len)

        return nodes
    }

    internal var `operator`: Operator? {
        var skPtr: OpaquePointer?
        var accountId = HederaAccountId()

        if hedera_client_get_operator(ptr, &accountId, &skPtr) {
            return Operator(
                accountId: AccountId(unsafeFromCHedera: accountId),
                signer: .unsafeFromPtr(skPtr!)
            )
        } else {
            return nil
        }
    }

    internal var maxTransactionFee: Hbar? {
        let val = hedera_client_get_max_transaction_fee(ptr)

        if val == 0 {
            return nil
        }

        return .fromTinybars(Int64(bitPattern: val))
    }

    /// Construct a Hedera client pre-configured for mainnet access.
    public static func forMainnet() -> Self {
        let eventLoop = PlatformSupport.makeEventLoopGroup(loopCount: 1)
        return Self(
            unsafeFromPtr: hedera_client_for_mainnet(),
            network: .mainnet(eventLoop),
            mirrorNetwork: .mainnet(eventLoop),
            eventLoop
        )
    }

    /// Construct a Hedera client pre-configured for testnet access.
    public static func forTestnet() -> Self {
        let eventLoop = PlatformSupport.makeEventLoopGroup(loopCount: 1)
        return Self(
            unsafeFromPtr: hedera_client_for_testnet(),
            network: .testnet(eventLoop),
            mirrorNetwork: .testnet(eventLoop),
            eventLoop
        )
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    public static func forPreviewnet() -> Self {
        let eventLoop = PlatformSupport.makeEventLoopGroup(loopCount: 1)
        return Self(
            unsafeFromPtr: hedera_client_for_previewnet(),
            network: .previewnet(eventLoop),
            mirrorNetwork: .previewnet(eventLoop),
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
        accountId.unsafeWithCHedera { hAccountId in
            hedera_client_set_operator(ptr, hAccountId, privateKey.ptr)
        }

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

    @discardableResult
    public func setLedgerId(_ ledgerId: LedgerId?) -> Self {
        self.ledgerId = ledgerId

        return self
    }

    // note: matches JS
    public var ledgerId: LedgerId? {
        get {
            var bytes: UnsafeMutablePointer<UInt8>?
            let count = hedera_client_get_ledger_id(ptr, &bytes)

            return bytes.map { LedgerId(Data(bytesNoCopy: $0, count: count, deallocator: .unsafeCHederaBytesFree)) }
        }

        set(value) {
            if let ledgerId = value {
                ledgerId.bytes.withUnsafeTypedBytes { ledgerIdPtr in
                    hedera_client_set_ledger_id(ptr, ledgerIdPtr.baseAddress, ledgerIdPtr.count)
                }
            } else {
                hedera_client_set_ledger_id(ptr, nil, 0)
            }
        }
    }

    fileprivate var autoValidateChecksums: Bool {
        get { hedera_client_get_auto_validate_checksums(ptr) }
        set(value) { hedera_client_set_auto_validate_checksums(ptr, value) }
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

    deinit {
        hedera_client_free(ptr)
    }
}
