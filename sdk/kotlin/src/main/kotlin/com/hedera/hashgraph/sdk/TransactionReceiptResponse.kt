package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty

/**
 * Response from [TransactionReceiptQuery].
 */
@JsonIgnoreProperties("\$type")
class TransactionReceiptResponse @JsonCreator constructor(
    /**
     * The receipt of processing the first consensus transaction with the given id.
     */
    @JsonProperty("receipt")
    @JvmField
    val receipt: TransactionReceipt,

    /**
     * The receipts of processing all transactions with the given id, in consensus time order.
     */
    @JsonProperty("duplicateReceipts")
    @JvmField
    val duplicateReceipts: List<TransactionReceipt>,

    /**
     * The receipts (if any) of all child transactions spawned by the transaction with the
     * given top-level id, in consensus order.
     */
    @JsonProperty("childReceipts")
    @JvmField
    val childReceipts: List<TransactionReceipt>,
)
