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

public final class ContractInfo: Codable {
    /// ID of the contract instance, in the format used by transactions.
    public let contractId: ContractId

    /// ID of the cryptocurrency account owned by the contract instance,
    /// in the format used in transactions.
    public let accountId: AccountId

    /// ID of both the contract instance and the cryptocurrency account owned by the contract
    /// instance, in the format used by Solidity.
    public let contractAccountId: String

    /// The admin key of the contract instance.
    public let adminKey: Key?

    /// The current time at which this contract instance (and its account) is set to expire.
    public let expirationTime: Timestamp?

    /// The auto renew period for this contract instance.
    public let autoRenewPeriod: Duration?

    /// Number of bytes of storage being used by this instance.
    public let storage: UInt64

    /// The memo associated with the contract.
    public let contractMemo: String

    /// The current balance, in tinybars.
    public let balance: Hbar

    /// Whether the contract has been deleted.
    public let isDeleted: Bool

    /// ID of the an account to charge for auto-renewal of this contract.
    public let autoRenewAccountId: AccountId?

    /// The maximum number of tokens that a contract can be implicitly associated with.
    public let maxAutomaticTokenAssociations: UInt32

    public let ledgerId: LedgerId

    /// Staking metadata for this contract.
    public let stakingInfo: StakingInfo

    public static func fromBytes(_ bytes: Data) throws -> Self {
        let json: String = try bytes.withUnsafeTypedBytes { pointer in
            var ptr: UnsafeMutablePointer<CChar>? = nil
            let err = hedera_contract_info_from_bytes(
                pointer.baseAddress,
                pointer.count,
                &ptr
            )

            if err != HEDERA_ERROR_OK {
                throw HError(err)!
            }

            return String(hString: ptr!)
        }

        return try JSONDecoder().decode(Self.self, from: json.data(using: .utf8)!)
    }

    private func toBytesInner() throws -> Data {
        let jsonBytes = try JSONEncoder().encode(self)
        let json = String(data: jsonBytes, encoding: .utf8)!
        var buf: UnsafeMutablePointer<UInt8>?
        var bufSize: Int = 0
        let err = hedera_contract_info_to_bytes(json, &buf, &bufSize)

        if err != HEDERA_ERROR_OK {
            throw HError(err)!
        }

        return Data(bytesNoCopy: buf!, count: bufSize, deallocator: Data.unsafeCHederaBytesFree)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toBytesInner()
    }
}
