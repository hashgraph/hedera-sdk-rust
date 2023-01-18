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

/// Create a new file, containing the given contents.
public final class FileCreateTransaction: Transaction {
    internal init(
        fileMemo: String = "",
        keys: KeyList = [],
        contents: Data = Data(),
        autoRenewPeriod: Duration? = nil,
        autoRenewAccountId: AccountId? = nil,
        expirationTime: Timestamp? = Timestamp(
            from: Calendar.current.date(byAdding: .day, value: 90, to: Date())!)
    ) {
        self.fileMemo = fileMemo
        self.keys = keys
        self.contents = contents
        self.autoRenewPeriod = autoRenewPeriod
        self.autoRenewAccountId = autoRenewAccountId
        self.expirationTime = expirationTime

        super.init()
    }

    /// Create a new `FileCreateTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        fileMemo = try container.decodeIfPresent(.fileMemo) ?? ""
        keys = try container.decodeIfPresent(.keys) ?? []
        contents = try container.decodeIfPresent(.contents).map(Data.base64Encoded) ?? Data()
        autoRenewPeriod = try container.decodeIfPresent(.autoRenewPeriod)
        autoRenewAccountId = try container.decodeIfPresent(.autoRenewAccountId)
        expirationTime = try container.decodeIfPresent(.expirationTime)

        try super.init(from: decoder)
    }

    /// The memo associated with the file.
    public var fileMemo: String = ""

    /// Sets the memo associated with the file.
    @discardableResult
    public func fileMemo(_ fileMemo: String) -> Self {
        self.fileMemo = fileMemo

        return self
    }

    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    public var keys: KeyList = []

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
    public var contents: Data = Data()

    /// Sets the bytes that are to be the contents of the file.
    @discardableResult
    public func contents(_ contents: Data) -> Self {
        self.contents = contents

        return self
    }

    /// The auto renew period for this file.
    public var autoRenewPeriod: Duration?

    /// Set the auto renew period for this file.
    public func autoRenewPeriod(_ autoRenewPeriod: Duration) -> Self {
        self.autoRenewPeriod = autoRenewPeriod

        return self
    }

    /// The account to be used at the files's expiration time to extend the
    /// life of the file.
    public var autoRenewAccountId: AccountId?

    /// Sets the account to be used at the files's expiration time to extend the
    /// life of the file.
    public func autoRenewAccountId(_ autoRenewAccountId: AccountId) -> Self {
        self.autoRenewAccountId = autoRenewAccountId

        return self
    }

    /// The time at which this file should expire.
    public var expirationTime: Timestamp? = Timestamp(
        from: Calendar.current.date(byAdding: .day, value: 90, to: Date())!)

    /// Sets the time at which this file should expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Timestamp) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileMemo
        case keys
        case contents
        case expirationTime
        case autoRenewPeriod
        case autoRenewAccountId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(fileMemo, forKey: .fileMemo)
        try container.encode(keys, forKey: .keys)
        try container.encode(contents.base64EncodedString(), forKey: .contents)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encodeIfPresent(autoRenewAccountId, forKey: .autoRenewAccountId)
        try container.encodeIfPresent(expirationTime, forKey: .expirationTime)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try self.autoRenewAccountId?.validateChecksums(on: ledgerId)
    }
}
