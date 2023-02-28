internal enum AnySchedulableTransaction {
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
    case scheduleDelete(ScheduleDeleteTransaction)

    internal init(upcasting transaction: Transaction) {
        self.init(AnyTransaction(upcasting: transaction))
    }

    // There's unfortunately very little that can be done about this.
    // swiftlint:disable:next cyclomatic_complexity function_body_length
    fileprivate init(_ anyTransaction: AnyTransaction) {
        switch anyTransaction {
        case .accountCreate(let transaction):
            self = .accountCreate(transaction)
        case .accountUpdate(let transaction):
            self = .accountUpdate(transaction)
        case .accountDelete(let transaction):
            self = .accountDelete(transaction)
        case .accountAllowanceApprove(let transaction):
            self = .accountAllowanceApprove(transaction)
        case .accountAllowanceDelete(let transaction):
            self = .accountAllowanceDelete(transaction)
        case .contractCreate(let transaction):
            self = .contractCreate(transaction)
        case .contractUpdate(let transaction):
            self = .contractUpdate(transaction)
        case .contractDelete(let transaction):
            self = .contractDelete(transaction)
        case .contractExecute(let transaction):
            self = .contractExecute(transaction)
        case .transfer(let transaction):
            self = .transfer(transaction)
        case .topicCreate(let transaction):
            self = .topicCreate(transaction)
        case .topicUpdate(let transaction):
            self = .topicUpdate(transaction)
        case .topicDelete(let transaction):
            self = .topicDelete(transaction)
        case .topicMessageSubmit(let transaction):
            self = .topicMessageSubmit(transaction)
        case .fileAppend(let transaction):
            self = .fileAppend(transaction)
        case .fileCreate(let transaction):
            self = .fileCreate(transaction)
        case .fileUpdate(let transaction):
            self = .fileUpdate(transaction)
        case .fileDelete(let transaction):
            self = .fileDelete(transaction)
        case .tokenAssociate(let transaction):
            self = .tokenAssociate(transaction)
        case .tokenBurn(let transaction):
            self = .tokenBurn(transaction)
        case .tokenCreate(let transaction):
            self = .tokenCreate(transaction)
        case .tokenDelete(let transaction):
            self = .tokenDelete(transaction)
        case .tokenDissociate(let transaction):
            self = .tokenDissociate(transaction)
        case .tokenFeeScheduleUpdate(let transaction):
            self = .tokenFeeScheduleUpdate(transaction)
        case .tokenFreeze(let transaction):
            self = .tokenFreeze(transaction)
        case .tokenGrantKyc(let transaction):
            self = .tokenGrantKyc(transaction)
        case .tokenMint(let transaction):
            self = .tokenMint(transaction)
        case .tokenPause(let transaction):
            self = .tokenPause(transaction)
        case .tokenRevokeKyc(let transaction):
            self = .tokenRevokeKyc(transaction)
        case .tokenUnfreeze(let transaction):
            self = .tokenUnfreeze(transaction)
        case .tokenUnpause(let transaction):
            self = .tokenUnpause(transaction)
        case .tokenUpdate(let transaction):
            self = .tokenUpdate(transaction)
        case .tokenWipe(let transaction):
            self = .tokenWipe(transaction)
        case .systemDelete(let transaction):
            self = .systemDelete(transaction)
        case .systemUndelete(let transaction):
            self = .systemUndelete(transaction)
        case .freeze(let transaction):
            self = .freeze(transaction)
        case .scheduleDelete(let transaction):
            self = .scheduleDelete(transaction)
        case .ethereum:
            fatalError("Cannot schedule `EthereumTransaction`")
        case .scheduleCreate:
            fatalError("Cannot schedule `ScheduleCreateTransaction`")
        case .scheduleSign:
            fatalError("Cannot schedule `ScheduleSignTransaction`")
        }
    }
}

extension AnySchedulableTransaction {
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
        case scheduleDelete
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
        case .scheduleDelete(let transaction):
            return transaction
        }
    }
}

extension AnySchedulableTransaction: Codable {
    internal init(from decoder: Decoder) throws {
        self.init(try AnyTransaction(from: decoder))
    }

    internal func encode(to encoder: Encoder) throws {
        try transaction.encode(to: encoder)
    }
}
