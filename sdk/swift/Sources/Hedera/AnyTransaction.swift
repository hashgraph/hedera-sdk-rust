/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

internal enum AnyTransaction {
    case accountCreate(AccountCreateTransaction)
    case accountUpdate(AccountUpdateTransaction)
    case accountDelete(AccountDeleteTransaction)
    case accountAllowanceApprove(AccountAllowanceApproveTransaction)
    case accountAllowanceDelete(AccountAllowanceDeleteTransaction)
    case contractCreate(ContractCreateTransaction)
    case contractUpdate(ContractUpdateTransaction)
    case contractDelete(ContractDeleteTransaction)
    case contractExecute(ContractExecuteTransaction)
    case transfer(TransferTransaction)
    case topicCreate(TopicCreateTransaction)
    case topicUpdate(TopicUpdateTransaction)
    case topicDelete(TopicDeleteTransaction)
    case topicMessageSubmit(TopicMessageSubmitTransaction)
    case fileAppend(FileAppendTransaction)
    case fileCreate(FileCreateTransaction)
    case fileUpdate(FileUpdateTransaction)
    case fileDelete(FileDeleteTransaction)
    case tokenAssociate(TokenAssociateTransaction)
    case tokenBurn(TokenBurnTransaction)
    case tokenCreate(TokenCreateTransaction)
    case tokenDelete(TokenDeleteTransaction)
    case tokenDissociate(TokenDissociateTransaction)
    case tokenFeeScheduleUpdate(TokenFeeScheduleUpdateTransaction)
    case tokenFreeze(TokenFreezeTransaction)
    case tokenGrantKyc(TokenGrantKycTransaction)
    case tokenMint(TokenMintTransaction)
    case tokenPause(TokenPauseTransaction)
    case tokenRevokeKyc(TokenRevokeKycTransaction)
    case tokenUnfreeze(TokenUnfreezeTransaction)
    case tokenUnpause(TokenUnpauseTransaction)
    case tokenUpdate(TokenUpdateTransaction)
    case tokenWipe(TokenWipeTransaction)
    case systemDelete(SystemDeleteTransaction)
    case systemUndelete(SystemUndeleteTransaction)
    case freeze(FreezeTransaction)
    case scheduleCreate(ScheduleCreateTransaction)
    case scheduleSign(ScheduleSignTransaction)
    case scheduleDelete(ScheduleDeleteTransaction)
    case ethereum(EthereumTransaction)

    // swiftlint:disable:next cyclomatic_complexity
    internal init(upcasting transaction: Transaction) {
        switch transaction {
        case let transaction as AccountCreateTransaction: self = .accountCreate(transaction)
        case let transaction as AccountUpdateTransaction: self = .accountUpdate(transaction)
        case let transaction as AccountDeleteTransaction: self = .accountDelete(transaction)
        case let transaction as AccountAllowanceApproveTransaction: self = .accountAllowanceApprove(transaction)
        case let transaction as AccountAllowanceDeleteTransaction: self = .accountAllowanceDelete(transaction)
        case let transaction as ContractCreateTransaction: self = .contractCreate(transaction)
        case let transaction as ContractUpdateTransaction: self = .contractUpdate(transaction)
        case let transaction as ContractDeleteTransaction: self = .contractDelete(transaction)
        case let transaction as ContractExecuteTransaction: self = .contractExecute(transaction)
        case let transaction as TransferTransaction: self = .transfer(transaction)
        case let transaction as TopicCreateTransaction: self = .topicCreate(transaction)
        case let transaction as TopicUpdateTransaction: self = .topicUpdate(transaction)
        case let transaction as TopicDeleteTransaction: self = .topicDelete(transaction)
        case let transaction as TopicMessageSubmitTransaction: self = .topicMessageSubmit(transaction)
        case let transaction as FileAppendTransaction: self = .fileAppend(transaction)
        case let transaction as FileCreateTransaction: self = .fileCreate(transaction)
        case let transaction as FileUpdateTransaction: self = .fileUpdate(transaction)
        case let transaction as FileDeleteTransaction: self = .fileDelete(transaction)
        case let transaction as TokenAssociateTransaction: self = .tokenAssociate(transaction)
        case let transaction as TokenBurnTransaction: self = .tokenBurn(transaction)
        case let transaction as TokenCreateTransaction: self = .tokenCreate(transaction)
        case let transaction as TokenDeleteTransaction: self = .tokenDelete(transaction)
        case let transaction as TokenDissociateTransaction: self = .tokenDissociate(transaction)
        case let transaction as TokenFeeScheduleUpdateTransaction: self = .tokenFeeScheduleUpdate(transaction)
        case let transaction as TokenFreezeTransaction: self = .tokenFreeze(transaction)
        case let transaction as TokenGrantKycTransaction: self = .tokenGrantKyc(transaction)
        case let transaction as TokenMintTransaction: self = .tokenMint(transaction)
        case let transaction as TokenPauseTransaction: self = .tokenPause(transaction)
        case let transaction as TokenRevokeKycTransaction: self = .tokenRevokeKyc(transaction)
        case let transaction as TokenUnfreezeTransaction: self = .tokenUnfreeze(transaction)
        case let transaction as TokenUnpauseTransaction: self = .tokenUnpause(transaction)
        case let transaction as TokenUpdateTransaction: self = .tokenUpdate(transaction)
        case let transaction as TokenWipeTransaction: self = .tokenWipe(transaction)
        case let transaction as SystemDeleteTransaction: self = .systemDelete(transaction)
        case let transaction as SystemUndeleteTransaction: self = .systemUndelete(transaction)
        case let transaction as FreezeTransaction: self = .freeze(transaction)
        case let transaction as ScheduleCreateTransaction: self = .scheduleCreate(transaction)
        case let transaction as ScheduleSignTransaction: self = .scheduleSign(transaction)
        case let transaction as ScheduleDeleteTransaction: self = .scheduleDelete(transaction)
        case let transaction as EthereumTransaction: self = .ethereum(transaction)
        default: fatalError("Unrecognized transaction type")
        }
    }

    internal enum Kind: String, Codable {
        case accountCreate
        case accountUpdate
        case accountDelete
        case accountAllowanceApprove
        case accountAllowanceDelete
        case contractCreate
        case contractUpdate
        case contractDelete
        case contractExecute
        case transfer
        case topicCreate
        case topicUpdate
        case topicDelete
        case topicMessageSubmit
        case fileAppend
        case fileCreate
        case fileUpdate
        case fileDelete
        case tokenAssociate
        case tokenBurn
        case tokenCreate
        case tokenDelete
        case tokenDissociate
        case tokenFeeScheduleUpdate
        case tokenFreeze
        case tokenGrantKyc
        case tokenMint
        case tokenPause
        case tokenRevokeKyc
        case tokenUnfreeze
        case tokenUnpause
        case tokenUpdate
        case tokenWipe
        case systemDelete
        case systemUndelete
        case freeze
        case scheduleCreate
        case scheduleSign
        case scheduleDelete
        case ethereum
    }

    internal var transaction: Transaction {
        switch self {
        case .accountCreate(let transaction): return transaction
        case .accountUpdate(let transaction): return transaction
        case .accountDelete(let transaction): return transaction
        case .accountAllowanceApprove(let transaction): return transaction
        case .accountAllowanceDelete(let transaction): return transaction
        case .contractCreate(let transaction): return transaction
        case .contractUpdate(let transaction): return transaction
        case .contractDelete(let transaction): return transaction
        case .contractExecute(let transaction): return transaction
        case .transfer(let transaction): return transaction
        case .topicCreate(let transaction): return transaction
        case .topicUpdate(let transaction): return transaction
        case .topicDelete(let transaction): return transaction
        case .topicMessageSubmit(let transaction): return transaction
        case .fileAppend(let transaction): return transaction
        case .fileCreate(let transaction): return transaction
        case .fileUpdate(let transaction): return transaction
        case .fileDelete(let transaction): return transaction
        case .tokenAssociate(let transaction): return transaction
        case .tokenBurn(let transaction): return transaction
        case .tokenCreate(let transaction): return transaction
        case .tokenDelete(let transaction): return transaction
        case .tokenDissociate(let transaction): return transaction
        case .tokenFeeScheduleUpdate(let transaction): return transaction
        case .tokenFreeze(let transaction): return transaction
        case .tokenGrantKyc(let transaction): return transaction
        case .tokenMint(let transaction): return transaction
        case .tokenPause(let transaction): return transaction
        case .tokenRevokeKyc(let transaction): return transaction
        case .tokenUnfreeze(let transaction): return transaction
        case .tokenUnpause(let transaction): return transaction
        case .tokenUpdate(let transaction): return transaction
        case .tokenWipe(let transaction): return transaction
        case .systemDelete(let transaction): return transaction
        case .systemUndelete(let transaction): return transaction
        case .freeze(let transaction): return transaction
        case .scheduleCreate(let transaction): return transaction
        case .scheduleSign(let transaction): return transaction
        case .scheduleDelete(let transaction): return transaction
        case .ethereum(let transaction): return transaction
        }
    }
}

extension AnyTransaction: Decodable {
    internal enum CodingKeys: String, CodingKey {
        case type = "$type"
    }

    // swiftlint:disable:next function_body_length cyclomatic_complexity
    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        let kind = try container.decode(Kind.self, forKey: .type)

        switch kind {
        case .accountCreate:
            self = .accountCreate(try AccountCreateTransaction(from: decoder))
        case .accountUpdate:
            self = .accountUpdate(try AccountUpdateTransaction(from: decoder))
        case .accountDelete:
            self = .accountDelete(try AccountDeleteTransaction(from: decoder))
        case .accountAllowanceApprove:
            self = .accountAllowanceApprove(try AccountAllowanceApproveTransaction(from: decoder))
        case .accountAllowanceDelete:
            self = .accountAllowanceDelete(try AccountAllowanceDeleteTransaction(from: decoder))
        case .contractCreate:
            self = .contractCreate(try ContractCreateTransaction(from: decoder))
        case .contractUpdate:
            self = .contractUpdate(try ContractUpdateTransaction(from: decoder))
        case .contractDelete:
            self = .contractDelete(try ContractDeleteTransaction(from: decoder))
        case .contractExecute:
            self = .contractExecute(try ContractExecuteTransaction(from: decoder))
        case .transfer:
            self = .transfer(try TransferTransaction(from: decoder))
        case .topicCreate:
            self = .topicCreate(try TopicCreateTransaction(from: decoder))
        case .topicUpdate:
            self = .topicUpdate(try TopicUpdateTransaction(from: decoder))
        case .topicDelete:
            self = .topicDelete(try TopicDeleteTransaction(from: decoder))
        case .topicMessageSubmit:
            self = .topicMessageSubmit(try TopicMessageSubmitTransaction(from: decoder))
        case .fileAppend:
            self = .fileAppend(try FileAppendTransaction(from: decoder))
        case .fileCreate:
            self = .fileCreate(try FileCreateTransaction(from: decoder))
        case .fileUpdate:
            self = .fileUpdate(try FileUpdateTransaction(from: decoder))
        case .fileDelete:
            self = .fileDelete(try FileDeleteTransaction(from: decoder))
        case .tokenAssociate:
            self = .tokenAssociate(try TokenAssociateTransaction(from: decoder))
        case .tokenBurn:
            self = .tokenBurn(try TokenBurnTransaction(from: decoder))
        case .tokenCreate:
            self = .tokenCreate(try TokenCreateTransaction(from: decoder))
        case .tokenDelete:
            self = .tokenDelete(try TokenDeleteTransaction(from: decoder))
        case .tokenDissociate:
            self = .tokenDissociate(try TokenDissociateTransaction(from: decoder))
        case .tokenFeeScheduleUpdate:
            self = .tokenFeeScheduleUpdate(try TokenFeeScheduleUpdateTransaction(from: decoder))
        case .tokenFreeze:
            self = .tokenFreeze(try TokenFreezeTransaction(from: decoder))
        case .tokenGrantKyc:
            self = .tokenGrantKyc(try TokenGrantKycTransaction(from: decoder))
        case .tokenMint:
            self = .tokenMint(try TokenMintTransaction(from: decoder))
        case .tokenPause:
            self = .tokenPause(try TokenPauseTransaction(from: decoder))
        case .tokenRevokeKyc:
            self = .tokenRevokeKyc(try TokenRevokeKycTransaction(from: decoder))
        case .tokenUnfreeze:
            self = .tokenUnfreeze(try TokenUnfreezeTransaction(from: decoder))
        case .tokenUnpause:
            self = .tokenUnpause(try TokenUnpauseTransaction(from: decoder))
        case .tokenUpdate:
            self = .tokenUpdate(try TokenUpdateTransaction(from: decoder))
        case .tokenWipe:
            self = .tokenWipe(try TokenWipeTransaction(from: decoder))
        case .systemDelete:
            self = .systemDelete(try SystemDeleteTransaction(from: decoder))
        case .systemUndelete:
            self = .systemUndelete(try SystemUndeleteTransaction(from: decoder))
        case .freeze:
            self = .freeze(try FreezeTransaction(from: decoder))
        case .scheduleCreate:
            self = .scheduleCreate(try ScheduleCreateTransaction(from: decoder))
        case .scheduleSign:
            self = .scheduleSign(try ScheduleSignTransaction(from: decoder))
        case .scheduleDelete:
            self = .scheduleDelete(try ScheduleDeleteTransaction(from: decoder))
        case .ethereum:
            self = .ethereum(try EthereumTransaction(from: decoder))
        }
    }
}
