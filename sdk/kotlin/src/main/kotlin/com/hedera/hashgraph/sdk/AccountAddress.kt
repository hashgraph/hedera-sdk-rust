package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import jnr.ffi.byref.NativeLongByReference
import jnr.ffi.byref.PointerByReference

/**
 * Either `AccountId` or `AccountAlias`. Some transactions and queries
 * accept `AccountAddress` as an input. All transactions and queries
 * return only `AccountId` as an output, however.
 */
sealed interface AccountAddress {
    companion object {
        @JvmStatic
        @JsonCreator
        fun parse(s: String): AccountAddress {
            val shard = NativeLongByReference()
            val realm = NativeLongByReference()
            val num = NativeLongByReference()
            val alias = PointerByReference()

            val err = CHedera.instance.hedera_account_address_from_string(s, shard, realm, num, alias)

            if (err != CHedera.Error.OK) {
                throw RuntimeException("oh no")
            }

            if (alias.value != null) {
                return AccountAlias(shard.toLong(), realm.toLong(), PublicKey(alias.value))
            }

            return AccountId(shard.toLong(), realm.toLong(), num.toLong())
        }
    }
}
