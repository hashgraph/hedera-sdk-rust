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

import Hedera
import SwiftDotenv

@main
public enum Program {
    public static func main() async throws {
        let env = try Dotenv.load()
        let client = try Client.forName(env.networkName)

        print(try await AccountBalanceQuery().accountId(1001).getCost(client))

        let balance = try await AccountBalanceQuery()
            .accountId("0.0.1001")
            .execute(client)

        print("balance = \(balance.hbars)")
    }
}

extension Environment {
    public var networkName: String {
        self["HEDERA_NETWORK"]?.stringValue ?? "testnet"
    }
}
