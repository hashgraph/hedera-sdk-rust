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

import HederaProtobufs

// big type
// swiftlint:disable file_length
// big type.
// swiftlint:disable:next type_body_length
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

    internal static func fromProtobuf(_ proto: Proto_TransactionBody.OneOf_Data) throws -> Transaction {
        switch proto {
        case .cryptoCreateAccount(let data):
            return Self.accountCreate(try AccountCreateTransaction.fromProtobufData(data)).transaction

        case .contractCall(let data):
            return Self.contractCall(_Transaction.fromProtobufData(data)).transaction

        case .contractCreateInstance(let data):
            return Self.contractCreateInstance(_Transaction.fromProtobufData(data)).transaction

        case .contractUpdateInstance(let data):
            return Self.contractUpdateInstance(_Transaction.fromProtobufData(data)).transaction

        case .contractDeleteInstance(let data):
            return Self.contractDeleteInstance(_Transaction.fromProtobufData(data)).transaction

        case .ethereumTransaction(let data):
            return Self.ethereumTransaction(_Transaction.fromProtobufData(data)).transaction

        case .cryptoAddLiveHash(let data):
            return Self.cryptoAddLiveHash(_Transaction.fromProtobufData(data)).transaction

        case .cryptoApproveAllowance(let data):
            return Self.cryptoApproveAllowance(_Transaction.fromProtobufData(data)).transaction

        case .cryptoDeleteAllowance(let data):
            return Self.cryptoDeleteAllowance(_Transaction.fromProtobufData(data)).transaction

        case .cryptoDelete(let data):
            return Self.cryptoDelete(_Transaction.fromProtobufData(data)).transaction

        case .cryptoDeleteLiveHash(let data):
            return Self.cryptoDeleteLiveHash(_Transaction.fromProtobufData(data)).transaction

        case .cryptoTransfer(let data):
            return Self.cryptoTransfer(_Transaction.fromProtobufData(data)).transaction

        case .cryptoUpdateAccount(let data):
            return Self.cryptoUpdateAccount(_Transaction.fromProtobufData(data)).transaction

        case .fileAppend(let data):
            return Self.fileAppend(_Transaction.fromProtobufData(data)).transaction

        case .fileCreate(let data):
            return Self.fileCreate(_Transaction.fromProtobufData(data)).transaction

        case .fileDelete(let data):
            return Self.fileDelete(_Transaction.fromProtobufData(data)).transaction

        case .fileUpdate(let data):
            return Self.fileUpdate(_Transaction.fromProtobufData(data)).transaction

        case .systemDelete(let data):
            return Self.systemDelete(_Transaction.fromProtobufData(data)).transaction

        case .systemUndelete(let data):
            return Self.systemUndelete(_Transaction.fromProtobufData(data)).transaction

        case .freeze(let data):
            return Self.freeze(_Transaction.fromProtobufData(data)).transaction

        case .consensusCreateTopic(let data):
            return Self.consensusCreateTopic(_Transaction.fromProtobufData(data)).transaction

        case .consensusUpdateTopic(let data):
            return Self.consensusUpdateTopic(_Transaction.fromProtobufData(data)).transaction

        case .consensusDeleteTopic(let data):
            return Self.consensusDeleteTopic(_Transaction.fromProtobufData(data)).transaction

        case .consensusSubmitMessage(let data):
            return Self.consensusSubmitMessage(_Transaction.fromProtobufData(data)).transaction

        case .uncheckedSubmit(let data):
            return Self.uncheckedSubmit(_Transaction.fromProtobufData(data)).transaction

        case .tokenCreation(let data):
            return Self.tokenCreation(_Transaction.fromProtobufData(data)).transaction

        case .tokenFreeze(let data):
            return Self.tokenFreeze(_Transaction.fromProtobufData(data)).transaction

        case .tokenUnfreeze(let data):
            return Self.tokenUnfreeze(_Transaction.fromProtobufData(data)).transaction

        case .tokenGrantKyc(let data):
            return Self.tokenGrantKyc(_Transaction.fromProtobufData(data)).transaction

        case .tokenRevokeKyc(let data):
            return Self.tokenRevokeKyc(_Transaction.fromProtobufData(data)).transaction

        case .tokenDeletion(let data):
            return Self.tokenDeletion(_Transaction.fromProtobufData(data)).transaction

        case .tokenUpdate(let data):
            return Self.tokenUpdate(_Transaction.fromProtobufData(data)).transaction

        case .tokenMint(let data):
            return Self.tokenMint(_Transaction.fromProtobufData(data)).transaction

        case .tokenBurn(let data):
            return Self.tokenBurn(_Transaction.fromProtobufData(data)).transaction

        case .tokenWipe(let data):
            return Self.tokenWipe(_Transaction.fromProtobufData(data)).transaction

        case .tokenAssociate(let data):
            return Self.tokenAssociate(_Transaction.fromProtobufData(data)).transaction

        case .tokenDissociate(let data):
            return Self.tokenDissociate(_Transaction.fromProtobufData(data)).transaction

        case .tokenFeeScheduleUpdate(let data):
            return Self.tokenFeeScheduleUpdate(_Transaction.fromProtobufData(data)).transaction

        case .tokenPause(let data):
            return Self.tokenPause(_Transaction.fromProtobufData(data)).transaction

        case .tokenUnpause(let data):
            return Self.tokenUnpause(_Transaction.fromProtobufData(data)).transaction

        case .scheduleCreate(let data):
            return Self.scheduleCreate(_Transaction.fromProtobufData(data)).transaction

        case .scheduleDelete(let data):
            return Self.scheduleDelete(_Transaction.fromProtobufData(data)).transaction

        case .scheduleSign(let data):
            return Self.scheduleSign(_Transaction.fromProtobufData(data)).transaction

        case .nodeStakeUpdate(let data):
            return Self.nodeStakeUpdate(_Transaction.fromProtobufData(data)).transaction

        case .utilPrng(let data):
            return Self.utilPrng(_Transaction.fromProtobufData(data)).transaction

        }
    }

    // swiftlint:disable:next function_body_length cyclomatic_complexity
    internal init(upcasting transaction: Transaction) {
        if let transaction = transaction as? AccountCreateTransaction {
            self = .accountCreate(transaction)
            return
        }

        if let transaction = transaction as? AccountUpdateTransaction {
            self = .accountUpdate(transaction)
            return
        }

        if let transaction = transaction as? AccountDeleteTransaction {
            self = .accountDelete(transaction)
            return
        }

        if let transaction = transaction as? AccountAllowanceApproveTransaction {
            self = .accountAllowanceApprove(transaction)
            return
        }

        if let transaction = transaction as? AccountAllowanceDeleteTransaction {
            self = .accountAllowanceDelete(transaction)
            return
        }

        if let transaction = transaction as? ContractCreateTransaction {
            self = .contractCreate(transaction)
            return
        }

        if let transaction = transaction as? ContractUpdateTransaction {
            self = .contractUpdate(transaction)
            return
        }

        if let transaction = transaction as? ContractDeleteTransaction {
            self = .contractDelete(transaction)
            return
        }

        if let transaction = transaction as? ContractExecuteTransaction {
            self = .contractExecute(transaction)
            return
        }

        if let transaction = transaction as? TransferTransaction {
            self = .transfer(transaction)
            return
        }

        if let transaction = transaction as? TopicCreateTransaction {
            self = .topicCreate(transaction)
            return
        }

        if let transaction = transaction as? TopicUpdateTransaction {
            self = .topicUpdate(transaction)
            return
        }

        if let transaction = transaction as? TopicDeleteTransaction {
            self = .topicDelete(transaction)
            return
        }

        if let transaction = transaction as? TopicMessageSubmitTransaction {
            self = .topicMessageSubmit(transaction)
            return
        }

        if let transaction = transaction as? FileAppendTransaction {
            self = .fileAppend(transaction)
            return
        }

        if let transaction = transaction as? FileCreateTransaction {
            self = .fileCreate(transaction)
            return
        }

        if let transaction = transaction as? FileUpdateTransaction {
            self = .fileUpdate(transaction)
            return
        }

        if let transaction = transaction as? FileDeleteTransaction {
            self = .fileDelete(transaction)
            return
        }

        if let transaction = transaction as? TokenAssociateTransaction {
            self = .tokenAssociate(transaction)
            return
        }

        if let transaction = transaction as? TokenBurnTransaction {
            self = .tokenBurn(transaction)
            return
        }

        if let transaction = transaction as? TokenCreateTransaction {
            self = .tokenCreate(transaction)
            return
        }

        if let transaction = transaction as? TokenDeleteTransaction {
            self = .tokenDelete(transaction)
            return
        }

        if let transaction = transaction as? TokenDissociateTransaction {
            self = .tokenDissociate(transaction)
            return
        }

        if let transaction = transaction as? TokenFeeScheduleUpdateTransaction {
            self = .tokenFeeScheduleUpdate(transaction)
            return
        }

        if let transaction = transaction as? TokenFreezeTransaction {
            self = .tokenFreeze(transaction)
            return
        }

        if let transaction = transaction as? TokenGrantKycTransaction {
            self = .tokenGrantKyc(transaction)
            return
        }

        if let transaction = transaction as? TokenMintTransaction {
            self = .tokenMint(transaction)
            return
        }

        if let transaction = transaction as? TokenPauseTransaction {
            self = .tokenPause(transaction)
            return
        }

        if let transaction = transaction as? TokenRevokeKycTransaction {
            self = .tokenRevokeKyc(transaction)
            return
        }

        if let transaction = transaction as? TokenUnfreezeTransaction {
            self = .tokenUnfreeze(transaction)
            return
        }

        if let transaction = transaction as? TokenUnpauseTransaction {
            self = .tokenUnpause(transaction)
            return
        }

        if let transaction = transaction as? TokenUpdateTransaction {
            self = .tokenUpdate(transaction)
            return
        }

        if let transaction = transaction as? TokenWipeTransaction {
            self = .tokenWipe(transaction)
            return
        }

        if let transaction = transaction as? SystemDeleteTransaction {
            self = .systemDelete(transaction)
            return
        }

        if let transaction = transaction as? SystemUndeleteTransaction {
            self = .systemUndelete(transaction)
            return
        }

        if let transaction = transaction as? FreezeTransaction {
            self = .freeze(transaction)
            return
        }

        if let transaction = transaction as? ScheduleCreateTransaction {
            self = .scheduleCreate(transaction)
            return
        }

        if let transaction = transaction as? ScheduleSignTransaction {
            self = .scheduleSign(transaction)
            return
        }

        if let transaction = transaction as? ScheduleDeleteTransaction {
            self = .scheduleDelete(transaction)
            return
        }

        if let transaction = transaction as? EthereumTransaction {
            self = .ethereum(transaction)
            return
        }

        fatalError("Unrecognized transaction type")
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
        case .accountCreate(let transaction):
            return transaction
        case .accountUpdate(let transaction):
            return transaction
        case .accountDelete(let transaction):
            return transaction
        case .accountAllowanceApprove(let transaction):
            return transaction
        case .accountAllowanceDelete(let transaction):
            return transaction
        case .contractCreate(let transaction):
            return transaction
        case .contractUpdate(let transaction):
            return transaction
        case .contractDelete(let transaction):
            return transaction
        case .contractExecute(let transaction):
            return transaction
        case .transfer(let transaction):
            return transaction
        case .topicCreate(let transaction):
            return transaction
        case .topicUpdate(let transaction):
            return transaction
        case .topicDelete(let transaction):
            return transaction
        case .topicMessageSubmit(let transaction):
            return transaction
        case .fileAppend(let transaction):
            return transaction
        case .fileCreate(let transaction):
            return transaction
        case .fileUpdate(let transaction):
            return transaction
        case .fileDelete(let transaction):
            return transaction
        case .tokenAssociate(let transaction):
            return transaction
        case .tokenBurn(let transaction):
            return transaction
        case .tokenCreate(let transaction):
            return transaction
        case .tokenDelete(let transaction):
            return transaction
        case .tokenDissociate(let transaction):
            return transaction
        case .tokenFeeScheduleUpdate(let transaction):
            return transaction
        case .tokenFreeze(let transaction):
            return transaction
        case .tokenGrantKyc(let transaction):
            return transaction
        case .tokenMint(let transaction):
            return transaction
        case .tokenPause(let transaction):
            return transaction
        case .tokenRevokeKyc(let transaction):
            return transaction
        case .tokenUnfreeze(let transaction):
            return transaction
        case .tokenUnpause(let transaction):
            return transaction
        case .tokenUpdate(let transaction):
            return transaction
        case .tokenWipe(let transaction):
            return transaction
        case .systemDelete(let transaction):
            return transaction
        case .systemUndelete(let transaction):
            return transaction
        case .freeze(let transaction):
            return transaction
        case .scheduleCreate(let transaction):
            return transaction
        case .scheduleSign(let transaction):
            return transaction
        case .scheduleDelete(let transaction):
            return transaction
        case .ethereum(let transaction):
            return transaction
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
