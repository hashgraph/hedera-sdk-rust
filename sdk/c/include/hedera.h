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
  HEDERA_ERROR_BAD_ENTITY_ID,
  HEDERA_ERROR_CANNOT_TO_STRING_WITH_CHECKSUM,
  HEDERA_ERROR_CANNOT_PERFORM_TASK_WITHOUT_LEDGER_ID,
} HederaError;

/**
 * Managed client for use on the Hedera network.
 */
typedef struct HederaClient HederaClient;

/**
 *  `BIP-39` 24-word mnemonic phrase compatible with the Android and iOS mobile wallets.
 */
typedef struct HederaMnemonic HederaMnemonic;

/**
 * A private key on the Hedera network.
 */
typedef struct HederaPrivateKey HederaPrivateKey;

/**
 * A public key on the Hedera network.
 */
typedef struct HederaPublicKey HederaPublicKey;

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
} HederaAccountId;

typedef struct HederaAccountBalance {
  struct HederaAccountId id;
  int64_t hbars;
} HederaAccountBalance;

typedef struct HederaContractId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
  /**
   * # Safety
   * - must either be null or valid for 20 bytes
   * - if allocated by `hedera` it must be freed by hedera
   * - otherwise must *not* be freed by hedera.
   */
  uint8_t *evm_address;
} HederaContractId;

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

typedef struct HederaSemanticVersion {
  /**
   * Increases with incompatible API changes
   */
  uint32_t major;
  /**
   * Increases with backwards-compatible new functionality
   */
  uint32_t minor;
  /**
   * Increases with backwards-compatible bug fixes]
   */
  uint32_t patch;
  /**
   * A pre-release version MAY be denoted by appending a hyphen and a series of dot separated identifiers (https://semver.org/#spec-item-9);
   * so given a semver 0.14.0-alpha.1+21AF26D3, this field would contain ‘alpha.1’
   *
   * treat `null` as an empty string.
   *
   * # Safety
   *
   * - If allocated by Hedera, must be freed with `hedera_string_free`,
   *   notably this means that it must not be freed with `free`.
   * - If *not* allocated by Hedera, must be freed however it normally would,
   *   notably this means that it must not be freed with `hedera_string_free`
   * - This field must be valid for reads (unless it's null)
   * - If this is allocated by Hedera,
   *   this will also be valid for writes *if* the field is non-null,
   *   however, the length of this field must *not* be changed.
   */
  char *prerelease;
  /**
   * Build metadata MAY be denoted by appending a plus sign and a series of dot separated identifiers
   * immediately following the patch or pre-release version (https://semver.org/#spec-item-10);
   * so given a semver 0.14.0-alpha.1+21AF26D3, this field would contain ‘21AF26D3’
   *
   * treat `null` as an empty string.
   *
   * # Safety
   *
   * - If allocated by Hedera, must be freed with `hedera_string_free`,
   *   notably this means that it must not be freed with `free`.
   * - If *not* allocated by Hedera, must be freed however it normally would,
   *   notably this means that it must not be freed with `hedera_string_free`
   * - This field must be valid for reads (unless it's null)
   * - If this is allocated by Hedera,
   *   this will also be valid for writes *if* the field is non-null,
   *   however, the length of this field must *not* be changed.
   */
  char *build;
} HederaSemanticVersion;

typedef struct HederaNetworkVersionInfo {
  /**
   * Version of the protobuf schema in use by the network.
   */
  struct HederaSemanticVersion protobuf_version;
  /**
   * Version of the Hedera services in use by the network.
   */
  struct HederaSemanticVersion services_version;
} HederaNetworkVersionInfo;

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

int32_t hedera_error_receipt_status_status(void);

/**
 * Parse a Hedera `AccountBalance` from the passed bytes.
 */
enum HederaError hedera_account_balance_from_bytes(const uint8_t *bytes,
                                                   size_t bytes_size,
                                                   struct HederaAccountBalance *id);

/**
 * Serialize the passed `AccountBalance` as bytes
 *
 * # Safety
 * - `id` must uphold the safety requirements of `AccountBalance`.
 * - `buf` must be valid for writes.
 * - `buf` must only be freed with `hedera_bytes_free`, notably this means that it must not be freed with `free`.
 */
size_t hedera_account_balance_to_bytes(struct HederaAccountBalance id,
                                       uint8_t **buf);

/**
 * Parse a Hedera `AccountId` from the passed bytes.
 */
enum HederaError hedera_account_id_from_bytes(const uint8_t *bytes,
                                              size_t bytes_size,
                                              struct HederaAccountId *id);

/**
 * Serialize the passed `AccountId` as bytes
 *
 * # Safety
 * - `id` must uphold the safety requirements of `AccountId`.
 * - `buf` must be valid for writes.
 * - `buf` must only be freed with `hedera_bytes_free`, notably this means that it must not be freed with `free`.
 */
size_t hedera_account_id_to_bytes(struct HederaAccountId id,
                                  uint8_t **buf);

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
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_account_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_account_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_assessed_custom_fee_from_bytes(const uint8_t *bytes,
                                                       size_t bytes_size,
                                                       char **s);

enum HederaError hedera_assessed_custom_fee_to_bytes(const char *s,
                                                     uint8_t **buf,
                                                     size_t *buf_size);

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

/**
 * Parse a Hedera `ContractId` from the passed bytes.
 *
 * # Safety
 * - `contract_id` be valid for writes.
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 */
enum HederaError hedera_contract_id_from_bytes(const uint8_t *bytes,
                                               size_t bytes_size,
                                               struct HederaContractId *contract_id);

/**
 * Create a `ContractId` from a `shard.realm.evm_address` set.
 *
 * # Safety
 * - `contract_id` must be valid for writes.
 * - `address` must be valid for reads up until the first `\0` character.
 */
enum HederaError hedera_contract_id_from_evm_address(uint64_t shard,
                                                     uint64_t realm,
                                                     const char *evm_address,
                                                     struct HederaContractId *contract_id);

/**
 * create a `ContractId` from a solidity address.
 *
 * # Safety
 * - `contract_id` must be valid for writes.
 * - `address` must be valid for reads up until the first `\0` character.
 */
enum HederaError hedera_contract_id_from_solidity_address(const char *address,
                                                          struct HederaContractId *contract_id);

/**
 * Serialize the passed `ContractId` as bytes
 *
 * # Safety
 * - `buf` must be valid for writes.
 */
size_t hedera_contract_id_to_bytes(struct HederaContractId contract_id, uint8_t **buf);

/**
 * Serialize the passed `ContractId` as a solidity `address`
 *
 * # Safety
 * - `s` must be valid for writes
 */
enum HederaError hedera_contract_id_to_solidity_address(struct HederaContractId contract_id,
                                                        char **s);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_contract_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_contract_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * Parse a Hedera `FileId` from the passed bytes.
 *
 * # Safety
 * - `file_id_shard`, `file_id_realm`, and `file_id_num` must all be valid for writes.
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 */
enum HederaError hedera_file_id_from_bytes(const uint8_t *bytes,
                                           size_t bytes_size,
                                           uint64_t *file_id_shard,
                                           uint64_t *file_id_realm,
                                           uint64_t *file_id_num);

/**
 * Serialize the passed `FileId` as bytes
 *
 * # Safety
 * - `buf` must be valid for writes.
 */
size_t hedera_file_id_to_bytes(uint64_t file_id_shard,
                               uint64_t file_id_realm,
                               uint64_t file_id_num,
                               uint8_t **buf);

/**
 * Parse a Hedera `TopicId` from the passed bytes.
 *
 * # Safety
 * - `topic_id_shard`, `topic_id_realm`, and `topic_id_num` must all be valid for writes.
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 */
enum HederaError hedera_topic_id_from_bytes(const uint8_t *bytes,
                                            size_t bytes_size,
                                            uint64_t *topic_id_shard,
                                            uint64_t *topic_id_realm,
                                            uint64_t *topic_id_num);

/**
 * Serialize the passed `TopicId` as bytes
 *
 * # Safety
 * - `buf` must be valid for writes.
 */
size_t hedera_topic_id_to_bytes(uint64_t topic_id_shard,
                                uint64_t topic_id_realm,
                                uint64_t topic_id_num,
                                uint8_t **buf);

/**
 * Parse a Hedera `TokenId` from the passed bytes.
 *
 * # Safety
 * - `token_id_shard`, `token_id_realm`, and `token_id_num` must all be valid for writes.
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 */
enum HederaError hedera_token_id_from_bytes(const uint8_t *bytes,
                                            size_t bytes_size,
                                            uint64_t *token_id_shard,
                                            uint64_t *token_id_realm,
                                            uint64_t *token_id_num);

/**
 * Serialize the passed TokenId as bytes
 *
 * # Safety
 * - `buf` must be valid for writes.
 */
size_t hedera_token_id_to_bytes(uint64_t token_id_shard,
                                uint64_t token_id_realm,
                                uint64_t token_id_num,
                                uint8_t **buf);

/**
 * Parse a Hedera `ScheduleId` from the passed bytes.
 *
 * # Safety
 * - `schedule_id_shard`, `schedule_id_realm`, and `schedule_id_num` must all be valid for writes.
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 */
enum HederaError hedera_schedule_id_from_bytes(const uint8_t *bytes,
                                               size_t bytes_size,
                                               uint64_t *schedule_id_shard,
                                               uint64_t *schedule_id_realm,
                                               uint64_t *schedule_id_num);

/**
 * Serialize the passed ScheduleId as bytes
 *
 * # Safety
 * - `buf` must be valid for writes.
 */
size_t hedera_schedule_id_to_bytes(uint64_t schedule_id_shard,
                                   uint64_t schedule_id_realm,
                                   uint64_t schedule_id_num,
                                   uint8_t **buf);

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
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_file_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_file_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

enum HederaError hedera_key_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

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
 * - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
 * - `passphrase` must be valid for reads up until and including the first NUL (`'\0'`) byte.
 * - the retured `PrivateKey` must only be freed via [`hedera_private_key_free`], notably, this means that it *must not* be freed with `free`.
 */
struct HederaPrivateKey *hedera_private_key_from_mnemonic(struct HederaMnemonic *mnemonic,
                                                          const char *passphrase);

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
 * Releases memory associated with the public key.
 */
void hedera_public_key_free(struct HederaPublicKey *key);

/**
 * Parse a `Mnemonic` from a string.
 *
 * # Safety
 * - `s` must be valid for reads up until and including the first NUL (`'\0'`) byte.
 * - `mnemonic` must be valid for writes according to the [*Rust* pointer rules]
 * - if this method returns anything other than [`Error::Ok`],
 *   then the contents of `mnemonic` are undefined and must not be used or inspected.
 * - `mnemonic` must only be freed via [`hedera_mnemonic_free`].
 *   Notably this means that it *must not* be freed with `free`.
 *
 * # Errors
 * - [`Error::MnemonicParse`] if the mnemonic has an invalid length.
 * - [`Error::MnemonicParse`] if the mnemonic uses invalid words.
 * - [`Error::MnemonicParse`] if the mnemonic has an invalid checksum.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_mnemonic_from_string(const char *s, struct HederaMnemonic **mnemonic);

/**
 * Generate a new 24 word mnemonic.
 *
 * # Safety
 * This function is safe. However, there are invariants that must be upheld on the result.
 *
 * - The returned mnemonic must only be freed via [`hedera_mnemonic_free`].
 *   Notably this means that it *must not* be freed with `free`.
 */
struct HederaMnemonic *hedera_mnemonic_generate_24(void);

/**
 * Generate a new 12 word mnemonic.
 *
 * # Safety
 * This function is safe. However, there are invariants that must be upheld on the result.
 *
 * - The returned mnemonic must only be freed via [`hedera_mnemonic_free`].
 *   Notably this means that it *must not* be freed with `free`.
 */
struct HederaMnemonic *hedera_mnemonic_generate_12(void);

/**
 * Returns `true` if `mnemonic` is a legacy mnemonic.
 *
 * # Safety
 * - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
bool hedera_mnemonic_is_legacy(struct HederaMnemonic *mnemonic);

/**
 * Recover a [`PrivateKey`] from `mnemonic`.
 *
 * # Safety
 * - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
 * - `passphrase` must be valid for reads up until and including the first NUL (`'\0'`) byte.
 * - `private_key` must be valid for writes according to the [*Rust* pointer rules].
 * - if this method returns anything other than [`Error::Ok`],
 *   then the contents of `private_key` are undefined and must not be used or inspected.
 * - `private_key` must only be freed via `hedera_private_key_free`.
 *   Notably, this means that it *must not* be freed with `free`.
 *
 * # Errors
 * - [`Error::MnemonicEntropy`] if this is a legacy private key, and the passphrase isn't empty.
 * - [`Error::MnemonicEntropy`] if this is a legacy private key,
 *   and the `Mnemonic`'s checksum doesn't match up with the computed one.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_mnemonic_to_private_key(struct HederaMnemonic *mnemonic,
                                                const char *passphrase,
                                                struct HederaPrivateKey **private_key);

/**
 * Recover a [`PrivateKey`] from `mnemonic`.
 *
 * # Safety
 * - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
 * - `private_key` must be valid for writes according to the [*Rust* pointer rules].
 * - if this method returns anything other than [`Error::Ok`],
 *   then the contents of `private_key` are undefined and must not be used or inspected.
 * - `private_key` must only be freed via `hedera_private_key_free`.
 *   Notably, this means that it *must not* be freed with `free`.
 *
 * # Errors
 * - [`Error::MnemonicEntropy`] if the computed checksum doesn't match the actual checksum.
 * - [`Error::MnemonicEntropy`] if this is a v2 legacy mnemonic and doesn't have `24` words.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_mnemonic_to_legacy_private_key(struct HederaMnemonic *mnemonic,
                                                       struct HederaPrivateKey **private_key);

/**
 * Format `mnemonic` as a string.
 *
 * # Safety
 * - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
char *hedera_mnemonic_to_string(struct HederaMnemonic *mnemonic);

/**
 * Free `mnemonic` and release all resources associated with it.
 *
 * # Safety
 * - `mnemonic` must be valid for reads and writes according to the [*Rust* pointer rules].
 * - `mnemonic` must not be used at all after this function is called.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
void hedera_mnemonic_free(struct HederaMnemonic *mnemonic);

enum HederaError hedera_network_version_info_from_bytes(const uint8_t *bytes,
                                                        size_t bytes_size,
                                                        struct HederaNetworkVersionInfo *info);

size_t hedera_network_version_info_to_bytes(struct HederaNetworkVersionInfo info, uint8_t **buf);

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
 * Serialize the passed `NftId` as bytes
 */
size_t hedera_nft_id_to_bytes(uint64_t token_id_shard,
                              uint64_t token_id_realm,
                              uint64_t token_id_num,
                              uint64_t serial,
                              uint8_t **buf);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_node_address_book_from_bytes(const uint8_t *bytes,
                                                     size_t bytes_size,
                                                     char **s);

enum HederaError hedera_node_address_book_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_schedule_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_schedule_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

enum HederaError hedera_semantic_version_from_bytes(const uint8_t *bytes,
                                                    size_t bytes_size,
                                                    struct HederaSemanticVersion *semver);

enum HederaError hedera_semantic_version_from_string(const char *s,
                                                     struct HederaSemanticVersion *semver);

size_t hedera_semantic_version_to_bytes(struct HederaSemanticVersion semver, uint8_t **buf);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_staking_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_staking_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * Subscribe with this request against the provided client of the Hedera network.
 * On successful completion, calls `callback` with `ERROR_OK` and a `NULL` `message`.
 */
enum HederaError hedera_subscribe(const struct HederaClient *client,
                                  const char *request,
                                  const void *context,
                                  void (*callback)(const void *context, enum HederaError err, const char *message));

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_token_association_from_bytes(const uint8_t *bytes,
                                                     size_t bytes_size,
                                                     char **s);

enum HederaError hedera_token_association_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_token_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_token_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_token_nft_info_from_bytes(const uint8_t *bytes,
                                                  size_t bytes_size,
                                                  char **s);

enum HederaError hedera_token_nft_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_topic_info_from_bytes(const uint8_t *bytes, size_t bytes_size, char **s);

enum HederaError hedera_topic_info_to_bytes(const char *s, uint8_t **buf, size_t *buf_size);

/**
 * # Safety
 * - `s` must be a valid string
 * - `transaction_id` must be a valid for writes according to [*Rust* pointer rules].
 */
enum HederaError hedera_transaction_id_from_string(const char *s,
                                                   struct HederaTransactionId *transation_id);

enum HederaError hedera_transaction_id_from_bytes(const uint8_t *bytes,
                                                  size_t bytes_size,
                                                  struct HederaTransactionId *transation_id);

size_t hedera_transaction_id_to_bytes(struct HederaTransactionId id, uint8_t **buf);

/**
 * # Safety
 * - `bytes` must be valid for reads of up to `bytes_size` bytes.
 * - `s` must only be freed with `hedera_string_free`,
 *   notably this means it must not be freed with `free`.
 */
enum HederaError hedera_transaction_receipt_from_bytes(const uint8_t *bytes,
                                                       size_t bytes_size,
                                                       char **s);

enum HederaError hedera_transaction_receipt_to_bytes(const char *s,
                                                     uint8_t **buf,
                                                     size_t *buf_size);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _HEDERA_H */
