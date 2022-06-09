package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty

// TODO: Hbar
// TODO: tokens
@JsonIgnoreProperties("\$type")
class AccountBalanceResponse {
    @JsonProperty
    @JvmField
    val accountId: AccountId? = null

    @JsonProperty
    @JvmField
    val balance: Long = 0
}
