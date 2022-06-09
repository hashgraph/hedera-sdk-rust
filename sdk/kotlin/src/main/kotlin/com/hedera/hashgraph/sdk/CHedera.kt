package com.hedera.hashgraph.sdk

import jnr.ffi.LibraryLoader
import jnr.ffi.LibraryOption
import jnr.ffi.Pointer
import jnr.ffi.annotations.Delegate
import jnr.ffi.annotations.In
import jnr.ffi.annotations.Out
import jnr.ffi.byref.NativeLongByReference
import java.io.File
import java.io.IOException
import java.nio.file.Files
import java.nio.file.StandardCopyOption
import java.util.*

internal open class CHedera {
    companion object {
        val instance: LibHedera

        init {
            var os = System.getProperty("os.name").lowercase(Locale.getDefault())
            val arch = System.getProperty("os.arch").lowercase(Locale.getDefault())
            var libraryName: String

            if (os.contains("win")) {
                os = "windows"
                libraryName = "hedera.dll"
            } else if (os.contains("mac")) {
                os = "macos"
                libraryName = "libhedera.dylib"
            } else {
                os = "linux"
                libraryName = "libhedera.so"
            }

            val resourceName = String.format("com/hedera/hashgraph/sdk/native/%s/%s/%s", os, arch, libraryName)

            try {
                ClassLoader.getSystemClassLoader().getResourceAsStream(resourceName).use { stream ->
                    if (stream == null) {
                        throw RuntimeException(String.format("unsupported platform, os: %s, arch: %s", os, arch))
                    }

                    val temporaryFile = File.createTempFile("chedera", libraryName)
                    temporaryFile.deleteOnExit()

                    Files.copy(stream, temporaryFile.toPath(), StandardCopyOption.REPLACE_EXISTING)

                    instance = LibraryLoader.create(LibHedera::class.java)
                        .option(LibraryOption.LoadNow, true)
                        .option(LibraryOption.IgnoreError, true)
                        .failImmediately()
                        .load(temporaryFile.absolutePath)
                }
            } catch (e: IOException) {
                throw RuntimeException(e)
            }
        }
    }

    /**
     * Represents any possible result from a fallible function in the Hedera SDK.
     */
    enum class Error {
        OK,
        TIMED_OUT,
        GRPC_STATUS,
        FROM_PROTOBUF,
        TRANSACTION_PRE_CHECK_STATUS,
        TRANSACTION_NO_ID_PRE_CHECK_STATUS,
        QUERY_PRE_CHECK_STATUS,
        QUERY_PAYMENT_PRE_CHECK_STATUS,
        QUERY_NO_PAYMENT_PRE_CHECK_STATUS,
        BASIC_PARSE,
        KEY_PARSE,
        NO_PAYER_ACCOUNT_OR_TRANSACTION_ID,
        MAX_QUERY_PAYMENT_EXCEEDED,
        NODE_ACCOUNT_UNKNOWN,
        RESPONSE_STATUS_UNRECOGNIZED,
        RECEIPT_STATUS,
        SIGNATURE,
        REQUEST_PARSE
    }

    fun interface Callback {
        @Delegate
        operator fun invoke(context: Pointer?, error: Error?, response: String?)
    }

    @Suppress("FunctionName")
    interface LibHedera {
        /**
         * Construct a Hedera client pre-configured for testnet access.
         */
        fun hedera_client_for_testnet(): Pointer?

        /**
         * Release memory associated with the previously-opened Hedera client.
         */
        fun hedera_client_free(@In client: Pointer?)

        /**
         * Parse a Hedera `EntityId` from the passed string.
         */
        fun hedera_entity_id_from_string(
            @In s: String?,
            @Out shard: NativeLongByReference?,
            @Out realm: NativeLongByReference?,
            @Out num: NativeLongByReference?
        ): Error?

        /**
         * Returns English-language text that describes the last error.
         * Undefined if there has been no last error.
         */
        fun hedera_error_message(): String?

        /**
         * Execute this request against the provided client of the Hedera network.
         */
        fun hedera_execute(
            @In client: Pointer?,
            @In request: String?,
            @In context: Pointer?,
            @In callback: Callback?
        ): Error?
    }
}
