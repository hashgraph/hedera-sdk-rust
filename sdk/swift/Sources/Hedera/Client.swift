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

/// Managed client for use on the Hedera network.
public final class Client {
    internal let ptr: OpaquePointer

    private init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }

    deinit {
        hedera_client_free(ptr)
    }

    /// Construct a Hedera client pre-configured for testnet access.
    public static func forTestnet() -> Client {
        Client(hedera_client_for_testnet())
    }

    /// Sets the account that will, by default, be paying for transactions and queries built with
    /// this client.
    public func setOperator(_ accountId: AccountId, _ privateKey: PrivateKey) {
        hedera_client_set_operator(
            ptr, accountId.shard, accountId.realm, accountId.num, privateKey.ptr)
    }
}
