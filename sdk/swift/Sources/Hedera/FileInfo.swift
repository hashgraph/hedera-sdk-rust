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

// TODO: keys
/// Response from `FileInfoQuery`.
public final class FileInfo: Codable {
    /// The file ID of the file for which information is requested.
    public let fileId: FileId

    /// Number of bytes in contents.
    public let size: UInt64

    /// Current time which this account is set to expire.
    public let expirationTime: TimeInterval?

    /// True if deleted but not yet expired.
    public let isDeleted: Bool

    /// Memo associated with the file.
    public let fileMemo: String
}
