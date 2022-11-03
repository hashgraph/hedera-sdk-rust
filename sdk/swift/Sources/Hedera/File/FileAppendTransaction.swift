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
public final class FileAppendTransaction: Transaction {
    /// Create a new `FileAppendTransaction` ready for configuration.
    public override init() {}

    /// The file to which the bytes will be appended.
    public var fileId: FileId?

    /// Sets the file to which the bytes will be appended.
    @discardableResult
    public func fileId(_ fileId: FileId) -> Self {
        self.fileId = fileId

        return self
    }

    /// The bytes that will be appended to the end of the specified file.
    public var contents: Data = Data()

    /// Sets the bytes that will be appended to the end of the specified file.
    @discardableResult
    public func contents(_ contents: Data) -> Self {
        self.contents = contents

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case fileId
        case contents
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(fileId, forKey: .fileId)
        try container.encode(contents.base64EncodedString(), forKey: .contents)

        try super.encode(to: encoder)
    }
}
