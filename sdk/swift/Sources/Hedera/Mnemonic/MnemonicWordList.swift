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

internal struct MnemonicWordList: ExpressibleByStringLiteral {
    internal init(stringLiteral value: StringLiteralType) {
        backingData = value
        words = value.split { $0.isNewline }
        isSorted = words.isSorted()
    }

    private let words: [Substring]
    private let backingData: String
    private let isSorted: Bool

    internal func indexOf<S: StringProtocol>(word: S) -> Int? {
        // todo: binary search if sorted
        words.firstIndex { $0 == word }
    }

    internal func contains<S: StringProtocol>(word: S) -> Bool {
        words.contains { $0 == word }
    }

    internal subscript(index: Int) -> Substring? {
        words[safe: index]
    }
}

extension Array where Element: Comparable {
    fileprivate func isSorted() -> Bool {
        // empty and mono-element arrays are sorted, just,
        // by nature of there being no (other) elements.
        if self.count < 2 {
            return true
        }

        return zip(self[1...], self).allSatisfy { !($0 > $1) }
    }
}
