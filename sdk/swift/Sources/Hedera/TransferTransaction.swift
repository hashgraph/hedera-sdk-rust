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

/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
public final class TransferTransaction: Transaction {
    // avoid scope collisions by nesting :/
    private struct Transfer: Codable, ValidateChecksums {
        let accountId: AccountId
        let amount: Int64
        let isApproval: Bool

        internal func validateChecksums(on ledgerId: LedgerId) throws {
            try accountId.validateChecksums(on: ledgerId)
        }
    }

    private struct TokenTransfer: Codable, ValidateChecksums {
        let tokenId: TokenId
        var transfers: [TransferTransaction.Transfer]
        var nftTransfers: [TransferTransaction.NftTransfer]
        var expectedDecimals: UInt32?

        internal func validateChecksums(on ledgerId: LedgerId) throws {
            try tokenId.validateChecksums(on: ledgerId)
            try transfers.validateChecksums(on: ledgerId)
            try nftTransfers.validateChecksums(on: ledgerId)
        }
    }

    private struct NftTransfer: Codable, ValidateChecksums {
        let senderAccountId: AccountId
        let receiverAccountId: AccountId
        let serial: UInt64
        let isApproval: Bool

        internal func validateChecksums(on ledgerId: LedgerId) throws {
            try senderAccountId.validateChecksums(on: ledgerId)
            try receiverAccountId.validateChecksums(on: ledgerId)
        }
    }

    private var transfers: [TransferTransaction.Transfer] = [] {
        willSet {
            ensureNotFrozen(fieldName: "transfers")
        }
    }

    private var tokenTransfers: [TransferTransaction.TokenTransfer] = [] {
        willSet {
            ensureNotFrozen(fieldName: "tokenTransfers")
        }
    }

    /// Create a new `TransferTransaction`.
    public override init() {
        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        transfers = try container.decodeIfPresent(.transfers) ?? []
        tokenTransfers = try container.decodeIfPresent(.tokenTransfers) ?? []

        try super.init(from: decoder)
    }

    /// Add a non-approved hbar transfer to the transaction.
    @discardableResult
    public func hbarTransfer(_ accountId: AccountId, _ amount: Int64) -> Self {
        doHbarTransfer(accountId, amount, false)
    }

    /// Add an approved hbar transfer to the transaction.
    @discardableResult
    public func approvedHbarTransfer(_ accountId: AccountId, _ amount: Int64) -> Self {
        doHbarTransfer(accountId, amount, true)
    }

    /// Add a non-approved token transfer to the transaction.
    @discardableResult
    public func tokenTransfer(_ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64) -> Self {
        doTokenTransfer(tokenId, accountId, amount, false, nil)
    }

    /// Add an approved token transfer to the transaction.
    @discardableResult
    public func approvedTokenTransfer(_ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64) -> Self {
        doTokenTransfer(tokenId, accountId, amount, true, nil)
    }

    /// Add a non-approved token transfer with decimals to the transaction.
    @discardableResult
    public func tokenTransferWithDecimals(
        _ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64, _ expectedDecimals: UInt32
    ) -> Self {
        doTokenTransfer(tokenId, accountId, amount, false, expectedDecimals)
    }

    /// Add an approved token transfer with decimals to the transaction.
    @discardableResult
    public func approvedTokenTransferWithDecimals(
        _ tokenId: TokenId, _ accountId: AccountId, _ amount: Int64, _ expectedDecimals: UInt32
    ) -> Self {
        doTokenTransfer(tokenId, accountId, amount, false, expectedDecimals)
    }

    /// Add a non-approved nft transfer to the transaction.
    @discardableResult
    public func nftTransfer(_ nftId: NftId, _ senderAccountId: AccountId, _ receiverAccountId: AccountId)
        -> Self
    {
        doNftTransfer(nftId, senderAccountId, receiverAccountId, false)
    }

    /// Add an approved nft transfer to the transaction.
    @discardableResult
    public func approvedNftTransfer(
        _ nftId: NftId, _ senderAccountId: AccountId, _ receiverAccountId: AccountId
    ) -> Self {
        doNftTransfer(nftId, senderAccountId, receiverAccountId, true)
    }

    private func doHbarTransfer(
        _ accountId: AccountId,
        _ amount: Int64,
        _ approved: Bool
    ) -> Self {
        transfers.append(Transfer(accountId: accountId, amount: amount, isApproval: approved))

        return self
    }

    private func doTokenTransfer(
        _ tokenId: TokenId,
        _ accountId: AccountId,
        _ amount: Int64,
        _ approved: Bool,
        _ expectedDecimals: UInt32?
    ) -> Self {
        let transfer = Transfer(accountId: accountId, amount: amount, isApproval: approved)

        if var tokenTransfer = tokenTransfers.first(where: { (tokenTransfer) in tokenTransfer.tokenId == tokenId }) {
            tokenTransfer.expectedDecimals = expectedDecimals
            tokenTransfer.transfers.append(transfer)
        } else {
            tokenTransfers.append(
                TokenTransfer(
                    tokenId: tokenId,
                    transfers: [transfer],
                    nftTransfers: [],
                    expectedDecimals: expectedDecimals
                ))
        }

        return self
    }

    private func doNftTransfer(
        _ nftId: NftId,
        _ senderAccountId: AccountId,
        _ receiverAccountId: AccountId,
        _ approved: Bool
    ) -> Self {
        let transfer = NftTransfer(
            senderAccountId: senderAccountId,
            receiverAccountId: receiverAccountId,
            serial: nftId.serial,
            isApproval: approved
        )

        if var tokenTransfer = tokenTransfers.first(where: { (transfer) in transfer.tokenId == nftId.tokenId }) {
            tokenTransfer.nftTransfers.append(transfer)
        } else {
            tokenTransfers.append(
                TokenTransfer(
                    tokenId: nftId.tokenId,
                    transfers: [],
                    nftTransfers: [transfer],
                    expectedDecimals: nil
                ))
        }

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case transfers
        case tokenTransfers
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(transfers, forKey: .transfers)
        try container.encode(tokenTransfers, forKey: .tokenTransfers)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try transfers.validateChecksums(on: ledgerId)
        try tokenTransfers.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
