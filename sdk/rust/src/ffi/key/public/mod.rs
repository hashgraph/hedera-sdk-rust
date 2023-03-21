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

use std::{
    ptr,
    slice,
};

use libc::size_t;

use super::{
    parse_bytes,
    to_bytes,
};
use crate::ffi::error::Error;
use crate::PublicKey;

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

/// Verify a `signature` on a `message` with this public key.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
/// - `message` must be valid for reads of up to `message_size` message.
/// - `signature` must be valid for reads of up to `signature_size` signature.
///
/// # Errors
/// - [`Error::SignatureVerify`] if the signature algorithm doesn't match this `PublicKey`.
/// - [`Error::SignatureVerify`] if the signature is invalid for this `PublicKey`.
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_verify(
    key: *mut PublicKey,
    message: *const u8,
    message_size: size_t,
    signature: *const u8,
    signature_size: size_t,
) -> Error {
    assert!(!message.is_null());
    assert!(!signature.is_null());
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    // safety: caller promises that `message` is valid for r/w of up to `message_size`, which is exactly what `slice::from_raw_parts` wants.
    let message = unsafe { slice::from_raw_parts(message, message_size) };

    // safety: caller promises that `signature` is valid for r/w of up to `signature_size`, which is exactly what `slice::from_raw_parts` wants.
    let signature = unsafe { slice::from_raw_parts(signature, signature_size) };

    ffi_try!(key.verify(message, signature));

    Error::Ok
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

/// Convert this public key into an evm address. The evm address is This is the rightmost 20 bytes of the 32 byte Keccak-256 hash of the ECDSA public key.
///
/// This function may return `null`, if this function does *not* return null, the returned pointer will be valid for exactly 20 bytes.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_evm_address(key: *mut PublicKey) -> *mut u8 {
    let key = unsafe { key.as_ref() }.unwrap();

    let Some(out) = key.to_evm_address() else {
        return ptr::null_mut();
    };

    Box::into_raw(Box::new(out.to_bytes())).cast::<u8>()
}

/// Releases memory associated with the public key.
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_free(key: *mut PublicKey) {
    assert!(!key.is_null());

    let _key = unsafe { Box::from_raw(key) };
}
