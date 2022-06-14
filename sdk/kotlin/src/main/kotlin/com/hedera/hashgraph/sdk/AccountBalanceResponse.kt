package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty

// TODO: Hbar
// TODO: tokens
@JsonIgnoreProperties("\$type")
class AccountBalanceResponse constructor(
    @JsonProperty("accountId")
    @JvmField
    val accountId: AccountId,

    @JsonProperty("balance")
    @JvmField
    val balance: Long
)
