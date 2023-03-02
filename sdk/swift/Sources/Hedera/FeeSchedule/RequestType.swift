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

/// The functionality provided by Hedera.
public enum RequestType: Codable {
    /// UNSPECIFIED - Need to keep first value as unspecified because first element is ignored and not parsed (0 is ignored by parser)
    case none

    /// Transfer from one account to another.
    case cryptoTransfer

    /// Update an account.
    case cryptoUpdate

    /// Delete an account.
    case cryptoDelete

    /// Add a live hash to an account (not currently supported).
    case cryptoAddLiveHash

    /// Remove a live hash from an account (not currently supported).
    case cryptoDeleteLiveHash

    /// Execute a contract.
    case contractCall

    /// Create a contract.
    case contractCreate

    /// Update a contract.
    case contractUpdate

    /// Create a file.
    case fileCreate

    /// Append data to a file.
    case fileAppend

    /// Update a file.
    case fileUpdate

    /// Delete a file.
    case fileDelete

    /// Query the balance for an account.
    case cryptoGetAccountBalance

    /// Query the records for an account.
    case cryptoGetAccountRecords

    /// Query the info for an account.
    case cryptoGetInfo

    /// Execute a contract locally on a node.
    case contractCallLocal

    /// Query the info for a contract.
    case contractGetInfo

    /// Query the bytecode for a contract.
    case contractGetBytecode

    /// Lookup a contract by its solidity ID.
    case getBySolidityId

    /// Lookup a contract by key.
    case getByKey

    /// Query the live hashes for a account (not currently supported).
    case cryptoGetLiveHash

    /// Query the stakers for an account.
    case cryptoGetStakers

    /// Query the contents of a file.
    case fileGetContents

    /// Query the info for a file.
    case fileGetInfo

    /// Query the record for a transaction.
    case transactionGetRecord

    /// Query the records for a contract.
    case contractGetRecords

    /// Create an account.
    case cryptoCreate

    /// System delete a file or contract.
    case systemDelete

    /// System undelete a file or contract.
    case systemUndelete

    /// Delete a contract.
    case contractDelete

    /// Freeze the network.
    case freeze

    /// Creation of a transaction record..
    case createTransactionRecord

    /// Auto renewal of an account.
    case cryptoAccountAutoRenew

    /// Auto renewal of a contract
    case contractAutoRenew

    /// Query the version info of the network.
    case getVersionInfo

    /// Query the receipt for a transaction.
    case transactionGetReceipt

    /// Create a topic.
    case consensusCreateTopic

    /// Update a topic.
    case consensusUpdateTopic

    /// Delete a topic.
    case consensusDeleteTopic

    /// Query the info for a topic.
    case consensusGetTopicInfo

    /// Submit a message to a topic.
    case consensusSubmitMessage

    /// Submit a transaction without validation.
    case uncheckedSubmit

    /// Create a topic.
    case tokenCreate

    /// Query the info for a token.
    case tokenGetInfo

    /// Freeze an account's balance of a token.
    case tokenFreezeAccount

    /// Unfreeze an account's balance of a token.
    case tokenUnfreezeAccount

    /// Grant KYC to an account for a token.
    case tokenGrantKycToAccount

    /// Revoke KYC from an account for a token.
    case tokenRevokeKycFromAccount

    /// Delete a token.
    case tokenDelete

    /// Update a token.
    case tokenUpdate

    /// Mint items on a token.
    case tokenMint

    /// Burn items from a token.
    case tokenBurn

    /// Wipe an account's balance of a token.
    case tokenAccountWipe

    /// Associate tokens to an account.
    case tokenAssociateToAccount

    /// Dissociate tokens from an account.
    case tokenDissociateFromAccount

    /// Create a schedule.
    case scheduleCreate

    /// Delete a schedule.
    case scheduleDelete

    /// Sign a schedule.
    case scheduleSign

    /// Query the info for a schedule.
    case scheduleGetInfo

    /// Query the info of held NFTs for an account.
    case tokenGetAccountNftInfos

    /// Query the info of an NFT for a token.
    case tokenGetNftInfo

    /// Query the info of NFT for a token.
    case tokenGetNftInfos

    /// Update the fee schedule for a token.
    case tokenFeeScheduleUpdate

    /// Query the execution time of a transaction.
    case networkGetExecutionTime

    /// Pause usage of a token.
    case tokenPause

    /// Unpause usage of a token.
    case tokenUnpause

    /// Approve an account spending another account's currency.
    case cryptoApproveAllowance

    /// Unapprove an account spending another account's currency.
    case cryptoDeleteAllowance

    /// Query the details for an account.
    case getAccountDetails

    /// Execute an ethereum style transaction.
    case ethereumTransaction

    /// Update an account/contract's staked node.
    case nodeStakeUpdate

    /// Execute a PRNG transaction.
    case utilPrng
}
