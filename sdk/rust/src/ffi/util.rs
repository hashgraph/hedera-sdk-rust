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

use std::borrow::Cow;
use std::ffi::{
    CStr,
    CString,
};
use std::os::raw::c_char;
use std::{
    ptr,
    slice,
};

use crate::ffi::error::Error;

pub(crate) unsafe fn cstr_from_ptr<'a>(ptr: *const c_char) -> Cow<'a, str> {
    assert!(!ptr.is_null());

    unsafe { CStr::from_ptr(ptr).to_string_lossy() }
}

/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `s` must only be freed with `hedera_string_free`,
///   notably this means it must not be freed with `free`.
pub(crate) unsafe fn json_from_bytes<T: serde::Serialize, F: FnOnce(&[u8]) -> crate::Result<T>>(
    bytes: *const u8,
    bytes_size: libc::size_t,
    s: *mut *mut c_char,
    f: F,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!s.is_null());

    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(f(bytes));

    let out = serde_json::to_vec(&parsed).unwrap();

    let out = CString::new(out).unwrap().into_raw();

    unsafe {
        ptr::write(s, out);
    }

    Error::Ok
}

pub(crate) unsafe fn json_to_bytes<T: serde::de::DeserializeOwned, F: FnOnce(&T) -> Vec<u8>>(
    s: *const c_char,
    buf: *mut *mut u8,
    buf_size: *mut libc::size_t,
    f: F,
) -> Error {
    assert!(!buf.is_null());
    assert!(!s.is_null());
    assert!(!buf_size.is_null());

    let s = unsafe { cstr_from_ptr(s) };

    let data: T = ffi_try!(serde_json::from_str(&s).map_err(crate::Error::request_parse));

    let bytes = f(&data).into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    unsafe {
        ptr::write(buf, bytes);
        ptr::write(buf_size, len);
    }

    Error::Ok
}
