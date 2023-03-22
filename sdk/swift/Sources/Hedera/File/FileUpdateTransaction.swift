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

import Foundation
import GRPC
import HederaProtobufs
import SwiftProtobuf

/// Modify the metadata and/or the contents of a file.
///
/// If a field is not set in the transaction body, the
/// corresponding file attribute will be unchanged.
///
public final class FileUpdateTransaction: Transaction {
    internal init(
        fileId: FileId? = nil,
        fileMemo: String = "",
        keys: KeyList? = nil,
        contents: Data = Data(),
        autoRenewPeriod: Duration? = nil,
        autoRenewAccountId: AccountId? = nil,
        expirationTime: Timestamp? = nil
    ) {
        self.fileId = fileId
        self.fileMemo = fileMemo
        self.keys = keys
        self.contents = contents
        self.autoRenewPeriod = autoRenewPeriod
        self.autoRenewAccountId = autoRenewAccountId
        self.expirationTime = expirationTime

        super.init()
    }

    /// Create a new `FileUpdateTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    public required init(from decoder: Swift.Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        fileId = try container.decodeIfPresent(.fileId)
        fileMemo = try container.decodeIfPresent(.fileMemo) ?? ""
        keys = try container.decodeIfPresent(.keys)
        contents = try container.decodeIfPresent(.contents).map(Data.base64Encoded) ?? Data()
        autoRenewPeriod = try container.decodeIfPresent(.autoRenewPeriod)
        autoRenewAccountId = try container.decodeIfPresent(.autoRenewAccountId)
        expirationTime = try container.decodeIfPresent(.expirationTime)

        try super.init(from: decoder)
    }

    /// The file ID which is being updated in this transaction.
    public var fileId: FileId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the file ID which is being updated in this transaction.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The memo associated with the file.
    public var fileMemo: String = "" {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the memo associated with the file.
    @discardableResult
    public func fileMemo(_ fileMemo: String) -> Self {
        self.fileMemo = fileMemo

        return self
    }

    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    public var keys: KeyList? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the keys for this file.
    ///
    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    ///
    @discardableResult
    public func keys(_ keys: KeyList) -> Self {
        self.keys = keys

        return self
    }

    /// The bytes that are to be the contents of the file.
    public var contents: Data = Data() {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the bytes that are to be the contents of the file.
    @discardableResult
    public func contents(_ contents: Data) -> Self {
        self.contents = contents

        return self
    }

    /// The auto renew period for this file.
    ///
    /// > Warning: This not supported on any hedera network at this time.
    public var autoRenewPeriod: Duration? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Set the auto renew period for this file.
    ///
    /// > Warning: This not supported on any hedera network at this time.
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The account to be used at the files's expiration time to extend the
    /// life of the file.
    ///
    /// > Warning: This not supported on any hedera network at this time.
    public var autoRenewAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account to be used at the files's expiration time to extend the
    /// life of the file.
    ///
    /// > Warning: This not supported on any hedera network at this time.
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The time at which this file should expire.
    public var expirationTime: Timestamp? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the time at which this file should expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
        case fileMemo
        case keys
        case contents
        case expirationTime
        case autoRenewPeriod
        case autoRenewAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(fileId, forKey: .fileId)
        try container.encode(fileMemo, forKey: .fileMemo)
        try container.encodeIfPresent(keys, forKey: .keys)
        try container.encode(contents.base64EncodedString(), forKey: .contents)
        try container.encodeIfPresent(expirationTime, forKey: .expirationTime)
        try container.encodeIfPresent(autoRenewPeriod, forKey: .autoRenewPeriod)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try autoRenewAccountId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_FileServiceAsyncClient(channel: channel).updateFile(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .fileUpdate(toProtobuf())
    }
}

extension FileUpdateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_FileUpdateTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            fileId?.toProtobufInto(&proto.fileID)
            proto.memo = Google_Protobuf_StringValue(fileMemo)
            keys?.toProtobufInto(&proto.keys)
            proto.contents = contents
            expirationTime?.toProtobufInto(&proto.expirationTime)
        }
    }
}

extension FileUpdateTransaction: ToSchedulableTransactionData {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .fileUpdate(toProtobuf())
    }
}
