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
    case prng(PrngTransaction)
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

    // internal enum Kind: String {
    //     case accountCreate
    //     case accountUpdate
    //     case accountDelete
    //     case accountAllowanceApprove
    //     case accountAllowanceDelete
    //     case contractCreate
    //     case contractUpdate
    //     case contractDelete
    //     case contractExecute
    //     case transfer
    //     case topicCreate
    //     case topicUpdate
    //     case topicDelete
    //     case topicMessageSubmit
    //     case fileAppend
    //     case fileCreate
    //     case fileUpdate
    //     case fileDelete
    //     case tokenAssociate
    //     case tokenBurn
    //     case tokenCreate
    //     case tokenDelete
    //     case tokenDissociate
    //     case tokenFeeScheduleUpdate
    //     case tokenFreeze
    //     case tokenGrantKyc
    //     case tokenMint
    //     case tokenPause
    //     case tokenRevokeKyc
    //     case tokenUnfreeze
    //     case tokenUnpause
    //     case tokenUpdate
    //     case tokenWipe
    //     case systemDelete
    //     case systemUndelete
    //     case freeze
    //     case scheduleCreate
    //     case scheduleSign
    //     case scheduleDelete
    //     case ethereum
    // }

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
        case .prng(let transaction): return transaction
        }
    }
}

extension AnyTransaction {
    internal static func fromProtobuf(
        _ firstBody: Proto_TransactionBody, _ data: [Proto_TransactionBody.OneOf_Data]
    ) throws -> Self {
        func intoOnlyValue<Element>(_ array: [Element]) throws -> Element {
            guard array.count == 1 else {
                throw HError.fromProtobuf("chunks in non chunkable transaction")
            }

            return array[0]
        }

        let data = try ServicesTransactionDataList.fromProtobuf(data)

        switch data {

        case .accountCreate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try AccountCreateTransaction(protobuf: firstBody, value))

        case .accountUpdate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try AccountUpdateTransaction(protobuf: firstBody, value))

        case .accountDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try AccountDeleteTransaction(protobuf: firstBody, value))

        case .accountAllowanceApprove(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try AccountAllowanceApproveTransaction(protobuf: firstBody, value))

        case .accountAllowanceDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try AccountAllowanceDeleteTransaction(protobuf: firstBody, value))

        case .contractCreate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try ContractCreateTransaction(protobuf: firstBody, value))

        case .contractUpdate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try ContractUpdateTransaction(protobuf: firstBody, value))

        case .contractDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try ContractDeleteTransaction(protobuf: firstBody, value))

        case .contractExecute(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try ContractExecuteTransaction(protobuf: firstBody, value))

        case .transfer(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TransferTransaction(protobuf: firstBody, value))

        case .topicCreate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TopicCreateTransaction(protobuf: firstBody, value))

        case .topicUpdate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TopicUpdateTransaction(protobuf: firstBody, value))

        case .topicDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TopicDeleteTransaction(protobuf: firstBody, value))

        case .topicMessageSubmit(let value):
            return Self(upcasting: try TopicMessageSubmitTransaction(protobuf: firstBody, value))

        case .fileAppend(let value):
            return Self(upcasting: try FileAppendTransaction(protobuf: firstBody, value))

        case .fileCreate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try FileCreateTransaction(protobuf: firstBody, value))

        case .fileUpdate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try FileUpdateTransaction(protobuf: firstBody, value))

        case .fileDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try FileDeleteTransaction(protobuf: firstBody, value))

        case .tokenAssociate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenAssociateTransaction(protobuf: firstBody, value))

        case .tokenBurn(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenBurnTransaction(protobuf: firstBody, value))

        case .tokenCreate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenCreateTransaction(protobuf: firstBody, value))

        case .tokenDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenDeleteTransaction(protobuf: firstBody, value))

        case .tokenDissociate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenDissociateTransaction(protobuf: firstBody, value))

        case .tokenFeeScheduleUpdate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenFeeScheduleUpdateTransaction(protobuf: firstBody, value))

        case .tokenFreeze(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenFreezeTransaction(protobuf: firstBody, value))

        case .tokenGrantKyc(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenGrantKycTransaction(protobuf: firstBody, value))

        case .tokenMint(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenMintTransaction(protobuf: firstBody, value))

        case .tokenPause(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenPauseTransaction(protobuf: firstBody, value))

        case .tokenRevokeKyc(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenRevokeKycTransaction(protobuf: firstBody, value))

        case .tokenUnfreeze(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenUnfreezeTransaction(protobuf: firstBody, value))

        case .tokenUnpause(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenUnpauseTransaction(protobuf: firstBody, value))

        case .tokenUpdate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenUpdateTransaction(protobuf: firstBody, value))

        case .tokenWipe(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try TokenWipeTransaction(protobuf: firstBody, value))

        case .systemDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try SystemDeleteTransaction(protobuf: firstBody, value))

        case .systemUndelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try SystemUndeleteTransaction(protobuf: firstBody, value))

        case .freeze(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try FreezeTransaction(protobuf: firstBody, value))

        case .scheduleCreate(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try ScheduleCreateTransaction(protobuf: firstBody, value))

        case .scheduleSign(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try ScheduleSignTransaction(protobuf: firstBody, value))

        case .scheduleDelete(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try ScheduleDeleteTransaction(protobuf: firstBody, value))

        case .ethereum(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try EthereumTransaction(protobuf: firstBody, value))

        case .prng(let value):
            let value = try intoOnlyValue(value)
            return Self(upcasting: try PrngTransaction(protobuf: firstBody, value))
        }
    }
}

// exists for the same reason as rust and still sucks :/\
internal enum ServicesTransactionDataList {
    case accountCreate([Proto_CryptoCreateTransactionBody])
    case accountUpdate([Proto_CryptoUpdateTransactionBody])
    case accountDelete([Proto_CryptoDeleteTransactionBody])
    case accountAllowanceApprove([Proto_CryptoApproveAllowanceTransactionBody])
    case accountAllowanceDelete([Proto_CryptoDeleteAllowanceTransactionBody])
    case contractCreate([Proto_ContractCreateTransactionBody])
    case contractUpdate([Proto_ContractUpdateTransactionBody])
    case contractDelete([Proto_ContractDeleteTransactionBody])
    case contractExecute([Proto_ContractCallTransactionBody])
    case transfer([Proto_CryptoTransferTransactionBody])
    case topicCreate([Proto_ConsensusCreateTopicTransactionBody])
    case topicUpdate([Proto_ConsensusUpdateTopicTransactionBody])
    case topicDelete([Proto_ConsensusDeleteTopicTransactionBody])
    case topicMessageSubmit([Proto_ConsensusSubmitMessageTransactionBody])
    case fileAppend([Proto_FileAppendTransactionBody])
    case fileCreate([Proto_FileCreateTransactionBody])
    case fileUpdate([Proto_FileUpdateTransactionBody])
    case fileDelete([Proto_FileDeleteTransactionBody])
    case tokenAssociate([Proto_TokenAssociateTransactionBody])
    case tokenBurn([Proto_TokenBurnTransactionBody])
    case tokenCreate([Proto_TokenCreateTransactionBody])
    case tokenDelete([Proto_TokenDeleteTransactionBody])
    case tokenDissociate([Proto_TokenDissociateTransactionBody])
    case tokenFeeScheduleUpdate([Proto_TokenFeeScheduleUpdateTransactionBody])
    case tokenFreeze([Proto_TokenFreezeAccountTransactionBody])
    case tokenGrantKyc([Proto_TokenGrantKycTransactionBody])
    case tokenMint([Proto_TokenMintTransactionBody])
    case tokenPause([Proto_TokenPauseTransactionBody])
    case tokenRevokeKyc([Proto_TokenRevokeKycTransactionBody])
    case tokenUnfreeze([Proto_TokenUnfreezeAccountTransactionBody])
    case tokenUnpause([Proto_TokenUnpauseTransactionBody])
    case tokenUpdate([Proto_TokenUpdateTransactionBody])
    case tokenWipe([Proto_TokenWipeAccountTransactionBody])
    case systemDelete([Proto_SystemDeleteTransactionBody])
    case systemUndelete([Proto_SystemUndeleteTransactionBody])
    case freeze([Proto_FreezeTransactionBody])
    case scheduleCreate([Proto_ScheduleCreateTransactionBody])
    case scheduleSign([Proto_ScheduleSignTransactionBody])
    case scheduleDelete([Proto_ScheduleDeleteTransactionBody])
    case ethereum([Proto_EthereumTransactionBody])
    case prng([Proto_UtilPrngTransactionBody])

    internal mutating func append(_ transaction: Proto_TransactionBody.OneOf_Data) throws {
        switch (self, transaction) {
        case (.accountCreate(var array), .cryptoCreateAccount(let data)):
            array.append(data)
            self = .accountCreate(array)

        case (.accountUpdate(var array), .cryptoUpdateAccount(let data)):
            array.append(data)
            self = .accountUpdate(array)

        case (.accountDelete(var array), .cryptoDelete(let data)):
            array.append(data)
            self = .accountDelete(array)

        case (.accountAllowanceApprove(var array), .cryptoApproveAllowance(let data)):
            array.append(data)
            self = .accountAllowanceApprove(array)

        case (.accountAllowanceDelete(var array), .cryptoDeleteAllowance(let data)):
            array.append(data)
            self = .accountAllowanceDelete(array)

        case (.contractCreate(var array), .contractCreateInstance(let data)):
            array.append(data)
            self = .contractCreate(array)

        case (.contractUpdate(var array), .contractUpdateInstance(let data)):
            array.append(data)
            self = .contractUpdate(array)

        case (.contractDelete(var array), .contractDeleteInstance(let data)):
            array.append(data)
            self = .contractDelete(array)

        case (.contractExecute(var array), .contractCall(let data)):
            array.append(data)
            self = .contractExecute(array)

        case (.transfer(var array), .cryptoTransfer(let data)):
            array.append(data)
            self = .transfer(array)

        case (.topicCreate(var array), .consensusCreateTopic(let data)):
            array.append(data)
            self = .topicCreate(array)

        case (.topicUpdate(var array), .consensusUpdateTopic(let data)):
            array.append(data)
            self = .topicUpdate(array)

        case (.topicDelete(var array), .consensusDeleteTopic(let data)):
            array.append(data)
            self = .topicDelete(array)

        case (.topicMessageSubmit(var array), .consensusSubmitMessage(let data)):
            array.append(data)
            self = .topicMessageSubmit(array)

        case (.fileAppend(var array), .fileAppend(let data)):
            array.append(data)
            self = .fileAppend(array)

        case (.fileCreate(var array), .fileCreate(let data)):
            array.append(data)
            self = .fileCreate(array)

        case (.fileUpdate(var array), .fileUpdate(let data)):
            array.append(data)
            self = .fileUpdate(array)

        case (.fileDelete(var array), .fileDelete(let data)):
            array.append(data)
            self = .fileDelete(array)

        case (.tokenAssociate(var array), .tokenAssociate(let data)):
            array.append(data)
            self = .tokenAssociate(array)

        case (.tokenBurn(var array), .tokenBurn(let data)):
            array.append(data)
            self = .tokenBurn(array)

        case (.tokenCreate(var array), .tokenCreation(let data)):
            array.append(data)
            self = .tokenCreate(array)

        case (.tokenDelete(var array), .tokenDeletion(let data)):
            array.append(data)
            self = .tokenDelete(array)

        case (.tokenDissociate(var array), .tokenDissociate(let data)):
            array.append(data)
            self = .tokenDissociate(array)

        case (.tokenFeeScheduleUpdate(var array), .tokenFeeScheduleUpdate(let data)):
            array.append(data)
            self = .tokenFeeScheduleUpdate(array)

        case (.tokenFreeze(var array), .tokenFreeze(let data)):
            array.append(data)
            self = .tokenFreeze(array)

        case (.tokenGrantKyc(var array), .tokenGrantKyc(let data)):
            array.append(data)
            self = .tokenGrantKyc(array)

        case (.tokenMint(var array), .tokenMint(let data)):
            array.append(data)
            self = .tokenMint(array)

        case (.tokenPause(var array), .tokenPause(let data)):
            array.append(data)
            self = .tokenPause(array)

        case (.tokenRevokeKyc(var array), .tokenRevokeKyc(let data)):
            array.append(data)
            self = .tokenRevokeKyc(array)

        case (.tokenUnfreeze(var array), .tokenUnfreeze(let data)):
            array.append(data)
            self = .tokenUnfreeze(array)

        case (.tokenUnpause(var array), .tokenUnpause(let data)):
            array.append(data)
            self = .tokenUnpause(array)

        case (.tokenUpdate(var array), .tokenUpdate(let data)):
            array.append(data)
            self = .tokenUpdate(array)

        case (.tokenWipe(var array), .tokenWipe(let data)):
            array.append(data)
            self = .tokenWipe(array)

        case (.systemDelete(var array), .systemDelete(let data)):
            array.append(data)
            self = .systemDelete(array)

        case (.systemUndelete(var array), .systemUndelete(let data)):
            array.append(data)
            self = .systemUndelete(array)

        case (.freeze(var array), .freeze(let data)):
            array.append(data)
            self = .freeze(array)

        case (.scheduleCreate(var array), .scheduleCreate(let data)):
            array.append(data)
            self = .scheduleCreate(array)

        case (.scheduleSign(var array), .scheduleSign(let data)):
            array.append(data)
            self = .scheduleSign(array)

        case (.scheduleDelete(var array), .scheduleDelete(let data)):
            array.append(data)
            self = .scheduleDelete(array)

        case (.ethereum(var array), .ethereumTransaction(let data)):
            array.append(data)
            self = .ethereum(array)

        case (.prng(var array), .utilPrng(let data)):
            array.append(data)
            self = .prng(array)

        default:
            throw HError.fromProtobuf("mismatched transaction types")
        }
    }
}

extension ServicesTransactionDataList: TryFromProtobuf {
    internal typealias Protobuf = [Proto_TransactionBody.OneOf_Data]

    // swiftlint:disable:next function_body_length
    internal init(protobuf proto: Protobuf) throws {
        var iter = proto.makeIterator()

        let first = iter.next()!

        var value: Self

        switch first {
        case .contractCall(let data): value = .contractExecute([data])
        case .contractCreateInstance(let data): value = .contractCreate([data])
        case .contractUpdateInstance(let data): value = .contractUpdate([data])
        case .contractDeleteInstance(let data): value = .contractDelete([data])
        case .ethereumTransaction(let data): value = .ethereum([data])
        case .cryptoApproveAllowance(let data): value = .accountAllowanceApprove([data])
        case .cryptoDeleteAllowance(let data): value = .accountAllowanceDelete([data])
        case .cryptoCreateAccount(let data): value = .accountCreate([data])
        case .cryptoDelete(let data): value = .accountDelete([data])
        case .cryptoTransfer(let data): value = .transfer([data])
        case .cryptoUpdateAccount(let data): value = .accountUpdate([data])
        case .fileAppend(let data): value = .fileAppend([data])
        case .fileCreate(let data): value = .fileCreate([data])
        case .fileDelete(let data): value = .fileDelete([data])
        case .fileUpdate(let data): value = .fileUpdate([data])
        case .systemDelete(let data): value = .systemDelete([data])
        case .systemUndelete(let data): value = .systemUndelete([data])
        case .freeze(let data): value = .freeze([data])
        case .consensusCreateTopic(let data): value = .topicCreate([data])
        case .consensusUpdateTopic(let data): value = .topicUpdate([data])
        case .consensusDeleteTopic(let data): value = .topicDelete([data])
        case .consensusSubmitMessage(let data): value = .topicMessageSubmit([data])
        case .tokenCreation(let data): value = .tokenCreate([data])
        case .tokenFreeze(let data): value = .tokenFreeze([data])
        case .tokenUnfreeze(let data): value = .tokenUnfreeze([data])
        case .tokenGrantKyc(let data): value = .tokenGrantKyc([data])
        case .tokenRevokeKyc(let data): value = .tokenRevokeKyc([data])
        case .tokenDeletion(let data): value = .tokenDelete([data])
        case .tokenUpdate(let data): value = .tokenUpdate([data])
        case .tokenMint(let data): value = .tokenMint([data])
        case .tokenBurn(let data): value = .tokenBurn([data])
        case .tokenWipe(let data): value = .tokenWipe([data])
        case .tokenAssociate(let data): value = .tokenAssociate([data])
        case .tokenDissociate(let data): value = .tokenDissociate([data])
        case .tokenFeeScheduleUpdate(let data): value = .tokenFeeScheduleUpdate([data])
        case .tokenPause(let data): value = .tokenPause([data])
        case .tokenUnpause(let data): value = .tokenUnpause([data])
        case .scheduleCreate(let data): value = .scheduleCreate([data])
        case .scheduleDelete(let data): value = .scheduleDelete([data])
        case .scheduleSign(let data): value = .scheduleSign([data])
        case .utilPrng(let data): value = .prng([data])

        case .cryptoAddLiveHash: throw HError.fromProtobuf("Unsupported transaction `AddLiveHashTransaction`")
        case .cryptoDeleteLiveHash: throw HError.fromProtobuf("Unsupported transaction `DeleteLiveHashTransaction`")
        case .uncheckedSubmit: throw HError.fromProtobuf("Unsupported transaction `UncheckedSubmitTransaction`")
        case .nodeStakeUpdate: throw HError.fromProtobuf("Unsupported transaction `NodeStakeUpdateTransaction`")
        }

        for transaction in iter {
            try value.append(transaction)
        }

        self = value
    }
}
