package com.hedera.hashgraph.sdk

import jnr.ffi.Pointer

class Client private constructor(val ptr: Pointer) {
    companion object {
        @JvmStatic
        fun forTestnet(): Client {
            return Client(CHedera.instance.hedera_client_for_testnet()!!)
        }
    }
}
