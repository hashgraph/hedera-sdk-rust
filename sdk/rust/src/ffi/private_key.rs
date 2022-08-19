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

use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::str::FromStr;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::{
    PrivateKey,
    PublicKey,
};

/// Generates a new Ed25519 private key.
#[no_mangle]
pub extern "C" fn hedera_private_key_generate_ed25519() -> *mut PrivateKey {
    let key = PrivateKey::generate_ed25519();
    let key = Box::into_raw(Box::new(key));

    key
}

/// Generates a new ECDSA(secp256k1) private key.
#[no_mangle]
pub extern "C" fn hedera_private_key_generate_ecdsa_secp256k1() -> *mut PrivateKey {
    let key = PrivateKey::generate_ecdsa_secp256k1();
    let key = Box::into_raw(Box::new(key));

    key
}

/// Gets the public key which corresponds to this private key.
#[no_mangle]
pub extern "C" fn hedera_private_key_get_public_key(key: *mut PrivateKey) -> *mut PublicKey {
    assert!(!key.is_null());

    let key_private = unsafe { &*key };

    let key_public = key_private.public_key();
    let key_public = Box::into_raw(Box::new(key_public));

    key_public
}

/// Parse a Hedera private key from the passed string.
#[no_mangle]
pub extern "C" fn hedera_private_key_from_string(
    s: *const c_char,
    key: *mut *mut PrivateKey,
) -> Error {
    assert!(!key.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(PrivateKey::from_str(&s));

    unsafe {
        *key = Box::into_raw(Box::new(parsed));
    }

    Error::Ok
}

/// Format a Hedera private key as a string.
#[no_mangle]
pub extern "C" fn hedera_private_key_to_string(key: *mut PrivateKey) -> *const c_char {
    thread_local! {
        static PRIVATE_KEY_DISPLAY: RefCell<Option<CString>> = RefCell::new(None);
    }

    assert!(!key.is_null());

    let key = unsafe { &*key };

    PRIVATE_KEY_DISPLAY
        .with(|cell| cell.borrow_mut().insert(CString::new(key.to_string()).unwrap()).as_ptr())
}

/// Releases memory associated with the private key.
#[no_mangle]
pub extern "C" fn hedera_private_key_free(key: *mut PrivateKey) {
    assert!(!key.is_null());

    let _key = unsafe { Box::from_raw(key) };
}
