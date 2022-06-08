package com.hedera.hashgraph.sdk;

import jnr.ffi.Pointer;

public class Client {
    final Pointer ptr;

    private Client(Pointer ptr) {
        this.ptr = ptr;
    }

    public static Client forTestnet() {
        return new Client(CHedera.instance.hedera_client_for_testnet());
    }
}
