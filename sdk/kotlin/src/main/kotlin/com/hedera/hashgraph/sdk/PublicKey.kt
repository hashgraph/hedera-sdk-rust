package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.annotation.JsonValue
import jnr.ffi.Pointer
import jnr.ffi.byref.PointerByReference
import java.lang.ref.Cleaner

/**
 * A public key on the Hedera network.
 */
@JsonTypeName("single")
class PublicKey internal constructor(internal val ptr: Pointer) : Key {
    init {
        cleaner.register(this) {
            CHedera.instance.hedera_public_key_free(ptr)
        }
    }

    companion object {
        private val cleaner: Cleaner = Cleaner.create()

        @JsonCreator
        @JvmStatic
        fun parse(s: String): PublicKey {
            val publicKey = PointerByReference()
            val err = CHedera.instance.hedera_public_key_from_string(s, publicKey)

            if (err != CHedera.Error.OK) {
                throw RuntimeException(String.format("hedera_public_key_from_string returned with error, %s\n", err))
            }

            return PublicKey(publicKey.value)
        }
    }

    @JsonValue
    override fun toString(): String {
        return CHedera.instance.hedera_public_key_to_string(ptr)
    }
}
