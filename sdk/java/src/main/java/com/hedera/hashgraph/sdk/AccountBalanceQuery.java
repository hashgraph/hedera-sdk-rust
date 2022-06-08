package com.hedera.hashgraph.sdk;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;

/**
 * Get the balance of a cryptocurrency account.
 *
 * This returns only the balance, so it is a smaller reply
 * than `AccountInfoQuery`, which returns the balance plus
 * additional information.
 */
@JsonTypeName("accountBalance")
public final class AccountBalanceQuery extends Query<AccountBalanceResponse> {
    @JsonProperty
    private AccountAddress accountId;

    /**
     * Create a new {@link AccountBalanceQuery} ready for configuration.
     */
    public AccountBalanceQuery() {
        super(AccountBalanceResponse.class);
    }

    /**
     * Gets the account ID for which information is requested.
     */
    public AccountAddress getAccountId() {
        return this.accountId;
    }

    /**
     * Sets the account ID for which information is requested.
     * This is mutually exclusive with `contractId`.
     */
    public AccountBalanceQuery setAccountId(AccountAddress accountId) {
        this.accountId = accountId;
        this.contractId = null;

        return this;
    }

    /**
     * The contract ID for which information is requested.
     */
    @JsonProperty
    private AccountAddress contractId;

    /**
     * Gets the contract ID for which information is requested.
     */
    // TODO: Use ContractIdOrEvmAddress
    public AccountAddress getContractId() {
        return contractId;
    }

    /**
     * Sets the account ID for which information is requested.
     * This is mutually exclusive with `contractId`.
     */
    public AccountBalanceQuery setContractId(AccountAddress contractId) {
        this.contractId = contractId;
        this.accountId = null;

        return this;
    }
}
