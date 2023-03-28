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

public struct TransactionHash: CustomStringConvertible {
    internal init(hashing data: Data) {
        self.data = Crypto.Sha2.sha384(data)
    }

    public let data: Data

    public var description: String {
        data.hexStringEncoded()
    }
}

#if compiler(<5.7)
    // Swift 5.7 added the conformance to data, despite to the best of my knowledge, not changing anything in the underlying type.
    extension TransactionHash: @unchecked Sendable {}
#else
    extension TransactionHash: Sendable {}
#endif
