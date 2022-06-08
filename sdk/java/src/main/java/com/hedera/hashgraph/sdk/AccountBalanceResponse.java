package com.hedera.hashgraph.sdk;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;

// TODO: Hbar
// TODO: tokens
@JsonIgnoreProperties("$type")
public final class AccountBalanceResponse {
    @JsonProperty
    public AccountId accountId;

    @JsonProperty
    public long balance;
}
