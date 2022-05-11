package com.hedera.hashgraph.sdk;

/**
 * Either `AccountId` or `AccountAlias`. Some transactions and queries
 * accept `AccountIdOrAlias` as an input. All transactions and queries
 * return only `AccountId` as an output however.
 */
public sealed class AccountIdOrAlias permits AccountId, AccountAlias {
    public final long shard;

    public final long realm;

    AccountIdOrAlias(long shard, long realm) {
        this.shard = shard;
        this.realm = realm;
    }
}
