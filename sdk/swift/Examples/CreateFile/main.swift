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

        // Defaults the operator account ID and key such that all generated transactions will be paid for
        // by this account and be signed by this key
        client.setOperator(env.operatorAccountId, env.operatorKey)

        // The file is required to be a byte array,
        // you can easily use the bytes of a file instead.
        let fileContents = "Hedera hashgraph is great!"

        let response = try await FileCreateTransaction()
            .keys([.single(env.operatorKey.publicKey)])
            .contents(fileContents.data(using: .utf8)!)
            .maxTransactionFee(2)
            .execute(client)

        let receipt = try await response.getReceipt(client)

        print("file: \(String(describing: receipt.fileId))")
    }
}

extension Environment {
    public var operatorAccountId: AccountId {
        AccountId(self["OPERATOR_ACCOUNT_ID"]!.stringValue)!
    }

    public var operatorKey: PrivateKey {
        PrivateKey(self["OPERATOR_KEY"]!.stringValue)!
    }

    public var networkName: String {
        self["HEDERA_NETWORK"]?.stringValue ?? "testnet"
    }
}
