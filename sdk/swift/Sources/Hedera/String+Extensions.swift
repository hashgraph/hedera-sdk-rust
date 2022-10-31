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

extension String {
    /// Creates a new string by copying from a CHedera owned string.
    ///
    /// - Invariant: ``hString`` must not be used after calling this method.
    /// - Invariant: ``hString`` must have come from a `hedera_*` C function.
    ///
    /// - Parameter hString: A pointer to a CString
    ///
    internal init!(hString: UnsafeMutablePointer<CChar>) {
        // `String.init(validatingUTF8)` copies the string:
        // https://developer.apple.com/documentation/swift/string/init(validatingutf8:)-208fn
        // > Creates a new string by copying and validating
        //   the null-terminated UTF-8 data referenced by the given pointer.
        self.init(validatingUTF8: hString)

        hedera_string_free(hString)
    }
}
