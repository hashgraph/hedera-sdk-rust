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

/// Managed client for use on the Hedera network.
public final class Client {
    internal let ptr: OpaquePointer

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    /// Note: this operation is O(n)
    private var nodes: [AccountId] {
        var ids: UnsafeMutablePointer<HederaAccountId>?

        let len = hedera_client_get_nodes(ptr, &ids)

        let nodes = UnsafeMutableBufferPointer(start: ids, count: len).map { AccountId(unsafeFromCHedera: $0) }

        hedera_account_id_array_free(ids, len)

        return nodes
    }

    /// Construct a Hedera client pre-configured for mainnet access.
    public static func forMainnet() -> Self {
        Self(hedera_client_for_testnet())
    }

    /// Construct a Hedera client pre-configured for testnet access.
    public static func forTestnet() -> Self {
        Self(hedera_client_for_testnet())
    }

    /// Construct a Hedera client pre-configured for previewnet access.
    public static func forPreviewnet() -> Self {
        Self(hedera_client_for_previewnet())
    }

    // wish I could write `init(for name: String)`
    public static func forName(_ name: String) throws -> Self {
        switch name {
        case "mainnet":
            return Self.forMainnet()

        case "testnet":
            return Self.forTestnet()

        case "previewnet":
            return Self.forPreviewnet()

        default:
            throw HError(kind: .basicParse, description: "Unknown network name \(name)")
        }
    }

    /// Sets the account that will, by default, be paying for transactions and queries built with
    /// this client.
    @discardableResult
    public func setOperator(_ accountId: AccountId, _ privateKey: PrivateKey) -> Self {
        hedera_client_set_operator(
            ptr, accountId.shard, accountId.realm, accountId.num, privateKey.ptr)

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

    @discardableResult
    public func setAutoValidateChecksums(_ autoValidateChecksums: Bool) -> Self {
        hedera_client_set_auto_validate_checksums(ptr, autoValidateChecksums)

        return self
    }

    public func isAutoValidateChecksumsEnabled() -> Bool {
        hedera_client_get_auto_validate_checksums(ptr)
    }

    deinit {
        hedera_client_free(ptr)
    }
}
