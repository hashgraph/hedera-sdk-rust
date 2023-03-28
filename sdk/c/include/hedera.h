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
  HEDERA_ERROR_KEY_PARSE,
  HEDERA_ERROR_KEY_DERIVE,
  HEDERA_ERROR_SIGNATURE_VERIFY,
} HederaError;

typedef enum HederaHmacVariant {
  HEDERA_HMAC_VARIANT_SHA2_SHA256,
  HEDERA_HMAC_VARIANT_SHA2_SHA384,
  HEDERA_HMAC_VARIANT_SHA2_SHA512,
  HEDERA_HMAC_VARIANT_SHA3_KECCAK256,
} HederaHmacVariant;

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

size_t hedera_crypto_sha3_keccak256_digest(const uint8_t *bytes,
                                           size_t bytes_size,
                                           uint8_t **result_out);

size_t hedera_crypto_sha2_sha256_digest(const uint8_t *bytes,
                                        size_t bytes_size,
                                        uint8_t **result_out);

size_t hedera_crypto_sha2_sha384_digest(const uint8_t *bytes,
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
 * Parse a Hedera private key from the passed der bytes with the given password.
 *
 * # Safety
 * - `der` must be valid for reads of up to `der_size` bytes.
 * - `password` must be a valid string
 * - `key` must be a valid for writes according to [*Rust* pointer rules].
 *   The inner pointer need not point to a valid `PrivateKey`, however.
 *
 * # Errors
 * - [`Error::KeyParse`] if decrypting the private key fails.
 *
 * [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
 */
enum HederaError hedera_private_key_from_encrypted_info(const uint8_t *der,
                                                        size_t der_size,
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

/**
 * Releases memory associated with the public key.
 */
void hedera_public_key_free(struct HederaPublicKey *key);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _HEDERA_H */
