package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonValue

/**
 * The unique identifier for a cryptocurrency account represented with an
 * alias instead of an account number.
 */
class AccountAlias(shard: Long, realm: Long, val alias: PublicKey?) : AccountAddress(shard, realm) {
    constructor(alias: PublicKey?) : this(0, 0, alias)

    @JsonValue
    override fun toString(): String {
        return String.format("%d.%d.%s", shard, realm, alias)
    }
}
