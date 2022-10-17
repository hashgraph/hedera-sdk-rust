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

use std::ffi::{
    CStr,
    CString,
};
use std::os::raw::c_char;
use std::ptr;
use std::str::FromStr;

use libc::size_t;

use super::{
    parse_bytes,
    parse_str,
};
use crate::ffi::error::Error;
use crate::ffi::key::to_bytes;
use crate::{
    PrivateKey,
    PublicKey,
};

/// Generates a new Ed25519 private key.
#[no_mangle]
pub extern "C" fn hedera_private_key_generate_ed25519() -> *mut PrivateKey {
    let key = PrivateKey::generate_ed25519();

    Box::into_raw(Box::new(key))
}

/// Generates a new ECDSA(secp256k1) private key.
#[no_mangle]
pub extern "C" fn hedera_private_key_generate_ecdsa() -> *mut PrivateKey {
    let key = PrivateKey::generate_ecdsa();

    Box::into_raw(Box::new(key))
}

/// Gets the public key which corresponds to this [`PrivateKey`].
///
/// # Safety:
/// - `key` must be valid for reads according to [*Rust* pointer rules]
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_get_public_key(key: *mut PrivateKey) -> *mut PublicKey {
    // safety: caller promises that `key` must be valid for reads.
    let sk = unsafe { key.as_ref().unwrap() };

    let pk = sk.public_key();

    Box::into_raw(Box::new(pk))
}

/// Parse a `PrivateKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PrivateKey::from_bytes) }
}

/// Parse a `PrivateKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ed25519 `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_bytes_ed25519(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PrivateKey::from_bytes_ed25519) }
}

/// Parse a `PrivateKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ECDSA(secp256k1) `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_bytes_ecdsa(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PrivateKey::from_bytes_ecdsa) }
}

/// Parse a `PrivateKey` from a sequence of bytes.
///
/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_bytes_der(
    bytes: *const u8,
    bytes_size: size_t,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_bytes(bytes, bytes_size, key, PrivateKey::from_bytes_der) }
}

/// Parse a Hedera private key from the passed string.
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_string(
    s: *const c_char,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PrivateKey::from_str) }
}

/// Parse a `PrivateKey` from a der encoded string.
///
/// Optionally strips a `0x` prefix.
/// See [`hedera_private_key_from_bytes_der`].
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_string_der(
    s: *const c_char,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PrivateKey::from_str_der) }
}

/// Parse a Ed25519 `PrivateKey` from a string containing the raw key material.
///
/// Optionally strips a `0x` prefix.
/// See: [`hedera_private_key_from_bytes_ed25519`]
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a ed25519 `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_string_ed25519(
    s: *const c_char,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PrivateKey::from_str_der) }
}

/// Parse a ECDSA(secp256k1) `PrivateKey` from a string containing the raw key material.
///
/// Optionally strips a `0x` prefix.
/// See: [`hedera_private_key_from_bytes_ecdsa`]
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyParse`] if `s` cannot be parsed into a ECDSA(secp256k1) `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_string_ecdsa(
    s: *const c_char,
    key: *mut *mut PrivateKey,
) -> Error {
    // safety: invariants are passed through from the caller.
    unsafe { parse_str(s, key, PrivateKey::from_str_der) }
}

/// Parse a Hedera private key from the passed pem encoded string
///
/// # Safety
/// - `pem` must be a valid string
/// - `key` must be a valid for writes according to [*Rust* pointer rules].
///   The inner pointer need not point to a valid `PrivateKey`, however.
///
/// # Errors
/// - [`Error::KeyParse`] if `pem` is not valid PEM.
/// - [`Error::KeyParse`] if the type label (BEGIN XYZ) is not `PRIVATE KEY`.
/// - [`Error::KeyParse`] if the data contained inside the PEM is not a valid `PrivateKey`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_from_pem(
    pem: *const c_char,
    key: *mut *mut PrivateKey,
) -> Error {
    assert!(!key.is_null());

    // safety: function contract requires that pem is a valid `CStr`.
    let pem = unsafe { CStr::from_ptr(pem) };
    let parsed = ffi_try!(PrivateKey::from_pem(pem.to_bytes()));

    let parsed = Box::into_raw(Box::new(parsed));
    // safety:
    // function contract requires that `key` points to a valid `*mut *mut PrivateKey`
    // function contract requires us *not* to inspect the old value.
    //  ^ and we don't, which is good.
    unsafe { ptr::write(key, parsed) }

    Error::Ok
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
pub unsafe extern "C" fn hedera_private_key_to_bytes_der(
    key: *mut PrivateKey,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants are passed through from the caller.
    unsafe { to_bytes(key, buf, PrivateKey::to_bytes_der) }
}

/// Return `key`, serialized as bytes.
///
/// Note: `buf` must be freed via `hedera_bytes_free` in order to prevent a memory leak.
///
/// If this is an ed25519 private key, this is equivalent to [`hedera_private_key_to_bytes_raw`]
/// If this is an ecdsa private key, this is equivalent to [`hedera_private_key_to_bytes_der`]
/// # Safety
/// - `key` must be valid for reads according to [*Rust* pointer rules]
/// - `buf` must be valid for writes according to [*Rust* pointer rules]
/// - the length of the returned buffer must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_to_bytes(
    key: *mut PrivateKey,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants are passed through from the caller.
    unsafe { to_bytes(key, buf, PrivateKey::to_bytes) }
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
pub unsafe extern "C" fn hedera_private_key_to_bytes_raw(
    key: *mut PrivateKey,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants are passed through from the caller.
    unsafe { to_bytes(key, buf, PrivateKey::to_bytes_raw) }
}

/// Format a Hedera private key as a string.
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
pub unsafe extern "C" fn hedera_private_key_to_string(key: *mut PrivateKey) -> *mut c_char {
    // potential optimization: `PrivateKey::to_string` just calls `PrivateKey::to_string_der`,
    // so, this function *could* just call `hedera_private_key_to_string_der`
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    // if this unwrap fails the called method's impl has a bug,
    // because hex-encoded anything doesn't contain `\0`.
    CString::new(key.to_string()).unwrap().into_raw()
}

/// Format a Hedera private key as a der encoded string.
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
pub unsafe extern "C" fn hedera_private_key_to_string_der(key: *mut PrivateKey) -> *mut c_char {
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    // if this unwrap fails the called method's impl has a bug,
    // because hex-encoded anything doesn't contain `\0`.
    CString::new(key.to_string_der()).unwrap().into_raw()
}

/// Format a Hedera private key as a string.
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
pub unsafe extern "C" fn hedera_private_key_to_string_raw(key: *mut PrivateKey) -> *mut c_char {
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
/// Returns `true` if `key` is an Ed25519 `PrivateKey`.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_is_ed25519(key: *mut PrivateKey) -> bool {
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    key.is_ed25519()
}

/// Returns `true` if `key` is an ECDSA(secp256k1) `PrivateKey`.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_is_ecdsa(key: *mut PrivateKey) -> bool {
    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    key.is_ecdsa()
}

/// Derives a child key based on `index`.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
/// - `derived` must be a pointer that is valid for writes according to the [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyDerive`] if this is an Ecdsa key (unsupported operation)
/// - [`Error::KeyDerive`] if this key has no `chain_code` (key is not derivable)
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_derive(
    key: *mut PrivateKey,
    index: i32,
    derived: *mut *mut PrivateKey,
) -> Error {
    assert!(!derived.is_null());

    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    let output = ffi_try!(key.derive(index));

    // safety: caller promises that `derived` must be valid for reads
    unsafe {
        ptr::write(derived, Box::into_raw(Box::new(output)));
    }

    Error::Ok
}

/// Derive a `PrivateKey` based on `index`.
///
/// # Safety
/// - `key` must be a pointer that is valid for reads according to the [*Rust* pointer rules].
/// - `derived` must be a pointer that is valid for writes according to the [*Rust* pointer rules].
///
/// # Errors
/// - [`Error::KeyDerive`] if this is an Ecdsa key (unsupported operation)
///  
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_legacy_derive(
    key: *mut PrivateKey,
    index: i64,
    derived: *mut *mut PrivateKey,
) -> Error {
    assert!(!derived.is_null());

    // safety: caller promises that `key` must be valid for reads
    let key = unsafe { key.as_ref().unwrap() };

    let output = ffi_try!(key.legacy_derive(index));

    // safety: caller promises that `derived` must be valid for reads
    unsafe {
        ptr::write(derived, Box::into_raw(Box::new(output)));
    }

    Error::Ok
}

/// Releases memory associated with the private key.
#[no_mangle]
pub unsafe extern "C" fn hedera_private_key_free(key: *mut PrivateKey) {
    assert!(!key.is_null());

    drop(unsafe { Box::from_raw(key) });
}
