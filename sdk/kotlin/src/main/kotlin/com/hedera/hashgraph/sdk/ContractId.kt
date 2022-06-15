package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonTypeName

/**
 * The unique identifier for a smart contract on Hedera.
 */
@JsonTypeName("contractId")
class ContractId : EntityId, Key {
    constructor(num: Long) : super(num)

    constructor(shard: Long, realm: Long, num: Long) : super(shard, realm, num)

    companion object {
        @JvmStatic
        @JsonCreator
        fun parse(s: String) = EntityId.parse(s).run { ContractId(shard, realm, num) }
    }
}
