#ifndef _HEDERA_H
#define _HEDERA_H

/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Represents any possible result from a fallible function in the Hedera SDK.
 */
typedef enum HederaError {
  HEDERA_ERROR_OK,
  HEDERA_ERROR_TIMED_OUT,
  HEDERA_ERROR_GRPC_STATUS,
  HEDERA_ERROR_FROM_PROTOBUF,
  HEDERA_ERROR_TRANSACTION_PRE_CHECK_STATUS,
  HEDERA_ERROR_TRANSACTION_NO_ID_PRE_CHECK_STATUS,
  HEDERA_ERROR_QUERY_PRE_CHECK_STATUS,
  HEDERA_ERROR_QUERY_PAYMENT_PRE_CHECK_STATUS,
  HEDERA_ERROR_QUERY_NO_PAYMENT_PRE_CHECK_STATUS,
  HEDERA_ERROR_BASIC_PARSE,
  HEDERA_ERROR_KEY_PARSE,
  HEDERA_ERROR_KEY_DERIVE,
  HEDERA_ERROR_NO_PAYER_ACCOUNT_OR_TRANSACTION_ID,
  HEDERA_ERROR_MAX_QUERY_PAYMENT_EXCEEDED,
  HEDERA_ERROR_NODE_ACCOUNT_UNKNOWN,
  HEDERA_ERROR_RESPONSE_STATUS_UNRECOGNIZED,
  HEDERA_ERROR_RECEIPT_STATUS,
  HEDERA_ERROR_SIGNATURE,
  HEDERA_ERROR_REQUEST_PARSE,
  HEDERA_ERROR_MNEMONIC_PARSE,
  HEDERA_ERROR_MNEMONIC_ENTROPY,
  HEDERA_ERROR_SIGNATURE_VERIFY,
} HederaError;

/**
 * Managed client for use on the Hedera network.
 */
typedef struct HederaClient HederaClient;

/**
 * A private key on the Hedera network.
 */
typedef struct HederaPrivateKey HederaPrivateKey;

/**
 * A public key on the Hedera network.
 */
typedef struct HederaPublicKey HederaPublicKey;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Returns English-language text that describes the last error. `null` if there has been
 * no last error.
 *
 * Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
 *
 * # Safety
 * - the length of the returned string must not be modified.
 * - the returned string must NOT be freed with `free`.
 */
char *hedera_error_message(void);

/**
 * Returns the GRPC status code for the last error. Undefined if the last error was not
 * `HEDERA_ERROR_GRPC_STATUS`.
 */
int32_t hedera_error_grpc_status(void);

/**
 * Returns the hedera services response code for the last error. Undefined if the last error
 * was not `HEDERA_ERROR_PRE_CHECK_STATUS`.
 */
int32_t hedera_error_pre_check_status(void);

/**
 * Parse a Hedera `AccountId` from the passed string.
 */
enum HederaError hedera_account_id_from_string(const char *s,
                                               uint64_t *id_shard,
                                               uint64_t *id_realm,
                                               uint64_t *id_num,
                                               struct HederaPublicKey **id_alias);

/**
 * Free a string returned from a hedera API.
 *
 * A function will tell you if the string needs to be freed with this method.
 *
 * # Safety:
 * - `s` must have been allocated by this hedera sdk.
 * - `s` must be valid for reads and writes.
 * - `s` must not be used after this call.
 */
void hedera_string_free(char *s);

/**
 * Free byte buffer returned from a hedera API.
 *
 * A function will tell you if the buffer needs to be freed with this method.
 *
 * # Safety
 * - `buf` must have been allocated by this hedera sdk.
 * - `buf` must be valid for reads and writes up to `size`.
 * - `buf` must not be used after this call.
 */
void hedera_bytes_free(uint8_t *buf, size_t size);

/**
 * Construct a Hedera client pre-configured for mainnet access.
 */
struct HederaClient *hedera_client_for_mainnet(void);

/**
 * Construct a Hedera client pre-configured for testnet access.
 */
struct HederaClient *hedera_client_for_testnet(void);

/**
 * Construct a Hedera client pre-configured for previewnet access.
 */
struct HederaClient *hedera_client_for_previewnet(void);

/**
 * Release memory associated with the previously-opened Hedera client.
 */
void hedera_client_free(struct HederaClient *client);

/**
 * Sets the account that will, by default, be paying for transactions and queries built with
 * this client.
 */
void hedera_client_set_operator(struct HederaClient *client,
                                uint64_t id_shard,
                                uint64_t id_realm,
                                uint64_t id_num,
                                struct HederaPrivateKey *key);

/**
 * Parse a Hedera `EntityId` from the passed string.
 */
enum HederaError hedera_entity_id_from_string(const char *s,
                                              uint64_t *id_shard,
                                              uint64_t *id_realm,
                                              uint64_t *id_num);

/**
 * Execute this request against the provided client of the Hedera network.
 */
enum HederaError hedera_execute(const struct HederaClient *client,
                                const char *request,
                                const void *context,
                                void (*callback)(const void *context, enum HederaError err, const char *response));

/**
 * Generates a new Ed25519 private key.
 */
struct HederaPrivateKey *hedera_private_key_generate_ed25519(void);

/**
 * Generates a new ECDSA(secp256k1) private key.
 */
struct HederaPrivateKey *hedera_private_key_generate_ecdsa(void);

/**
 * Gets the public key which corresponds to this [`PrivateKey`].
 *
 * # Safety:
 * - `key` must be valid for reads according to [*Rust* pointer rules]
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
struct HederaPublicKey *hedera_private_key_get_public_key(struct HederaPrivateKey *key);

/**
 * Parse a `PrivateKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_bytes(const uint8_t *bytes,
                                               size_t bytes_size,
                                               struct HederaPrivateKey **key);

/**
 * Parse a `PrivateKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a ed25519 `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_bytes_ed25519(const uint8_t *bytes,
                                                       size_t bytes_size,
                                                       struct HederaPrivateKey **key);

/**
 * Parse a `PrivateKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a ECDSA(secp256k1) `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_bytes_ecdsa(const uint8_t *bytes,
                                                     size_t bytes_size,
                                                     struct HederaPrivateKey **key);

/**
 * Parse a `PrivateKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_bytes_der(const uint8_t *bytes,
                                                   size_t bytes_size,
                                                   struct HederaPrivateKey **key);

/**
 * Parse a Hedera private key from the passed string.
 *
 * Optionally strips a `0x` prefix.
 * See [`hedera_private_key_from_bytes`]
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_string(const char *s, struct HederaPrivateKey **key);

/**
 * Parse a `PrivateKey` from a der encoded string.
 *
 * Optionally strips a `0x` prefix.
 * See [`hedera_private_key_from_bytes_der`].
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_string_der(const char *s, struct HederaPrivateKey **key);

/**
 * Parse a Ed25519 `PrivateKey` from a string containing the raw key material.
 *
 * Optionally strips a `0x` prefix.
 * See: [`hedera_private_key_from_bytes_ed25519`]
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a ed25519 `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_string_ed25519(const char *s,
                                                        struct HederaPrivateKey **key);

/**
 * Parse a ECDSA(secp256k1) `PrivateKey` from a string containing the raw key material.
 *
 * Optionally strips a `0x` prefix.
 * See: [`hedera_private_key_from_bytes_ecdsa`]
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a ECDSA(secp256k1) `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_string_ecdsa(const char *s, struct HederaPrivateKey **key);

/**
 * Parse a Hedera private key from the passed pem encoded string
 *
 * # Safety
 * - `pem` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *   The inner pointer need not point to a valid `PrivateKey`, however.
 *
 * # Errors
 * - [`Error::KeyParse`] if `pem` is not valid PEM.
 * - [`Error::KeyParse`] if the type label (BEGIN XYZ) is not `PRIVATE KEY`.
 * - [`Error::KeyParse`] if the data contained inside the PEM is not a valid `PrivateKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_pem(const char *pem, struct HederaPrivateKey **key);

/**
 * Return `key`, serialized as der encoded bytes.
 *
 * Note: the returned `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be valid for reads according to [*Rust* pointer rules]
 * - `buf` must be valid for writes according to [*Rust* pointer rules]
 * - the length of the returned buffer must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
size_t hedera_private_key_to_bytes_der(struct HederaPrivateKey *key, uint8_t **buf);

/**
 * Return `key`, serialized as bytes.
 *
 * Note: `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
 *
 * If this is an ed25519 private key, this is equivalent to [`hedera_private_key_to_bytes_raw`]
 * If this is an ecdsa private key, this is equivalent to [`hedera_private_key_to_bytes_der`]
 * # Safety
 * - `key` must be valid for reads according to [*Rust* pointer rules]
 * - `buf` must be valid for writes according to [*Rust* pointer rules]
 * - the length of the returned buffer must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
size_t hedera_private_key_to_bytes(struct HederaPrivateKey *key, uint8_t **buf);

/**
 * Return `key`, serialized as bytes.
 *
 * Note: `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be valid for reads according to [*Rust* pointer rules]
 * - `buf` must be valid for writes according to [*Rust* pointer rules]
 * - the length of the returned buffer must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
size_t hedera_private_key_to_bytes_raw(struct HederaPrivateKey *key, uint8_t **buf);

/**
 * Format a Hedera private key as a string.
 *
 * Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - the length of the returned string must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
char *hedera_private_key_to_string(struct HederaPrivateKey *key);

/**
 * Format a Hedera private key as a der encoded string.
 *
 * Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - the length of the returned string must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
char *hedera_private_key_to_string_der(struct HederaPrivateKey *key);

/**
 * Format a Hedera private key as a string.
 *
 * Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - the length of the returned string must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
char *hedera_private_key_to_string_raw(struct HederaPrivateKey *key);

/**
 * Returns `true` if `key` is an Ed25519 `PrivateKey`.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
bool hedera_private_key_is_ed25519(struct HederaPrivateKey *key);

/**
 * Returns `true` if `key` is an ECDSA(secp256k1) `PrivateKey`.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
bool hedera_private_key_is_ecdsa(struct HederaPrivateKey *key);

/**
 * Derives a child key based on `index`.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - `derived` must be a pointer that is valid for writes according to the [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyDerive`] if this is an Ecdsa key (unsupported operation)
 * - [`Error::KeyDerive`] if this key has no `chain_code` (key is not derivable)
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_derive(struct HederaPrivateKey *key,
                                           int32_t index,
                                           struct HederaPrivateKey **derived);

/**
 * Derive a `PrivateKey` based on `index`.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - `derived` must be a pointer that is valid for writes according to the [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyDerive`] if this is an Ecdsa key (unsupported operation)
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_legacy_derive(struct HederaPrivateKey *key,
                                                  int64_t index,
                                                  struct HederaPrivateKey **derived);

/**
 * Releases memory associated with the private key.
 */
void hedera_private_key_free(struct HederaPrivateKey *key);

/**
 * Parse a `PublicKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_bytes(const uint8_t *bytes,
                                              size_t bytes_size,
                                              struct HederaPublicKey **key);

/**
 * Parse a `PublicKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a ed25519 `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_bytes_ed25519(const uint8_t *bytes,
                                                      size_t bytes_size,
                                                      struct HederaPublicKey **key);

/**
 * Parse a `PublicKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a ECDSA(secp256k1) `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_bytes_ecdsa(const uint8_t *bytes,
                                                    size_t bytes_size,
                                                    struct HederaPublicKey **key);

/**
 * Parse a `PublicKey` from a sequence of bytes.
 *
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_bytes_der(const uint8_t *bytes,
                                                  size_t bytes_size,
                                                  struct HederaPublicKey **key);

/**
 * Parse a Hedera public key from the passed string.
 *
 * Optionally strips a `0x` prefix.
 * See [`hedera_public_key_from_bytes`]
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_string(const char *s, struct HederaPublicKey **key);

/**
 * Parse a `PublicKey` from a der encoded string.
 *
 * Optionally strips a `0x` prefix.
 * See [`hedera_public_key_from_bytes_der`].
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_string_der(const char *s, struct HederaPublicKey **key);

/**
 * Parse a Ed25519 `PublicKey` from a string containing the raw key material.
 *
 * Optionally strips a `0x` prefix.
 * See: [`hedera_public_key_from_bytes_ed25519`]
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a ed25519 `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_string_ed25519(const char *s, struct HederaPublicKey **key);

/**
 * Parse a ECDSA(secp256k1) `PublicKey` from a string containing the raw key material.
 *
 * Optionally strips a `0x` prefix.
 * See: [`hedera_public_key_from_bytes_ecdsa`]
 *
 * # Safety
 * - `s` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *
 * # Errors
 * - [`Error::KeyParse`] if `s` cannot be parsed into a ECDSA(secp256k1) `PublicKey`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_public_key_from_string_ecdsa(const char *s, struct HederaPublicKey **key);

/**
 * Return `key`, serialized as der encoded bytes.
 *
 * Note: the returned `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be valid for reads according to [*Rust* pointer rules]
 * - `buf` must be valid for writes according to [*Rust* pointer rules]
 * - the length of the returned buffer must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
size_t hedera_public_key_to_bytes_der(struct HederaPublicKey *key, uint8_t **buf);

/**
 * Return `key`, serialized as bytes.
 *
 * Note: `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
 *
 * If this is an ed25519 public key, this is equivalent to [`hedera_public_key_to_bytes_raw`]
 * If this is an ecdsa public key, this is equivalent to [`hedera_public_key_to_bytes_der`]
 * # Safety
 * - `key` must be valid for reads according to [*Rust* pointer rules]
 * - `buf` must be valid for writes according to [*Rust* pointer rules]
 * - the length of the returned buffer must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
size_t hedera_public_key_to_bytes(struct HederaPublicKey *key, uint8_t **buf);

/**
 * Return `key`, serialized as bytes.
 *
 * Note: `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be valid for reads according to [*Rust* pointer rules]
 * - `buf` must be valid for writes according to [*Rust* pointer rules]
 * - the length of the returned buffer must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
size_t hedera_public_key_to_bytes_raw(struct HederaPublicKey *key, uint8_t **buf);

/**
 * Format a Hedera public key as a string.
 *
 * Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - the length of the returned string must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
char *hedera_public_key_to_string(struct HederaPublicKey *key);

/**
 * Format a Hedera public key as a der encoded string.
 *
 * Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - the length of the returned string must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
char *hedera_public_key_to_string_der(struct HederaPublicKey *key);

/**
 * Format a Hedera public key as a string.
 *
 * Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - the length of the returned string must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
char *hedera_public_key_to_string_raw(struct HederaPublicKey *key);

/**
 * Returns `true` if `key` is an Ed25519 `PublicKey`.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
bool hedera_public_key_is_ed25519(struct HederaPublicKey *key);

/**
 * Returns `true` if `key` is an ECDSA(secp256k1) `PublicKey`.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
bool hedera_public_key_is_ecdsa(struct HederaPublicKey *key);

/**
 * Releases memory associated with the public key.
 */
void hedera_public_key_free(struct HederaPublicKey *key);

/**
 * Parse a Hedera `NftId` from the passed string.
 */
enum HederaError hedera_nft_id_from_string(const char *s,
                                           uint64_t *token_id_shard,
                                           uint64_t *token_id_realm,
                                           uint64_t *token_id_num,
                                           uint64_t *serial);

/**
 * Parse a Hedera `NftId` from the passed bytes.
 */
enum HederaError hedera_nft_id_from_bytes(const uint8_t *bytes,
                                          size_t bytes_size,
                                          uint64_t *token_id_shard,
                                          uint64_t *token_id_realm,
                                          uint64_t *token_id_num,
                                          uint64_t *serial);

/**
 * Serialize the passed NftId as bytes
 */
size_t hedera_nft_id_to_bytes(uint64_t token_id_shard,
                              uint64_t token_id_realm,
                              uint64_t token_id_num,
                              uint64_t serial,
                              uint8_t **buf);

/**
 * Subscribe with this request against the provided client of the Hedera network.
 * On successful completion, calls `callback` with `ERROR_OK` and a `NULL` `message`.
 */
enum HederaError hedera_subscribe(const struct HederaClient *client,
                                  const char *request,
                                  const void *context,
                                  void (*callback)(const void *context, enum HederaError err, const char *message));

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _HEDERA_H */
