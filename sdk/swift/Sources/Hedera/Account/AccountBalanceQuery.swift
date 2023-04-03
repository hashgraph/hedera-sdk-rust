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

import GRPC
import HederaProtobufs

/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than `AccountInfoQuery`, which returns the balance plus
/// additional information.
public final class AccountBalanceQuery: Query<AccountBalance> {
    /// Create a new `AccountBalanceQuery`.
    public init(
        accountId: AccountId? = nil,
        contractId: ContractId? = nil
    ) {
        self.accountId = accountId
        self.contractId = contractId
    }

    /// The account ID for which information is requested.
    public var accountId: AccountId?

    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with `contractId`.
    @discardableResult
    public func accountId(_ accountId: AccountId) -> Self {
        self.accountId = accountId
        contractId = nil

        return self
    }

    /// The contract ID for which information is requested.
    public var contractId: ContractId?

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with `accountId`.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId
        accountId = nil

        return self
    }

    internal override var requiresPayment: Bool { false }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.cryptogetAccountBalance = .with { proto in
                proto.header = header
                if let accountId = self.accountId {
                    proto.accountID = accountId.toProtobuf()
                }

                if let contractId = self.contractId {
                    proto.contractID = contractId.toProtobuf()
                }
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_CryptoServiceAsyncClient(channel: channel).cryptoGetBalance(request)
    }

    internal override func makeQueryResponse(_ response: Proto_Response.OneOf_Response) throws -> Response {
        guard case .cryptogetAccountBalance(let proto) = response else {
            throw HError.fromProtobuf("unexpected \(response) received, expected `cryptogetAccountBalance`")
        }

        return try .fromProtobuf(proto)
    }

    public override func validateChecksums(on ledgerId: LedgerId) throws {
        try self.accountId?.validateChecksums(on: ledgerId)
        try self.contractId?.validateChecksums(on: ledgerId)

        try super.validateChecksums(on: ledgerId)
    }
}
