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

        let (initialAdminKeys, topicId) = try await createTopicWithAdminKey(client)
        try await updateTopicAdminKeyAndMemo(client, initialAdminKeys, topicId)
    }

    private static func createTopicWithAdminKey(_ client: Client) async throws -> ([PrivateKey], TopicId) {
        // Generate the initial keys that are part of the adminKey's thresholdKey.
        // 3 ED25519 keys part of a 2-of-3 threshold key.
        let initialAdminKeys = (0..<3).map { _ in PrivateKey.generateEd25519() }

        let thresholdKey = KeyList(keys: initialAdminKeys.map { .single($0.getPublicKey()) }, threshold: 2)

        let transaction = try TopicCreateTransaction()
            .topicMemo("demo topic")
            .adminKey(.keyList(thresholdKey))
            .freezeWith(client)

        for key in initialAdminKeys.dropFirst(1) {
            print("Signing ConsensusTopicCreateTransaction with key", key)
            transaction.sign(key)
        }

        let topicId = try await transaction.execute(client).getReceipt(client).topicId!

        print("Created new topic \(topicId) with 2-of-3 threshold key as adminKey")

        return (initialAdminKeys, topicId)
    }

    private static func updateTopicAdminKeyAndMemo(
        _ client: Client, _ initialAdminKeys: [PrivateKey], _ topicId: TopicId
    ) async throws {
        // Generate the new keys that are part of the adminKey's thresholdKey.
        // 4 ED25519 keys part of a 3-of-4 threshold key.
        let newAdminKeys = (0..<4).map { _ in PrivateKey.generateEd25519() }

        let thresholdKey = KeyList(keys: newAdminKeys.map { .single($0.getPublicKey()) }, threshold: 3)

        let transaction = try TopicUpdateTransaction().topicId(topicId).topicMemo("updated example topic").adminKey(
            .keyList(thresholdKey)
        )
        .freezeWith(client)

        // Sign with the initial adminKey. 2 of the 3 keys already part of the topic's adminKey.
        // Note that this time we're using a different subset of keys ([0, 1], rather than [1, 2])
        for key in initialAdminKeys.dropLast(1) {
            print("Signing ConsensusTopicUpdateTransaction with initial admin key", key)
            transaction.sign(key)
        }

        for key in newAdminKeys.dropFirst(1) {
            print("Signing ConsensusTopicUpdateTransaction with new admin key", key)
            transaction.sign(key)
        }

        _ = try await transaction.execute(client).getReceipt(client)

        print("Updated topic \(topicId) with 3-of-4 threshold key as adminKey")

        let topicInfo = try await TopicInfoQuery().topicId(topicId).execute(client)
        print(topicInfo)
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
