package com.hedera.hashgraph.sdk;

import jnr.ffi.LibraryLoader;
import jnr.ffi.LibraryOption;
import jnr.ffi.Pointer;
import jnr.ffi.annotations.Delegate;
import jnr.ffi.annotations.In;
import jnr.ffi.annotations.Out;
import jnr.ffi.byref.NativeLongByReference;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

import static java.nio.file.StandardCopyOption.REPLACE_EXISTING;

class CHedera {
    static LibHedera instance;

    static {
        var os = System.getProperty("os.name").toLowerCase();
        var arch = System.getProperty("os.arch").toLowerCase();
        String libraryName;

        if (os.contains("win")) {
            os = "windows";
            libraryName = "hedera.dll";
        } else if (os.contains("mac")) {
            os = "macos";
            libraryName = "libhedera.dylib";
        } else {
            os = "linux";
            libraryName = "libhedera.so";
        }

        var resourceName = String.format("com/hedera/hashgraph/sdk/native/%s/%s/%s", os, arch, libraryName);

        try (var stream = ClassLoader.getSystemClassLoader().getResourceAsStream(resourceName)) {
            if (stream == null) {
                throw new RuntimeException(String.format("unsupported platform, os: %s, arch: %s", os, arch));
            }

            var temporaryFile = File.createTempFile("chedera", libraryName);
            temporaryFile.deleteOnExit();

            Files.copy(stream, temporaryFile.toPath(), REPLACE_EXISTING);

            instance = LibraryLoader.create(LibHedera.class).option(LibraryOption.LoadNow, true).option(LibraryOption.IgnoreError, true).failImmediately().load(temporaryFile.getAbsolutePath());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * Represents any possible result from a fallible function in the Hedera SDK.
     */
    public enum Error {
        OK, TIMED_OUT, GRPC_STATUS, FROM_PROTOBUF, TRANSACTION_PRE_CHECK_STATUS, TRANSACTION_NO_ID_PRE_CHECK_STATUS, QUERY_PRE_CHECK_STATUS, QUERY_PAYMENT_PRE_CHECK_STATUS, QUERY_NO_PAYMENT_PRE_CHECK_STATUS, BASIC_PARSE, KEY_PARSE, NO_PAYER_ACCOUNT_OR_TRANSACTION_ID, MAX_QUERY_PAYMENT_EXCEEDED, NODE_ACCOUNT_UNKNOWN, RESPONSE_STATUS_UNRECOGNIZED, RECEIPT_STATUS, SIGNATURE, REQUEST_PARSE
    }

    @FunctionalInterface
    protected interface Callback {
        @Delegate
        void invoke(Pointer context, CHedera.Error error, String response);
    }

    protected interface LibHedera {
        /**
         * Construct a Hedera client pre-configured for testnet access.
         */
        Pointer hedera_client_for_testnet();

        /**
         * Release memory associated with the previously-opened Hedera client.
         */
        void hedera_client_free(@In Pointer client);

        /**
         * Parse a Hedera `EntityId` from the passed string.
         */
        Error hedera_entity_id_from_string(@In String s, @Out NativeLongByReference shard, @Out NativeLongByReference realm, @Out NativeLongByReference num);

        /**
         * Returns English-language text that describes the last error.
         * Undefined if there has been no last error.
         */
        String hedera_error_message();

        /**
         * Execute this request against the provided client of the Hedera network.
         */
        Error hedera_execute(@In Pointer client, @In String request, @In Pointer context, Callback callback);
    }
}
