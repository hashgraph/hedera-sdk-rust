package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeName

/**
 * Get the balance of a cryptocurrency account.
 *
 * This returns only the balance, so it is a smaller reply
 * than `AccountInfoQuery`, which returns the balance plus
 * additional information.
 */
@JsonTypeName("accountBalance")
class AccountBalanceQuery : Query<AccountBalanceResponse>(AccountBalanceResponse::class.java) {
    /**
     * The account ID for which information is requested.
     */
    @JsonProperty
    var accountId: AccountAddress? = null

    /**
     * The contract ID for which information is requested.
     */
    @JsonProperty
    var contractId: AccountAddress? = null
}
