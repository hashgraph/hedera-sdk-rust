// TODO: exchangeRate
/// The summary of a transaction's result so far, if the transaction has reached consensus.
public struct TransactionReceipt: Codable {
    // TODO: enum Status
    /// The consensus status of the transaction; is UNKNOWN if consensus has not been reached, or if
    /// the associated transaction did not have a valid payer signature.
    public let status: String

    /// In the receipt for an `AccountCreateTransaction`, the id of the newly created account.
    public let accountId: AccountId?

    /// In the receipt for a `FileCreateTransaction`, the id of the newly created file.
    public let fileId: FileId?

    /// In the receipt for a `ContractCreateTransaction`, the id of the newly created contract.
    public let contractId: ContractId?

    /// In the receipt for a `TopicCreateTransaction`, the id of the newly created topic.
    public let topicId: TopicId?

    /// In the receipt for a `TopicMessageSubmitTransaction`, the new sequence number of the topic
    /// that received the message.
    public let topicSequenceNumber: UInt64

    // TODO: hash type (?)
    /// In the receipt for a `TopicMessageSubmitTransaction`, the new running hash of the
    /// topic that received the message.
    public let topicRunningHash: String?

    /// In the receipt of a `TopicMessageSubmitTransaction`, the version of the SHA-384
    /// digest used to update the running hash.
    public let topicRunningHashVersion: UInt64

    /// In the receipt for a `TokenCreateTransaction`, the id of the newly created token.
    public let tokenId: TokenId?

    /// Populated in the receipt of `TokenMint`, `TokenWipe`, and `TokenBurn` transactions.
    ///
    /// For fungible tokens, the current total supply of this token.
    /// For non-fungible tokens, the total number of NFTs issued for a given token id.
    ///
    public let newTotalSupply: UInt64

    /// In the receipt for a `ScheduleCreateTransaction`, the id of the newly created schedule.
    public let scheduleId: ScheduleId?

    // TODO: TransactionId type
    /// In the receipt of a `ScheduleCreateTransaction` or `ScheduleSignTransaction` that resolves
    /// to `Success`, the `TransactionId` that should be used to query for the receipt or
    /// record of the relevant scheduled transaction.
    public let scheduledTransactionId: String?

    /// In the receipt of a `TokenMintTransaction` for tokens of type `NonFungibleUnique`,
    /// the serial numbers of the newly created NFTs.
    public let serialNumbers: [UInt64]?

    /// The receipts of processing all transactions with the given id, in consensus time order.
    public let duplicates: [TransactionReceipt]

    /// The receipts (if any) of all child transactions spawned by the transaction with the
    /// given top-level id, in consensus order.
    public let children: [TransactionReceipt]
}
