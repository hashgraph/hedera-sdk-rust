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

/// Response from ``AccountBalanceQuery``.
public final class AccountBalance: Codable {
    /// The account that is being referenced.
    public let accountId: AccountId

    /// Current balance of the referenced account.
    public let hbars: Hbar

    internal init(unsafeFromCHedera hedera: HederaAccountBalance) {
        accountId = AccountId(unsafeFromCHedera: hedera.id)
        hbars = Hbar.fromTinybars(hedera.hbars)
    }

    internal func unsafeWithCHedera<Result>(_ body: (HederaAccountBalance) throws -> Result) rethrows -> Result {
        try accountId.unsafeWithCHedera { hederaAccountId in
            try body(HederaAccountBalance(id: hederaAccountId, hbars: hbars.toTinybars()))
        }
    }

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try bytes.withUnsafeTypedBytes { pointer in
            var balance = HederaAccountBalance()

            try HError.throwing(error: hedera_account_balance_from_bytes(pointer.baseAddress, pointer.count, &balance))

            return Self(unsafeFromCHedera: balance)
        }
    }

    public func toBytes() -> Data {
        self.unsafeWithCHedera { hedera in
            var buf: UnsafeMutablePointer<UInt8>?
            let size = hedera_account_balance_to_bytes(hedera, &buf)

            return Data(bytesNoCopy: buf!, count: size, deallocator: .unsafeCHederaBytesFree)
        }
    }

    public func toString() -> String {
        String(describing: self)
    }
}
