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
  HEDERA_ERROR_OK = 0,
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
  HEDERA_ERROR_FREEZE_UNSET_NODE_ACCOUNT_IDS,
  HEDERA_ERROR_MAX_QUERY_PAYMENT_EXCEEDED,
  HEDERA_ERROR_NODE_ACCOUNT_UNKNOWN,
  HEDERA_ERROR_RESPONSE_STATUS_UNRECOGNIZED,
  HEDERA_ERROR_RECEIPT_STATUS,
  HEDERA_ERROR_REQUEST_PARSE,
  HEDERA_ERROR_SIGNATURE_VERIFY,
  HEDERA_ERROR_BAD_ENTITY_ID,
  HEDERA_ERROR_CANNOT_CREATE_CHECKSUM,
} HederaError;

typedef enum HederaHmacVariant {
  HEDERA_HMAC_VARIANT_SHA2_SHA256,
  HEDERA_HMAC_VARIANT_SHA2_SHA512,
  HEDERA_HMAC_VARIANT_SHA3_KECCAK256,
} HederaHmacVariant;

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

typedef struct HederaTransactionSources HederaTransactionSources;

typedef struct HederaAccountId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
  /**
   * Safety:
   * - If `alias` is not null, it must:
   *   - be properly aligned
   *   - be dereferenceable
   *   - point to a valid instance of `PublicKey` (any `PublicKey` that `hedera` provides which hasn't been freed yet)
   */
  struct HederaPublicKey *alias;
  /**
   * Safety:
   * - if `evm_address` is not null, it must:
   * - be properly aligned
   * - be dereferencable
   * - point to an array of 20 bytes
   */
  uint8_t *evm_address;
} HederaAccountId;

typedef struct HederaTimestamp {
  uint64_t secs;
  uint32_t nanos;
} HederaTimestamp;

typedef struct HederaTransactionId {
  struct HederaAccountId account_id;
  struct HederaTimestamp valid_start;
  int32_t nonce;
  bool scheduled;
} HederaTransactionId;

typedef enum HederaErrorDetails_Tag {
  HEDERA_ERROR_DETAILS_NONE,
  HEDERA_ERROR_DETAILS_ERROR_GRPC_STATUS,
  HEDERA_ERROR_DETAILS_ERROR_STATUS_TRANSACTION_ID,
  HEDERA_ERROR_DETAILS_ERROR_STATUS_NO_TRANSACTION_ID,
  HEDERA_ERROR_DETAILS_ERROR_MAX_QUERY_PAYMENT_EXCEEDED,
  HEDERA_ERROR_DETAILS_ERROR_BAD_ENTITY_ID,
} HederaErrorDetails_Tag;

typedef struct HederaErrorStatusTransactionId_Body {
  int32_t status;
  struct HederaTransactionId transaction_id;
} HederaErrorStatusTransactionId_Body;

typedef struct HederaErrorStatusNoTransactionId_Body {
  int32_t status;
} HederaErrorStatusNoTransactionId_Body;

typedef struct HederaErrorMaxQueryPaymentExceeded_Body {
  int64_t max_query_payment;
  int64_t query_cost;
} HederaErrorMaxQueryPaymentExceeded_Body;

typedef struct HederaErrorBadEntityId_Body {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
  uint8_t present_checksum[5];
  uint8_t expected_checksum[5];
} HederaErrorBadEntityId_Body;

typedef struct HederaErrorDetails {
  HederaErrorDetails_Tag tag;
  union {
    struct {
      int32_t error_grpc_status;
    };
    HederaErrorStatusTransactionId_Body ERROR_STATUS_TRANSACTION_ID;
    HederaErrorStatusNoTransactionId_Body ERROR_STATUS_NO_TRANSACTION_ID;
    HederaErrorMaxQueryPaymentExceeded_Body ERROR_MAX_QUERY_PAYMENT_EXCEEDED;
    HederaErrorBadEntityId_Body ERROR_BAD_ENTITY_ID;
  };
} HederaErrorDetails;

typedef struct HederaSigner {
  /**
   * Safety:
   * - Must not be null
   * - must be properly aligned
   * - must be dereferencable in the rust sense.
   */
  const struct HederaPublicKey *public_key;
  /**
   * Safety: It must be safe to send `context` to other threads.
   * Safety: It must be safe to share `context` between threads.
   */
  void *context;
  /**
   * Safety:
   * Must not be null
   * must be callable with the appropriate arguments
   */
  size_t (*sign_func)(void *context, const uint8_t *message, size_t message_size, const uint8_t **signature);
  /**
   * Safety:
   * Must not be null
   * must be callable with the appropriate arguments
   */
  void (*free_signature_func)(void *context, uint8_t *signature, size_t signature_size);
  /**
   * Safety:
   * May be null
   * must be callable with the appropriate arguments
   */
  void (*free_context_func)(void *context);
} HederaSigner;

typedef struct HederaSigners {
  /**
   * may only be null if signers_size is 0.
   */
  const struct HederaSigner *signers;
  size_t signers_size;
  /**
   * Free this array of signers (must *not* free the contexts for the original signers)
   */
  void (*free)(const struct HederaSigner *signers, size_t signers_size);
} HederaSigners;

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

struct HederaErrorDetails hedera_last_error_details(void);

/**
 * Free an array of account IDs.
 *
 * # Safety
 * - `ids` must point to an allocation made by `hedera`.
 * - `ids` must not already have been freed.
 * - `ids` must be valid for `size` elements.
 */
void hedera_account_id_array_free(struct HederaAccountId *ids, size_t size);

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
 * Sets the account that will, by default, be paying for transactions and queries built with
 * this client.
 */
void hedera_client_set_operator(struct HederaClient *client,
                                struct HederaAccountId id,
                                struct HederaPrivateKey *key);

/**
 * Returns `true` if there was an operator and `false` if there wasn't.
 *
 * If this method returns `false`, variables will not be modified.
 */
bool hedera_client_get_operator(struct HederaClient *client,
                                struct HederaAccountId *id_out,
                                struct HederaPrivateKey **key_out);

uint64_t hedera_client_get_max_transaction_fee(struct HederaClient *client);

size_t hedera_client_get_random_node_ids(struct HederaClient *client, struct HederaAccountId **ids);

/**
 * Get all the nodes for the `Client`
 *
 * For internal use _only_.
 *
 * # Safety:
 * - `Client` must be valid for reads.
 * - `ids` must be freed by using `hedera_account_id_array_free`, notably this means that it must *not* be freed with `free`.
 * - the length of `ids` must not be changed.
 */
size_t hedera_client_get_nodes(struct HederaClient *client,
                               struct HederaAccountId **ids);

void hedera_client_set_ledger_id(struct HederaClient *client,
                                 const uint8_t *ledger_id_bytes,
                                 size_t ledger_id_size);

size_t hedera_client_get_ledger_id(struct HederaClient *client, uint8_t **ledger_id_bytes);

void hedera_client_set_auto_validate_checksums(struct HederaClient *client,
                                               bool auto_validate_checksums);

bool hedera_client_get_auto_validate_checksums(struct HederaClient *client);

/**
 * Release memory associated with the previously-opened Hedera client.
 */
void hedera_client_free(struct HederaClient *client);

size_t hedera_crypto_sha3_keccak256_digest(const uint8_t *bytes,
                                           size_t bytes_size,
                                           uint8_t **result_out);

size_t hedera_crypto_sha2_sha256_digest(const uint8_t *bytes,
                                        size_t bytes_size,
                                        uint8_t **result_out);

size_t hedera_crypto_sha2_sha512_digest(const uint8_t *bytes,
                                        size_t bytes_size,
                                        uint8_t **result_out);

/**
 * # Safety
 * - `variant` must be one of the recognized values, it _must not_ be anything else.
 */
void hedera_crypto_pbkdf2_hmac(enum HederaHmacVariant variant,
                               const uint8_t *password,
                               size_t password_size,
                               const uint8_t *salt,
                               size_t salt_size,
                               uint32_t rounds,
                               uint8_t *key_buffer,
                               size_t key_size);

/**
 * Execute this request against the provided client of the Hedera network.
 *
 * # Safety
 * - todo(sr): Missing basically everything
 * - `callback` must not store `response` after it returns.
 */
enum HederaError hedera_execute(const struct HederaClient *client,
                                const char *request,
                                const void *context,
                                struct HederaSigners signers,
                                bool has_timeout,
                                double timeout,
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
 * Parse a Hedera private key from the passed pem encoded string with the given password.
 *
 * # Safety
 * - `pem` must be a valid string
 * - `password` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *   The inner pointer need not point to a valid `PrivateKey`, however.
 *
 * # Errors
 * - [`Error::KeyParse`] if `pem` is not valid PEM.
 * - [`Error::KeyParse`] if the type label (`BEGIN XYZ`) is not `ENCRYPTED PRIVATE KEY`.
 * - [`Error::KeyParse`] if decrypting the private key fails.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_pem_with_password(const char *pem,
                                                           const char *password,
                                                           struct HederaPrivateKey **key);

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
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - `message` must be valid for reads of up to `message_size` bytes.
 * - `buf` must be valid for writes according to [*Rust* pointer rules]
 * - the length of the returned buffer must not be modified.
 * - the returned pointer must NOT be freed with `free`.
 * [*Rust* pointer rules]: <https://doc.rust-lang.org/std/ptr/index.html#safety>
 */
size_t hedera_private_key_sign(struct HederaPrivateKey *key,
                               const uint8_t *message,
                               size_t message_size,
                               uint8_t **buf);

/**
 * Returns true if calling [`derive`](Self::derive) on `key` would succeed.
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
bool hedera_private_key_is_derivable(struct HederaPrivateKey *key);

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
 * Recover a `PrivateKey` from a mnemonic phrase and a passphrase.
 *
 * # Safety
 * - `seed` must be valid for reads of up to `seed_size` bytes according to the [*Rust* pointer rules].
 * - the retured `PrivateKey` must only be freed via [`hedera_private_key_free`], notably, this means that it *must not* be freed with `free`.
 */
struct HederaPrivateKey *hedera_private_key_from_mnemonic_seed(const uint8_t *seed,
                                                               size_t seed_size);

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
 * Verify a `signature` on a `message` with this public key.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 * - `message` must be valid for reads of up to `message_size` message.
 * - `signature` must be valid for reads of up to `signature_size` signature.
 *
 * # Errors
 * - [`Error::SignatureVerify`] if the signature algorithm doesn't match this `PublicKey`.
 * - [`Error::SignatureVerify`] if the signature is invalid for this `PublicKey`.
 */
enum HederaError hedera_public_key_verify(struct HederaPublicKey *key,
                                          const uint8_t *message,
                                          size_t message_size,
                                          const uint8_t *signature,
                                          size_t signature_size);

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
 * Convert this public key into an evm address. The evm address is This is the rightmost 20 bytes of the 32 byte Keccak-256 hash of the ECDSA public key.
 *
 * This function may return `null`, if this function does *not* return null, the returned pointer will be valid for exactly 20 bytes.
 *
 * # Safety
 * - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
uint8_t *hedera_public_key_to_evm_address(struct HederaPublicKey *key);

enum HederaError hedera_public_key_verify_sources(struct HederaPublicKey *key,
                                                  struct HederaTransactionSources *sources);

/**
 * Releases memory associated with the public key.
 */
void hedera_public_key_free(struct HederaPublicKey *key);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_schedule_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_schedule_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * Convert the provided transaction to protobuf-encoded bytes.
 *
 * # Safety
 * - todo(sr): Missing basically everything
 */
enum HederaError hedera_transaction_to_bytes(const char *transaction,
                                             struct HederaSigners signers,
                                             uint8_t **buf,
                                             size_t *buf_size);

enum HederaError hedera_transaction_from_bytes(const uint8_t *bytes,
                                               size_t bytes_size,
                                               const struct HederaTransactionSources **sources_out,
                                               char **transaction_out);

/**
 * Execute this request against the provided client of the Hedera network.
 *
 * # Safety
 * - todo(sr): Missing basically everything
 * - `callback` must not store `response` after it returns.
 */
enum HederaError hedera_transaction_execute(const struct HederaClient *client,
                                            const char *request,
                                            const void *context,
                                            struct HederaSigners signers,
                                            bool has_timeout,
                                            double timeout,
                                            const struct HederaTransactionSources *sources,
                                            void (*callback)(const void *context, enum HederaError err, const char *response));

/**
 * Execute this request against the provided client of the Hedera network.
 *
 * # Safety
 * - todo(sr): Missing basically everything
 * - `callback` must not store `response` after it returns.
 */
enum HederaError hedera_transaction_execute_all(const struct HederaClient *client,
                                                const char *request,
                                                const void *context,
                                                struct HederaSigners signers,
                                                bool has_timeout,
                                                double timeout,
                                                const struct HederaTransactionSources *sources,
                                                void (*callback)(const void *context, enum HederaError err, const char *response));

enum HederaError hedera_transaction_make_sources(const char *transaction,
                                                 struct HederaSigners signers,
                                                 const struct HederaTransactionSources **out);

/**
 * Signs `sources` with the given `signers`
 *
 * # Safety
 * - `sources` must not be null.
 * - `signers` must follow the associated safety requirements.
 */
const struct HederaTransactionSources *hedera_transaction_sources_sign(const struct HederaTransactionSources *sources,
                                                                       struct HederaSigners signers);

/**
 * Signs `sources` with the given `signer`
 *
 * # Safety
 * - `sources` must not be null.
 * - `signer` must follow the associated safety requirements.
 */
const struct HederaTransactionSources *hedera_transaction_sources_sign_single(const struct HederaTransactionSources *sources,
                                                                              struct HederaSigner signer);

/**
 * # Safety
 * - `sources` must be non-null and point to a `HederaTransactionSources` allocated by the Hedera SDK.
 */
void hedera_transaction_sources_free(const struct HederaTransactionSources *sources);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _HEDERA_H */
