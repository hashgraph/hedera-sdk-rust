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

// TODO: keys

/// Response from `FileInfoQuery`.
public final class FileInfo: Codable {
    /// The file ID of the file for which information is requested.
    public let fileId: FileId

    /// Number of bytes in contents.
    public let size: UInt64

    /// Current time which this account is set to expire.
    public let expirationTime: Duration?

    /// True if deleted but not yet expired.
    public let isDeleted: Bool

    /// One of these keys must sign in order to modify or delete the file.
    public let keys: KeyList

    /// Memo associated with the file.
    public let fileMemo: String

    public let ledgerId: LedgerId

    /// The auto renew period for this file.
    public let autoRenewPeriod: Duration?

    /// The account to be used at this file's expiration time to extend the
    /// life of the file.
    public let autoRenewAccountId: AccountId?

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self.fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! toJsonBytes()
    }
}

extension FileInfo: ToFromJsonBytes {
    internal static var cFromBytes: FromJsonBytesFunc { hedera_file_info_from_bytes }
    internal static var cToBytes: ToJsonBytesFunc { hedera_file_info_to_bytes }
}
