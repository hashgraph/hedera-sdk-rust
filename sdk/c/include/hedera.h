#ifndef _HEDERA_H
#define _HEDERA_H

/* Generated with cbindgen:0.23.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

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
 * Parse a Hedera `AccountId` from the passed string.
 */
int hedera_account_id_from_string(const char *s, struct HederaAccountId *id);

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
void hedera_execute(const struct HederaClient *client,
                    const char *request,
                    const void *context,
                    void (*callback)(const void *context, const char *value));

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
