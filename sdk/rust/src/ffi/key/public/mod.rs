/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
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

use std::ffi::CString;
use std::os::raw::c_char;
use std::str::FromStr;

use libc::size_t;

use super::{
    parse_bytes,
    parse_str,
    to_bytes,
};
use crate::ffi::error::Error;
use crate::PublicKey;

#[cfg(test)]
mod tests;

/// Parse a `PublicKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PublicKey::from_bytes) }
}

/// Parse a `PublicKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ed25519 `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_bytes_ed25519(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PublicKey::from_bytes_ed25519) }
}

/// Parse a `PublicKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ECDSA(secp256k1) `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_bytes_ecdsa(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PublicKey::from_bytes_ecdsa) }
}

/// Parse a `PublicKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_bytes_der(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PublicKey::from_bytes_der) }
}

/// Parse a Hedera public key from the passed string.
///
/// Optionally strips a `0x` prefix.
/// See [`hedera_public_key_from_bytes`]
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_string(
    s: *const c_char,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PublicKey::from_str) }
}

/// Parse a `PublicKey` from a der encoded string.
///
/// Optionally strips a `0x` prefix.
/// See [`hedera_public_key_from_bytes_der`].
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_string_der(
    s: *const c_char,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PublicKey::from_str_der) }
}

/// Parse a Ed25519 `PublicKey` from a string containing the raw key material.
///
/// Optionally strips a `0x` prefix.
/// See: [`hedera_public_key_from_bytes_ed25519`]
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a ed25519 `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_string_ed25519(
    s: *const c_char,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PublicKey::from_str_ed25519) }
}

/// Parse a ECDSA(secp256k1) `PublicKey` from a string containing the raw key material.
///
/// Optionally strips a `0x` prefix.
/// See: [`hedera_public_key_from_bytes_ecdsa`]
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a ECDSA(secp256k1) `PublicKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_string_ecdsa(
    s: *const c_char,
    key: *mut *mut PublicKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PublicKey::from_str_ecdsa) }
}

/// Return `key`, serialized as der encoded bytes.
///
/// Note: the returned `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
///
/// # Safety
/// - `key` must be valid for reads according to [*Rust* pointer rules]
/// - `buf` must be valid for writes according to [*Rust* pointer rules]
/// - the length of the returned buffer must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_bytes_der(
    key: *mut PublicKey,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants are passed through from the caller.
    unsafe { to_bytes(key, buf, PublicKey::to_bytes_der) }
}

/// Return `key`, serialized as bytes.
///
/// Note: `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
///
/// If this is an ed25519 public key, this is equivalent to [`hedera_public_key_to_bytes_raw`]
/// If this is an ecdsa public key, this is equivalent to [`hedera_public_key_to_bytes_der`]
/// # Safety
/// - `key` must be valid for reads according to [*Rust* pointer rules]
/// - `buf` must be valid for writes according to [*Rust* pointer rules]
/// - the length of the returned buffer must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_bytes(
    key: *mut PublicKey,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants are passed through from the caller.
    unsafe { to_bytes(key, buf, PublicKey::to_bytes) }
}

/// Return `key`, serialized as bytes.
///
/// Note: `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
///
/// # Safety
/// - `key` must be valid for reads according to [*Rust* pointer rules]
/// - `buf` must be valid for writes according to [*Rust* pointer rules]
/// - the length of the returned buffer must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_bytes_raw(
    key: *mut PublicKey,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants are passed through from the caller.
    unsafe { to_bytes(key, buf, PublicKey::to_bytes_raw) }
}

/// Format a Hedera public key as a string.
///
/// Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
/// - the length of the returned string must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_string(key: *mut PublicKey) -> *mut c_char {
    // potential optimization: `PublicKey::to_string` just calls `PublicKey::to_string_der`,
    // so, this function *could* just call `hedera_public_key_to_string_der`
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    // if this unwrap fails the called method's impl has a bug,
    // because hex-encoded anything doesn't contain `\0`.
    CString::new(key.to_string()).unwrap().into_raw()
}

/// Format a Hedera public key as a der encoded string.
///
/// Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
/// - the length of the returned string must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_string_der(key: *mut PublicKey) -> *mut c_char {
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    // if this unwrap fails the called method's impl has a bug,
    // because hex-encoded anything doesn't contain `\0`.
    CString::new(key.to_string_der()).unwrap().into_raw()
}

/// Format a Hedera public key as a string.
///
/// Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
/// - the length of the returned string must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_string_raw(key: *mut PublicKey) -> *mut c_char {
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    // if this unwrap fails the called method's impl has a bug,
    // because hex-encoded anything doesn't contain `\0`.
    CString::new(key.to_string_raw()).unwrap().into_raw()
}

// ffi note: `bool` is defined to have the same layout as c17's `_Bool`.
// see: https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#bool
// the wording is a little confusing because it says "which is implementation defined"
// but it means that Rust `bool` == c17 `bool` == "something"
/// Returns `true` if `key` is an Ed25519 `PublicKey`.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_is_ed25519(key: *mut PublicKey) -> bool {
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    key.is_ed25519()
}

/// Returns `true` if `key` is an ECDSA(secp256k1) `PublicKey`.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_is_ecdsa(key: *mut PublicKey) -> bool {
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    key.is_ecdsa()
}

/// Releases memory associated with the public key.
#[no_mangle]
pub extern "C" fn hedera_public_key_free(key: *mut PublicKey) {
    assert!(!key.is_null());

    let _key = unsafe { Box::from_raw(key) };
}
