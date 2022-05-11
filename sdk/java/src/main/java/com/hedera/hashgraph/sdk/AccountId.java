package com.hedera.hashgraph.sdk;

/**
 * The unique identifier for a cryptocurrency account on Hedera.
 */
public final class AccountId extends AccountIdOrAlias {
    public final long num;

    public AccountId(long num) {
        this(0, 0, num);
    }

    public AccountId(long shard, long realm, long num) {
        super(shard, realm);

        this.num = num;
    }
}
