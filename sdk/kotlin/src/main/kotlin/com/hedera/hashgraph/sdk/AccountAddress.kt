package com.hedera.hashgraph.sdk

/**
 * Either `AccountId` or `AccountAlias`. Some transactions and queries
 * accept `AccountAddress` as an input. All transactions and queries
 * return only `AccountId` as an output, however.
 */
sealed class AccountAddress constructor(val shard: Long, val realm: Long)
