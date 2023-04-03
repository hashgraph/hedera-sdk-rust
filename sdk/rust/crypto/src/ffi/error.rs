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
use std::ptr;

thread_local! {
    static LAST_ERROR: RefCell<Option<crate::Error>> = RefCell::new(None);
    static LAST_ERROR_MESSAGE: RefCell<CString> = RefCell::new(CString::default());
}

macro_rules! ffi_try {
    ($expr:expr) => {{
        match $expr {
            Ok(it) => it,
            Err(error) => {
                return $crate::ffi::error::Error::new(error);
            }
        }
    }};
}

/// Update the most recently set error, for this thread, clearing whatever may have been there before.
pub(crate) fn set_last_error(error: crate::Error) {
    LAST_ERROR.with(|slot| {
        slot.borrow_mut().replace(error);
    });
}

/// Represents any possible result from a fallible function in the Hedera SDK.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(C)]
pub enum Error {
    Ok = 0,
    KeyParse,
    KeyDerive,
    SignatureVerify,
}

impl Error {
    pub(crate) fn new(error: crate::Error) -> Self {
        let err = match &error {
            crate::Error::KeyParse(_) => Self::KeyParse,
            crate::Error::KeyDerive(_) => Self::KeyDerive,
            crate::Error::SignatureVerify(_) => Self::SignatureVerify,
        };

        set_last_error(error);

        err
    }
}

/// Returns English-language text that describes the last error. `null` if there has been
/// no last error.
///
/// Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
///
/// # Safety
/// - the length of the returned string must not be modified.
/// - the returned string must NOT be freed with `free`.
#[no_mangle]
pub extern "C" fn hedera_error_message() -> *mut c_char {
    LAST_ERROR.with(|error| {
        if let Some(error) = &*error.borrow() {
            return CString::new(error.to_string()).unwrap().into_raw();
        }

        ptr::null_mut()
    })
}
