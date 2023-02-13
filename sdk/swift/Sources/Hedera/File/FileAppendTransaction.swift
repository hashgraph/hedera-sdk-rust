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

/// Append the given contents to the end of the specified file.
public final class FileAppendTransaction: ChunkedTransaction {
    /// Create a new `FileAppendTransaction` ready for configuration.
    public override init() {
        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        fileId = try container.decodeIfPresent(.fileId)

        try super.init(from: decoder)
    }

    /// The file to which the bytes will be appended.
    public var fileId: FileId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the file to which the bytes will be appended.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The bytes that will be appended to the end of the specified file.
    public var contents: Data {
        get { data }
        set(contents) {
            ensureNotFrozen()
            data = contents
        }
    }

    /// Sets the bytes that will be appended to the end of the specified file.
    @discardableResult
    public func contents(_ contents: Data) -> Self {
        self.contents = contents

        return self
    }

    /// Sets `self.contents` to the UTF-8 encoded bytes of `contents`.
    @discardableResult
    public func contents(_ contents: String) -> Self {
        self.contents = contents.data(using: .utf8)!

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(fileId, forKey: .fileId)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
