package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonValue
import jnr.ffi.byref.NativeLongByReference
import jnr.ffi.byref.PointerByReference

/**
 * The unique identifier for a cryptocurrency account represented with an
 * alias instead of an account number.
 */
class AccountAlias(val shard: Long, val realm: Long, val alias: PublicKey?) : AccountAddress {
    constructor(alias: PublicKey?) : this(0, 0, alias)

    @JsonValue
    override fun toString(): String {
        return String.format("%d.%d.%s", shard, realm, alias)
    }

    companion object {
        @JvmStatic
        @JsonCreator
        fun parse(s: String): AccountAlias {
            val shard = NativeLongByReference()
            val realm = NativeLongByReference()
            val alias = PointerByReference()

            val err = CHedera.instance.hedera_account_alias_from_string(s, shard, realm, alias)

            if (err != CHedera.Error.OK) {
                throw RuntimeException("oh no")
            }

            return AccountAlias(shard.toLong(), realm.toLong(), PublicKey(alias.value))
        }
    }
}
