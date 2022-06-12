package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty

// TODO: Hbar
// TODO: tokens
@JsonIgnoreProperties("\$type")
class AccountBalanceResponse(
    @JsonProperty
    @JvmField
    val accountId: AccountId,

    @JsonProperty
    @field:JvmField
    val balance: Long
)
