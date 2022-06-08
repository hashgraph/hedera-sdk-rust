package com.hedera.hashgraph.sdk;

import com.fasterxml.jackson.annotation.JsonValue;

/**
 * The unique identifier for a cryptocurrency account represented with an
 * alias instead of an account number.
 */
public final class AccountAlias extends AccountAddress {
    public final PublicKey alias;

    public AccountAlias(PublicKey alias) {
        this(0, 0, alias);
    }

    public AccountAlias(long shard, long realm, PublicKey alias) {
        super(shard, realm);

        this.alias = alias;
    }

    @Override
    @JsonValue
    public String toString() {
        return String.format("%d.%d.%s", shard, realm, alias);
    }
}
