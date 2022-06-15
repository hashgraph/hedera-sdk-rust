package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator

/**
 * The unique identifier for a schedule on Hedera.
 */
class ScheduleId : EntityId {
    constructor(num: Long) : super(num)

    constructor(shard: Long, realm: Long, num: Long) : super(shard, realm, num)

    companion object {
        @JvmStatic
        @JsonCreator
        fun parse(s: String) = EntityId.parse(s).run { ScheduleId(shard, realm, num) }
    }
}
