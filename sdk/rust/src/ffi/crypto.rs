/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use core::slice;

use hmac::Hmac;
use libc::size_t;
use sha3::Digest;

use crate::ffi::util;

unsafe fn digest<H: Digest>(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    assert!(!bytes.is_null());
    assert!(!result_out.is_null());

    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_size) };

    let result = H::digest(bytes);

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe { super::util::make_bytes(result.to_vec(), result_out) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_crypto_sha3_keccak256_digest(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    // safety: we pass the safety requirements up to the caller.
    unsafe { digest::<sha3::Keccak256>(bytes, bytes_size, result_out) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_crypto_sha2_sha256_digest(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    // safety: we pass the safety requirements up to the caller.
    unsafe { digest::<sha2::Sha256>(bytes, bytes_size, result_out) }
}
#[no_mangle]
pub unsafe extern "C" fn hedera_crypto_sha2_sha384_digest(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    // safety: we pass the safety requirements up to the caller.
    unsafe { digest::<sha2::Sha384>(bytes, bytes_size, result_out) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_crypto_sha2_sha512_digest(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    // safety: we pass the safety requirements up to the caller.
    unsafe { digest::<sha2::Sha512>(bytes, bytes_size, result_out) }
}

// it's weird that I have to allow this,
// since a no-mangle function kinda implies that this is
// usable from elsewhere, but like...
#[allow(dead_code)]
#[repr(C)]
pub enum HmacVariant {
    Sha2Sha256,
    Sha2Sha384,
    Sha2Sha512,
    Sha3Keccak256,
}

/// # Safety
/// - `variant` must be one of the recognized values, it _must not_ be anything else.
#[no_mangle]
pub unsafe extern "C" fn hedera_crypto_pbkdf2_hmac(
    variant: HmacVariant,
    password: *const u8,
    password_size: size_t,
    salt: *const u8,
    salt_size: size_t,
    rounds: u32,
    key_buffer: *mut u8,
    key_size: size_t,
) {
    assert!(!key_buffer.is_null());

    let password = unsafe { util::slice_from_buffer(password, password_size) };
    let salt = unsafe { util::slice_from_buffer(salt, salt_size) };
    let key_buffer = unsafe { slice::from_raw_parts_mut(key_buffer, key_size) };

    match variant {
        HmacVariant::Sha2Sha256 => {
            pbkdf2::pbkdf2::<Hmac<sha2::Sha256>>(password, salt, rounds, key_buffer)
        }
        HmacVariant::Sha2Sha384 => {
            pbkdf2::pbkdf2::<Hmac<sha2::Sha384>>(password, salt, rounds, key_buffer)
        }
        HmacVariant::Sha2Sha512 => {
            pbkdf2::pbkdf2::<Hmac<sha2::Sha512>>(password, salt, rounds, key_buffer)
        }
        HmacVariant::Sha3Keccak256 => {
            pbkdf2::pbkdf2::<Hmac<sha3::Keccak256>>(password, salt, rounds, key_buffer)
        }
    }
}
