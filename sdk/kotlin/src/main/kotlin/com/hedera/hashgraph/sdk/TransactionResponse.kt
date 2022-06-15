package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.future.future
import kotlinx.coroutines.runBlocking

/**
 * Response from Transaction.execute.
 *
 * When the client sends a node a transaction of any kind, the node replies with this, which
 * simply says that the transaction passed the pre-check (so the node will submit it to
 * the network).
 *
 * To learn the consensus result, the client should later obtain a
 * receipt (free), or can buy a more detailed record (not free).
 */
// TODO: TransactionId
// TODO: Hash
@JsonIgnoreProperties("\$type")
class TransactionResponse(
    /**
     * The account ID of the node that the transaction was submitted to.
     */
    @JsonProperty("nodeAccountId")
    @JvmField
    val nodeAccountId: AccountId,

    /**
     * The client-generated transaction ID of the transaction that was submitted.
     * This can be used to look up the transaction in an explorer.
     */
    @JsonProperty("transactionId")
    @JvmField
    val transactionId: String,

    /**
     * The client-generated SHA-384 hash of the transaction that was submitted.
     * This can be used to lookup the transaction in an explorer.
     */
    @JsonProperty("transactionHash")
    @JvmField
    val transactionHash: String
) {
    /**
     * Get the receipt of this transaction. Will wait for consensus.
     */
    suspend fun getReceipt(client: Client): TransactionReceipt {
        return TransactionReceiptQuery(transactionId = transactionId).execute(client).receipt
    }

    /**
     * Get the receipt of this transaction. Will wait for consensus.
     */
    @JvmName("getReceipt")
    fun getReceiptBlocking(client: Client) = runBlocking { getReceipt(client) }

    /**
     * Get the receipt of this transaction. Will wait for consensus.
     */
    fun getReceiptAsync(client: Client) = GlobalScope.future { getReceipt(client) }
}
