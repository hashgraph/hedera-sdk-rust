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
import HederaProtobufs

// swiftlint:disable file_length

/// The functionality provided by Hedera.
public enum RequestType {
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

extension RequestType: TryProtobufCodable {
    internal typealias Protobuf = Proto_HederaFunctionality

    // this literally can't be smaller.
    // swiftlint:disable:next function_body_length
    internal init(protobuf proto: Protobuf) throws {
        switch proto {
        case .none: self = .none
        case .cryptoTransfer: self = .cryptoTransfer
        case .cryptoUpdate: self = .cryptoUpdate
        case .cryptoDelete: self = .cryptoDelete
        case .cryptoAddLiveHash: self = .cryptoAddLiveHash
        case .cryptoDeleteLiveHash: self = .cryptoDeleteLiveHash
        case .contractCall: self = .contractCall
        case .contractCreate: self = .contractCreate
        case .contractUpdate: self = .contractUpdate
        case .fileCreate: self = .fileCreate
        case .fileAppend: self = .fileAppend
        case .fileUpdate: self = .fileUpdate
        case .fileDelete: self = .fileDelete
        case .cryptoGetAccountBalance: self = .cryptoGetAccountBalance
        case .cryptoGetAccountRecords: self = .cryptoGetAccountRecords
        case .cryptoGetInfo: self = .cryptoGetInfo
        case .contractCallLocal: self = .contractCallLocal
        case .contractGetInfo: self = .contractGetInfo
        case .contractGetBytecode: self = .contractGetBytecode
        case .getBySolidityID: self = .getBySolidityId
        case .getByKey: self = .getByKey
        case .cryptoGetLiveHash: self = .cryptoGetLiveHash
        case .cryptoGetStakers: self = .cryptoGetStakers
        case .fileGetContents: self = .fileGetContents
        case .fileGetInfo: self = .fileGetInfo
        case .transactionGetRecord: self = .transactionGetRecord
        case .contractGetRecords: self = .contractGetRecords
        case .cryptoCreate: self = .cryptoCreate
        case .systemDelete: self = .systemDelete
        case .systemUndelete: self = .systemUndelete
        case .contractDelete: self = .contractDelete
        case .freeze: self = .freeze
        case .createTransactionRecord: self = .createTransactionRecord
        case .cryptoAccountAutoRenew: self = .cryptoAccountAutoRenew
        case .contractAutoRenew: self = .contractAutoRenew
        case .getVersionInfo: self = .getVersionInfo
        case .transactionGetReceipt: self = .transactionGetReceipt
        case .consensusCreateTopic: self = .consensusCreateTopic
        case .consensusUpdateTopic: self = .consensusUpdateTopic
        case .consensusDeleteTopic: self = .consensusDeleteTopic
        case .consensusGetTopicInfo: self = .consensusGetTopicInfo
        case .consensusSubmitMessage: self = .consensusSubmitMessage
        case .uncheckedSubmit: self = .uncheckedSubmit
        case .tokenCreate: self = .tokenCreate
        case .tokenGetInfo: self = .tokenGetInfo
        case .tokenFreezeAccount: self = .tokenFreezeAccount
        case .tokenUnfreezeAccount: self = .tokenUnfreezeAccount
        case .tokenGrantKycToAccount: self = .tokenGrantKycToAccount
        case .tokenRevokeKycFromAccount: self = .tokenRevokeKycFromAccount
        case .tokenDelete: self = .tokenDelete
        case .tokenUpdate: self = .tokenUpdate
        case .tokenMint: self = .tokenMint
        case .tokenBurn: self = .tokenBurn
        case .tokenAccountWipe: self = .tokenAccountWipe
        case .tokenAssociateToAccount: self = .tokenAssociateToAccount
        case .tokenDissociateFromAccount: self = .tokenDissociateFromAccount
        case .scheduleCreate: self = .scheduleCreate
        case .scheduleDelete: self = .scheduleDelete
        case .scheduleSign: self = .scheduleSign
        case .scheduleGetInfo: self = .scheduleGetInfo
        case .tokenGetAccountNftInfos: self = .tokenGetAccountNftInfos
        case .tokenGetNftInfo: self = .tokenGetNftInfo
        case .tokenGetNftInfos: self = .tokenGetNftInfos
        case .tokenFeeScheduleUpdate: self = .tokenFeeScheduleUpdate
        case .networkGetExecutionTime: self = .networkGetExecutionTime
        case .tokenPause: self = .tokenPause
        case .tokenUnpause: self = .tokenUnpause
        case .cryptoApproveAllowance: self = .cryptoApproveAllowance
        case .cryptoDeleteAllowance: self = .cryptoDeleteAllowance
        case .getAccountDetails: self = .getAccountDetails
        case .ethereumTransaction: self = .ethereumTransaction
        case .nodeStakeUpdate: self = .nodeStakeUpdate
        case .utilPrng: self = .utilPrng
        case .UNRECOGNIZED(let code):
            throw HError.fromProtobuf("unrecognized RequestType: `\(code)`")
        }
    }

    // this literally can't be smaller.
    // swiftlint:disable:next function_body_length
    internal func toProtobuf() -> Protobuf {
        switch self {
        case .none: return .none
        case .cryptoTransfer: return .cryptoTransfer
        case .cryptoUpdate: return .cryptoUpdate
        case .cryptoDelete: return .cryptoDelete
        case .cryptoAddLiveHash: return .cryptoAddLiveHash
        case .cryptoDeleteLiveHash: return .cryptoDeleteLiveHash
        case .contractCall: return .contractCall
        case .contractCreate: return .contractCreate
        case .contractUpdate: return .contractUpdate
        case .fileCreate: return .fileCreate
        case .fileAppend: return .fileAppend
        case .fileUpdate: return .fileUpdate
        case .fileDelete: return .fileDelete
        case .cryptoGetAccountBalance: return .cryptoGetAccountBalance
        case .cryptoGetAccountRecords: return .cryptoGetAccountRecords
        case .cryptoGetInfo: return .cryptoGetInfo
        case .contractCallLocal: return .contractCallLocal
        case .contractGetInfo: return .contractGetInfo
        case .contractGetBytecode: return .contractGetBytecode
        case .getBySolidityId: return .getBySolidityID
        case .getByKey: return .getByKey
        case .cryptoGetLiveHash: return .cryptoGetLiveHash
        case .cryptoGetStakers: return .cryptoGetStakers
        case .fileGetContents: return .fileGetContents
        case .fileGetInfo: return .fileGetInfo
        case .transactionGetRecord: return .transactionGetRecord
        case .contractGetRecords: return .contractGetRecords
        case .cryptoCreate: return .cryptoCreate
        case .systemDelete: return .systemDelete
        case .systemUndelete: return .systemUndelete
        case .contractDelete: return .contractDelete
        case .freeze: return .freeze
        case .createTransactionRecord: return .createTransactionRecord
        case .cryptoAccountAutoRenew: return .cryptoAccountAutoRenew
        case .contractAutoRenew: return .contractAutoRenew
        case .getVersionInfo: return .getVersionInfo
        case .transactionGetReceipt: return .transactionGetReceipt
        case .consensusCreateTopic: return .consensusCreateTopic
        case .consensusUpdateTopic: return .consensusUpdateTopic
        case .consensusDeleteTopic: return .consensusDeleteTopic
        case .consensusGetTopicInfo: return .consensusGetTopicInfo
        case .consensusSubmitMessage: return .consensusSubmitMessage
        case .uncheckedSubmit: return .uncheckedSubmit
        case .tokenCreate: return .tokenCreate
        case .tokenGetInfo: return .tokenGetInfo
        case .tokenFreezeAccount: return .tokenFreezeAccount
        case .tokenUnfreezeAccount: return .tokenUnfreezeAccount
        case .tokenGrantKycToAccount: return .tokenGrantKycToAccount
        case .tokenRevokeKycFromAccount: return .tokenRevokeKycFromAccount
        case .tokenDelete: return .tokenDelete
        case .tokenUpdate: return .tokenUpdate
        case .tokenMint: return .tokenMint
        case .tokenBurn: return .tokenBurn
        case .tokenAccountWipe: return .tokenAccountWipe
        case .tokenAssociateToAccount: return .tokenAssociateToAccount
        case .tokenDissociateFromAccount: return .tokenDissociateFromAccount
        case .scheduleCreate: return .scheduleCreate
        case .scheduleDelete: return .scheduleDelete
        case .scheduleSign: return .scheduleSign
        case .scheduleGetInfo: return .scheduleGetInfo
        case .tokenGetAccountNftInfos: return .tokenGetAccountNftInfos
        case .tokenGetNftInfo: return .tokenGetNftInfo
        case .tokenGetNftInfos: return .tokenGetNftInfos
        case .tokenFeeScheduleUpdate: return .tokenFeeScheduleUpdate
        case .networkGetExecutionTime: return .networkGetExecutionTime
        case .tokenPause: return .tokenPause
        case .tokenUnpause: return .tokenUnpause
        case .cryptoApproveAllowance: return .cryptoApproveAllowance
        case .cryptoDeleteAllowance: return .cryptoDeleteAllowance
        case .getAccountDetails: return .getAccountDetails
        case .ethereumTransaction: return .ethereumTransaction
        case .nodeStakeUpdate: return .nodeStakeUpdate
        case .utilPrng: return .utilPrng
        }
    }
}
