package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeName

// TODO: use ContractAddress
/**
 * Get the balance of a cryptocurrency account.
 *
 * This returns only the balance, so it is a smaller reply
 * than `AccountInfoQuery`, which returns the balance plus
 * additional information.
 */
@JsonTypeName("accountBalance")
class AccountBalanceQuery(
    /**
     * The account ID for which information is requested.
     */
    @set:JvmSynthetic
    @JsonProperty
    var accountId: AccountAddress? = null,

    /**
     * The contract ID for which information is requested.
     */
    @set:JvmSynthetic
    @JsonProperty
    var contractId: AccountAddress? = null
) : Query<AccountBalanceResponse>(AccountBalanceResponse::class.java) {
    /**
     * Sets the account ID for which information is requested.
     */
    fun setAccountId(accountId: AccountAddress): AccountBalanceQuery {
        this.accountId = accountId

        return this
    }

    /**
     * Sets the contract ID for which information is requested.
     */
    fun setContractId(contractId: AccountAddress): AccountBalanceQuery {
        this.contractId = contractId

        return this
    }
}
