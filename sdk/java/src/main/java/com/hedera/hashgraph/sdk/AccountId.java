package com.hedera.hashgraph.sdk;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import jnr.ffi.byref.NativeLongByReference;

/**
 * The unique identifier for a cryptocurrency account on Hedera.
 */
public final class AccountId extends AccountAddress {
    public final long num;

    public AccountId(long num) {
        this(0, 0, num);
    }

    public AccountId(long shard, long realm, long num) {
        super(shard, realm);

        this.num = num;
    }

    @JsonCreator
    public static AccountId parse(String s) {
        var shard = new NativeLongByReference();
        var realm = new NativeLongByReference();
        var num = new NativeLongByReference();

        var err = CHedera.instance.hedera_entity_id_from_string(s, shard, realm, num);

        if (err != CHedera.Error.OK) {
            throw new RuntimeException("oh no");
        }

        return new AccountId(shard.longValue(), realm.longValue(), num.longValue());
    }

    @Override
    @JsonValue
    public String toString() {
        return String.format("%d.%d.%d", shard, realm, num);
    }
}
