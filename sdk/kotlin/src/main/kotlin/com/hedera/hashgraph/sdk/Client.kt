package com.hedera.hashgraph.sdk

import jnr.ffi.Pointer
import jnr.ffi.byref.NativeLongByReference
import java.lang.ref.Cleaner

class Client private constructor(internal val ptr: Pointer) {
    init {
        cleaner.register(this) {
            CHedera.instance.hedera_client_free(ptr)
        }
    }

    companion object {
        private val cleaner: Cleaner = Cleaner.create();

        @JvmStatic
        fun forTestnet(): Client {
            return Client(CHedera.instance.hedera_client_for_testnet()!!)
        }
    }


    /**
     * Gets the account that is, by default, paying for transactions and queries built with
     * this client.
     */
    val payerAccountId: AccountId?
        get() {
            val shard = NativeLongByReference()
            val realm = NativeLongByReference()
            val num = NativeLongByReference()

            if (num.toLong() == 0L) {
                return null
            }

            return AccountId(shard.toLong(), realm.toLong(), num.toLong())
        }

    /**
     * Sets the account that will, by default, be paying for transactions and queries built with
     * this client.
     */
    fun setPayerAccountId(payerAccountId: AccountId): Client {
        CHedera.instance.hedera_client_set_payer_account_id(
            ptr,
            payerAccountId.shard,
            payerAccountId.realm,
            payerAccountId.num
        )

        return this
    }

    /**
     * Adds a signer that will, by default, sign for all transactions and queries built
     * with this client.
     */
    fun addDefaultSigner(privateKey: PrivateKey): Client {
        val signer = CHedera.instance.hedera_signer_private_key(privateKey.ptr)
        CHedera.instance.hedera_client_add_default_signer(ptr, signer)

        return this
    }
}
