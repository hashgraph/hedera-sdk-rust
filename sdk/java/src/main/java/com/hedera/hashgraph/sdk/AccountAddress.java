package com.hedera.hashgraph.sdk;

/**
 * Either `AccountId` or `AccountAlias`. Some transactions and queries
 * accept `AccountAddress` as an input. All transactions and queries
 * return only `AccountId` as an output however.
 */
public sealed class AccountAddress permits AccountId, AccountAlias {
    public final long shard;

    public final long realm;

    AccountAddress(long shard, long realm) {
        this.shard = shard;
        this.realm = realm;
    }
}
