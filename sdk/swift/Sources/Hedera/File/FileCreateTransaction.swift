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
    /// Create a new `FileCreateTransaction` ready for configuration.
    public override init() {}

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
    public var keys: [Key] = []

    /// Sets the keys for this file.
    ///
    /// All keys at the top level of a key list must sign to create or
    /// modify the file. Any one of the keys at the top level key list
    /// can sign to delete the file.
    ///
    @discardableResult
    public func keys(_ keys: [Key]) -> Self {
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

    /// The time at which this file should expire.
    public var expirationTime: Date?

    /// Sets the time at which this file should expire.
    @discardableResult
    public func expirationTime(_ expirationTime: Date) -> Self {
        self.expirationTime = expirationTime

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileMemo
        case keys
        case contents
        case expirationTime
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(fileMemo, forKey: .fileMemo)
        try container.encode(keys, forKey: .keys)
        try container.encode(contents.base64EncodedString(), forKey: .contents)
        try container.encodeIfPresent(expirationTime?.unixTimestampNanos, forKey: .expirationTime)

        try super.encode(to: encoder)
    }
}
