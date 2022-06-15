package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty

// TODO: enum Status
// TODO: TransactionId
// TODO: Hash
class TransactionReceipt @JsonCreator constructor(
    /**
     * The consensus status of the transaction; is UNKNOWN if consensus has not been reached, or if
     * the associated transaction did not have a valid payer signature.
     */
    @JsonProperty("status")
    @JvmField
    val status: String,

    /**
     * In the receipt for an [AccountCreateTransaction], the id of the newly created account.
     */
    @JsonProperty("accountId")
    @JvmField
    val accountId: AccountId?,

    /**
     * In the receipt for a [FileCreateTransaction], the id of the newly created file.
     */
    @JsonProperty("fileId")
    @JvmField
    val fileId: FileId?,

    /**
     * In the receipt for a [ContractCreateTransaction], the id of the newly created contract.
     */
    @JsonProperty("contractId")
    @JvmField
    val contractId: ContractId?,

    /**
     * In the receipt for a [TopicCreateTransaction], the id of the newly created topic.
     */
    @JsonProperty("topicId")
    @JvmField
    val topicId: TopicId?,

    /**
     * In the receipt for a [TopicMessageSubmitTransaction], the new sequence number of the topic
     * that received the message.
     */
    @JsonProperty("topicSequenceNumber")
    @JvmField
    val topicSequenceNumber: Long,

    /**
     * In the receipt for a [TopicMessageSubmitTransaction], the new running hash of the
     * topic that received the message.
     */
    @JsonProperty("topicRunningHash")
    @JvmField
    val topicRunningHash: String?,

    /**
     * In the receipt of a [TopicMessageSubmitTransaction], the version of the SHA-384
     * digest used to update the running hash.
     */
    @JsonProperty("topicRunningHashVersion")
    @JvmField
    val topicRunningHashVersion: Long,

    /**
     * In the receipt for a [TokenCreateTransaction], the id of the newly created token.
     */
    @JsonProperty("tokenId")
    @JvmField
    val tokenId: TokenId?,

    /**
     * Populated in the receipt of [TokenMint], [TokenWipe], and [TokenBurn] transactions.
     *
     * For fungible tokens, the current total supply of this token.
     * For non-fungible tokens, the total number of NFTs issued for a given token id.
     */
    @JsonProperty("newTotalSupply")
    @JvmField
    val newTotalSupply: Long,

    /**
     * In the receipt for a [ScheduleCreateTransaction], the id of the newly created schedule.
     */
    @JsonProperty("scheduleId")
    @JvmField
    val scheduleId: ScheduleId?,

    /**
     * In the receipt of a [ScheduleCreateTransaction] or [ScheduleSignTransaction] that resolves
     * to SUCCESS, the [TransactionId] that should be used to query for the receipt or
     * record of the relevant scheduled transaction.
     */
    @JsonProperty("scheduledTransactionId")
    @JvmField
    val scheduledTransactionId: String?,

    /**
     * In the receipt of a [TokenMintTransaction] for tokens of type `NonFungibleUnique`,
     * the serial numbers of the newly created NFTs.
     */
    @JsonProperty("serialNumbers")
    @JvmField
    val serialNumbers: List<Long>
)
