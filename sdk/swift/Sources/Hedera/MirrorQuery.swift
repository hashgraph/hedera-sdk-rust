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

public class MirrorQuery<Response: Decodable>: Request {
    public typealias Response = Response

    internal init() {}

    private enum CodingKeys: String, CodingKey {
        case type = "$type"
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        let typeName = String(describing: type(of: self))
        let requestName = typeName.prefix(1).lowercased() + typeName.dropFirst().dropLast(5)

        try container.encode(requestName, forKey: .type)
    }

    public func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws -> Response {
        try await executeInternal(client, timeout)
    }

    internal func validateChecksums(on ledgerId: LedgerId) throws {
        // nothing to do.
    }
}
