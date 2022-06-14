package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonValue
import jnr.ffi.Pointer
import jnr.ffi.byref.PointerByReference
import java.lang.ref.Cleaner

/**
 * A private key on the Hedera network.
 */
class PrivateKey internal constructor(internal val ptr: Pointer) {
    init {
        cleaner.register(this) {
            CHedera.instance.hedera_private_key_free(ptr)
        }
    }

    companion object {
        private val cleaner: Cleaner = Cleaner.create()

        /**
         * Generates a new Ed25519 private key.
         */
        @JvmStatic
        fun generateEd25519(): PrivateKey {
            return PrivateKey(CHedera.instance.hedera_private_key_generate_ed25519())
        }

        /**
         * Generates a new ECDSA(secp256k1) private key.
         */
        @JvmStatic
        fun generateEcdsaSecp256k1(): PrivateKey {
            return PrivateKey(CHedera.instance.hedera_private_key_generate_ecdsa_secp256k1())
        }

        @JsonCreator
        @JvmStatic
        fun parse(s: String): PrivateKey {
            val privateKey = PointerByReference()
            val err = CHedera.instance.hedera_private_key_from_string(s, privateKey)

            if (err != CHedera.Error.OK) {
                throw RuntimeException(String.format("hedera_private_key_from_string returned with error, %s\n", err))
            }

            return PrivateKey(privateKey.value)
        }
    }

    /**
     * The public key which corresponds to this private key.
     */
    val publicKey: PublicKey
        get() = PublicKey(CHedera.instance.hedera_private_key_get_public_key(ptr))

    @JsonValue
    override fun toString(): String {
        return CHedera.instance.hedera_private_key_to_string(ptr)
    }
}
