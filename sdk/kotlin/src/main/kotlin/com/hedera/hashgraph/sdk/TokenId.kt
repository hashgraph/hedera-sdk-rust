package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator

/**
 * The unique identifier for a token on Hedera.
 */
class TokenId : EntityId {
    constructor(num: Long) : super(num)

    constructor(shard: Long, realm: Long, num: Long) : super(shard, realm, num)

    companion object {
        @JvmStatic
        @JsonCreator
        fun parse(s: String) = EntityId.parse(s).run { TokenId(shard, realm, num) }
    }
}
