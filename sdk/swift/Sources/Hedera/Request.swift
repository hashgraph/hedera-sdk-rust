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

internal protocol ValidateChecksums {
    func validateChecksums(on ledgerId: LedgerId) throws

    func validateChecksums(on client: Client) throws
}

extension ValidateChecksums {
    internal func validateChecksums(on client: Client) throws {
        try validateChecksums(on: client.ledgerId!)
    }
}

extension Array: ValidateChecksums where Self.Element: ValidateChecksums {
    internal func validateChecksums(on ledgerId: LedgerId) throws {
        try forEach { element in try element.validateChecksums(on: ledgerId) }
    }
}
