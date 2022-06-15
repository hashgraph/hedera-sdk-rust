package com.hedera.hashgraph.sdk

import jnr.ffi.LibraryLoader
import jnr.ffi.LibraryOption
import jnr.ffi.Pointer
import jnr.ffi.annotations.Delegate
import jnr.ffi.annotations.In
import jnr.ffi.annotations.Out
import jnr.ffi.byref.NativeLongByReference
import jnr.ffi.byref.PointerByReference
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
        fun hedera_client_for_testnet(): Pointer

        /**
         * Release memory associated with the previously-opened Hedera client.
         */
        fun hedera_client_free(@In client: Pointer)

        /**
         * Sets the account that will, by default, be paying for transactions and queries built with
         * this client.
         */
        fun hedera_client_set_payer_account_id(@In client: Pointer, @In shard: Long, @In realm: Long, @In num: Long);

        /**
         * Gets the account that is, by default, paying for transactions and queries built with
         * this client.
         */
        fun hedera_client_get_payer_account_id(
            @In client: Pointer,
            @Out shard: NativeLongByReference,
            @Out realm: NativeLongByReference,
            @Out num: NativeLongByReference
        );

        /**
         * Adds a signer that will, by default, sign for all transactions and queries built
         * with this client.
         *
         * Takes ownership of the passed signer.
         */
        fun hedera_client_add_default_signer(@In client: Pointer, @In signer: Pointer);

        /**
         * Create an opaque signer from a private key.
         */
        fun hedera_signer_private_key(@In privateKey: Pointer): Pointer;

        /**
         * Parse a Hedera `EntityId` from the passed string.
         */
        fun hedera_entity_id_from_string(
            @In s: String,
            @Out shard: NativeLongByReference,
            @Out realm: NativeLongByReference,
            @Out num: NativeLongByReference
        ): Error?

        /**
         * Parse a Hedera `AccountAlias` from the passed string.
         */
        fun hedera_account_alias_from_string(
            @In s: String,
            @Out shard: NativeLongByReference,
            @Out realm: NativeLongByReference,
            @Out alias: PointerByReference
        ): Error?

        /**
         * Parse a Hedera `AccountAddress` from the passed string.
         */
        fun hedera_account_address_from_string(
            @In s: String,
            @Out shard: NativeLongByReference,
            @Out realm: NativeLongByReference,
            @Out num: NativeLongByReference,
            @Out alias: PointerByReference
        ): Error?

        /**
         * Generates a new Ed25519 private key.
         */
        fun hedera_private_key_generate_ed25519(): Pointer

        /**
         * Generates a new ECDSA(secp256k1) private key.
         */
        fun hedera_private_key_generate_ecdsa_secp256k1(): Pointer

        /**
         * Gets the public key which corresponds to this private key.
         */
        fun hedera_private_key_get_public_key(
            @Out privateKey: Pointer
        ): Pointer

        /**
         * Parse a Hedera private key from the passed string.
         */
        fun hedera_private_key_from_string(
            @In s: String,
            @Out privateKey: PointerByReference
        ): Error?

        /**
         * Format a Hedera private key as a string.
         */
        fun hedera_private_key_to_string(@In privateKey: Pointer): String

        /**
         * Parse a Hedera public key from the passed string.
         */
        fun hedera_public_key_from_string(
            @In s: String,
            @Out publicKey: PointerByReference
        ): Error?

        /**
         * Format a Hedera public key as a string.
         */
        fun hedera_public_key_to_string(@In publicKey: Pointer): String

        /**
         * Releases memory associated with the private key.
         */
        fun hedera_private_key_free(@In privateKey: Pointer);

        /**
         * Releases memory associated with the public key.
         */
        fun hedera_public_key_free(@In publicKey: Pointer);

        /**
         * Returns English-language text that describes the last error.
         * Undefined if there has been no last error.
         */
        fun hedera_error_message(): String

        /**
         * Execute this request against the provided client of the Hedera network.
         */
        fun hedera_execute(
            @In client: Pointer,
            @In request: String,
            @In context: Pointer?,
            @In callback: Callback
        ): Error?
    }
}
