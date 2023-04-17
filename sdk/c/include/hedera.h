#ifndef _HEDERA_H
#define _HEDERA_H

/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

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

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _HEDERA_H */
