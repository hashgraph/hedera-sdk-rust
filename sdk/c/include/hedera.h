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
typedef enum HederaResult {
  HEDERA_RESULT_OK = 0,
  HEDERA_RESULT_ERR_TIMED_OUT = 1,
  HEDERA_RESULT_ERR_GRPC_STATUS = 2,
  HEDERA_RESULT_ERR_FROM_PROTOBUF = 3,
  HEDERA_RESULT_ERR_PRE_CHECK_STATUS = 4,
  HEDERA_RESULT_ERR_BASIC_PARSE = 5,
  HEDERA_RESULT_ERR_KEY_PARSE = 6,
  HEDERA_RESULT_ERR_NO_PAYER_ACCOUNT_OR_TRANSACTION_ID = 7,
  HEDERA_RESULT_ERR_MAX_ATTEMPTS_EXCEEDED = 8,
  HEDERA_RESULT_ERR_MAX_QUERY_PAYMENT_EXCEEDED = 9,
  HEDERA_RESULT_ERR_NODE_ACCOUNT_UNKNOWN = 10,
  HEDERA_RESULT_ERR_RESPONSE_STATUS_UNRECOGNIZED = 11,
  HEDERA_RESULT_ERR_SIGNATURE = 12,
  HEDERA_RESULT_ERR_REQUEST_PARSE = 13,
} HederaResult;

/**
 * Managed client for use on the Hedera network.
 */
typedef struct HederaClient HederaClient;

/**
 * A private key on the Hedera network.
 */
typedef struct HederaPrivateKey HederaPrivateKey;

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
 * `HEDERA_RESULT_ERR_GRPC_STATUS`.
 */
int32_t hedera_error_grpc_status(void);

/**
 * Returns the hedera services response code for the last error. Undefined if the last error
 * was not `HEDERA_RESULT_ERR_PRE_CHECK_STATUS`.
 */
int32_t hedera_error_pre_check_status(void);

/**
 * Parse a Hedera `AccountId` from the passed string.
 */
enum HederaResult hedera_account_id_from_string(const char *s, struct HederaAccountId *id);

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
 * Execute this request against the provided client of the Hedera network.
 */
enum HederaResult hedera_execute(const struct HederaClient *client,
                                 const char *request,
                                 const void *context,
                                 void (*callback)(const void *context, enum HederaResult err, const char *response));

/**
 * Parse a Hedera private key from the passed string.
 */
int hedera_private_key_from_string(const char *s, struct HederaPrivateKey **key);

/**
 * Releases memory associated with the private key.
 */
void hedera_private_key_free(struct HederaPrivateKey *key);

/**
 * Create an opaque signer from a `HederaPrivateKey`.
 */
struct HederaSigner *hedera_signer_private_key(struct HederaPrivateKey *key);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _HEDERA_H */
