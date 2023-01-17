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
import Hedera
import SwiftDotenv

@main
public enum Program {
    public static func main() async throws {
        let env = try Dotenv.load()
        let client = Client.forTestnet()

        client.setOperator(env.operatorAccountId, env.operatorKey)

        let transactionResponse = try await TransferTransaction()
            .hbarTransfer("0.0.1001", 20)
            .hbarTransfer("0.0.6189", -20)
            .execute(client)

        // either of these values can be used to lookup transactions in an explorer such as
        //  Kabuto or DragonGlass; the transaction ID is generally more useful as it also contains a rough
        //  estimation of when the transaction was created (+/- 8 seconds) and the account that paid for
        //  transaction
        print("transaction id: \(transactionResponse.transactionId)")
        print("transaction hash: \(transactionResponse.transactionHash)")
    }
}

extension Environment {
    internal var operatorAccountId: AccountId {
        AccountId(self["OPERATOR_ACCOUNT_ID"]!.stringValue)!
    }

    internal var operatorKey: PrivateKey {
        PrivateKey(self["OPERATOR_KEY"]!.stringValue)!
    }

    public var networkName: String {
        self["HEDERA_NETWORK"]?.stringValue ?? "testnet"
    }
}
