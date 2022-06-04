#ifndef _HEDERA_H
#define _HEDERA_H

/* Generated with cbindgen:0.23.0 */

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
  HEDERA_ERROR_NO_PAYER_ACCOUNT_OR_TRANSACTION_ID,
  HEDERA_ERROR_MAX_ATTEMPTS_EXCEEDED,
  HEDERA_ERROR_MAX_QUERY_PAYMENT_EXCEEDED,
  HEDERA_ERROR_NODE_ACCOUNT_UNKNOWN,
  HEDERA_ERROR_RESPONSE_STATUS_UNRECOGNIZED,
  HEDERA_ERROR_RECEIPT_STATUS,
  HEDERA_ERROR_SIGNATURE,
  HEDERA_ERROR_REQUEST_PARSE,
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

/**
 * An opaque signer that can sign Hedera transactions.
 *
 * Intended to be a temporary object that is generalized and passed into
 * a function accepting a `HederaSigner*`. Failure to do so will result in
 * a memory of leak.
 */
typedef struct HederaSigner HederaSigner;

/**
 * The unique identifier for a cryptocurrency account on Hedera.
 */
typedef struct HederaAccountId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
} HederaAccountId;

/**
 * The unique identifier for a smart contract on Hedera.
 */
typedef struct HederaContractId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
} HederaContractId;

/**
 * The unique identifier for a file on Hedera.
 */
typedef struct HederaFileId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
} HederaFileId;

/**
 * The unique identifier for a schedule on Hedera.
 */
typedef struct HederaScheduleId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
} HederaScheduleId;

/**
 * The unique identifier for a token on Hedera.
 */
typedef struct HederaTokenId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
} HederaTokenId;

/**
 * The unique identifier for a topic on Hedera.
 */
typedef struct HederaTopicId {
  uint64_t shard;
  uint64_t realm;
  uint64_t num;
} HederaTopicId;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Returns English-language text that describes the last error. Undefined if there has been
 * no last error.
 */
const char *hedera_error_message(void);

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
enum HederaError hedera_account_id_from_string(const char *s, struct HederaAccountId *id);

/**
 * Construct a Hedera client pre-configured for testnet access.
 */
struct HederaClient *hedera_client_for_testnet(void);

/**
 * Release memory associated with the previously-opened Hedera client.
 */
void hedera_client_free(struct HederaClient *client);

/**
 * Sets the account that will, by default, be paying for transactions and queries built with
 * this client.
 */
void hedera_client_set_payer_account_id(struct HederaClient *client, struct HederaAccountId id);

/**
 * Adds a signer that will, by default, sign for all transactions and queries built
 * with this client.
 *
 * Takes ownership of the passed signer.
 *
 */
void hedera_client_add_default_signer(struct HederaClient *client, struct HederaSigner *signer);

/**
 * Parse a Hedera `ContractId` from the passed string.
 */
enum HederaError hedera_contract_id_from_string(const char *s, struct HederaContractId *id);

/**
 * Execute this request against the provided client of the Hedera network.
 */
enum HederaError hedera_execute(const struct HederaClient *client,
                                const char *request,
                                const void *context,
                                void (*callback)(const void *context, enum HederaError err, const char *response));

/**
 * Parse a Hedera `FileId` from the passed string.
 */
enum HederaError hedera_file_id_from_string(const char *s, struct HederaFileId *id);

/**
 * Generates a new Ed25519 private key.
 */
struct HederaPrivateKey *hedera_private_key_generate_ed25519(void);

/**
 * Generates a new ECDSA(secp256k1) private key.
 */
struct HederaPrivateKey *hedera_private_key_generate_ecdsa_secp256k1(void);

/**
 * Gets the public key which corresponds to this private key.
 */
struct HederaPublicKey *hedera_private_key_get_public_key(struct HederaPrivateKey *key);

/**
 * Parse a Hedera private key from the passed string.
 */
enum HederaError hedera_private_key_from_string(const char *s, struct HederaPrivateKey **key);

/**
 * Format a Hedera private key as a string.
 */
const char *hedera_private_key_to_string(struct HederaPrivateKey *key);

/**
 * Releases memory associated with the private key.
 */
void hedera_private_key_free(struct HederaPrivateKey *key);

/**
 * Parse a Hedera public key from the passed string.
 */
enum HederaError hedera_public_key_from_string(const char *s, struct HederaPublicKey **key);

/**
 * Format a Hedera public key as a string.
 */
const char *hedera_public_key_to_string(struct HederaPublicKey *key);

/**
 * Releases memory associated with the public key.
 */
void hedera_public_key_free(struct HederaPublicKey *key);

/**
 * Parse a Hedera `ScheduleId` from the passed string.
 */
enum HederaError hedera_schedule_id_from_string(const char *s, struct HederaScheduleId *id);

/**
 * Create an opaque signer from a `HederaPrivateKey`.
 */
struct HederaSigner *hedera_signer_private_key(struct HederaPrivateKey *key);

/**
 * Parse a Hedera `TokenId` from the passed string.
 */
enum HederaError hedera_token_id_from_string(const char *s, struct HederaTokenId *id);

/**
 * Parse a Hedera `TopicId` from the passed string.
 */
enum HederaError hedera_topic_id_from_string(const char *s, struct HederaTopicId *id);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _HEDERA_H */
