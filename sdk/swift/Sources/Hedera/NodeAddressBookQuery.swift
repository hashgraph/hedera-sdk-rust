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

public class NodeAddressBookQuery: MirrorQuery<NodeAddressBook> {
    private var fileId: FileId
    private var limit: UInt32

    public init(_ fileId: FileId = FileId.addressBook, _ limit: UInt32 = 0) {
        self.fileId = fileId
        self.limit = limit
    }

    public func getFileId() -> FileId {
        fileId
    }

    public func setFileId(_ fileId: FileId) -> Self {
        self.fileId = fileId
        return self
    }

    public func getLimit() -> UInt32 {
        limit
    }

    public func setLimit(_ limit: UInt32) -> Self {
        self.limit = limit
        return self
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try fileId.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
