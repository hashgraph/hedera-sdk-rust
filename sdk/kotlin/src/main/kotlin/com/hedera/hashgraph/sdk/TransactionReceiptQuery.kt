package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeName

/**
 * Get the receipt of a transaction, given its transaction ID.
 *
 * Once a transaction reaches consensus, then information about whether it succeeded or failed
 * will be available until the end of the receipt period.
 */
@JsonTypeName("transactionReceipt")
class TransactionReceiptQuery(
    /**
     * The ID of the transaction for which the receipt is being requested.
     */
    @set:JvmSynthetic
    @JsonProperty
    var transactionId: String?,

    /**
     * Whether the response should include the receipts of any child transactions spawned by the
     * top-level transaction with the given transaction.
     */
    @set:JvmSynthetic
    @JsonProperty
    var includeChildren: Boolean = false,

    /**
     * Whether receipts of processing duplicate transactions should be returned.
     */
    @set:JvmSynthetic
    @JsonProperty
    var includeDuplicates: Boolean = false,
) : Query<TransactionReceiptResponse>(TransactionReceiptResponse::class.java) {
    /**
     * Sets the ID of the transaction for which the receipt is being requested.
     */
    fun setTransactionId(transactionId: String): TransactionReceiptQuery {
        this.transactionId = transactionId

        return this
    }

    /**
     * Whether the response should include the receipts of any child transactions spawned by the
     * top-level transaction with the given transaction.
     */
    fun setIncludeChildren(includeChildren: Boolean): TransactionReceiptQuery {
        this.includeChildren = includeChildren

        return this
    }

    /**
     * Whether receipts of processing duplicate transactions should be returned.
     */
    fun setIncludeDuplicates(includeDuplicates: Boolean): TransactionReceiptQuery {
        this.includeDuplicates = includeDuplicates

        return this
    }
}

