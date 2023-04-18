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

/// Create a new file, containing the given contents.
public final class FileCreateTransaction: Transaction {
    /// Create a new `FileCreateTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_FileCreateTransactionBody) throws {
        fileMemo = data.memo
        keys = try .fromProtobuf(data.keys)
        contents = data.contents
        expirationTime = data.hasExpirationTime ? .fromProtobuf(data.expirationTime) : nil

        // hedera doesn't have these currently.
        autoRenewPeriod = nil
        autoRenewAccountId = nil

        try super.init(protobuf: proto)
    }

    /// The memo associated with the file.
    public var fileMemo: String = "" {
        willSet {
            ensureNotFrozen(fieldName: "fileMemo")
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
    public var keys: KeyList = [] {
        willSet {
            ensureNotFrozen(fieldName: "keys")
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
            ensureNotFrozen(fieldName: "contents")
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
            ensureNotFrozen(fieldName: "autoRenewPeriod")
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
            ensureNotFrozen(fieldName: "autoRenewAccountId")
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
    public var expirationTime: Timestamp? = .now + .days(90) {
        willSet {
            ensureNotFrozen(fieldName: "expirationTime")
        }
    }

    /// Sets the time at which this file should expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }
    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try self.autoRenewAccountId?.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_FileServiceAsyncClient(channel: channel).createFile(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .fileCreate(toProtobuf())
    }
}

extension FileCreateTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_FileCreateTransactionBody

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.memo = fileMemo
            proto.keys = keys.toProtobuf()
            proto.contents = contents

            expirationTime?.toProtobufInto(&proto.expirationTime)
        }
    }
}

extension FileCreateTransaction {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .fileCreate(toProtobuf())
    }
}
