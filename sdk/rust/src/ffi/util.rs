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
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use libc::size_t;

pub(crate) unsafe fn cstr_from_ptr<'a>(ptr: *const c_char) -> Cow<'a, str> {
    assert!(!ptr.is_null());

    unsafe { CStr::from_ptr(ptr).to_string_lossy() }
}

/// Convert something bytes-like into a format C understands
///
/// # Safety
/// - `buf` must be non-null and writable.
pub(crate) unsafe fn make_bytes<T>(bytes: T, buf: *mut *mut u8) -> libc::size_t
where
    T: Into<Box<[u8]>>,
{
    let bytes = bytes.into();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    unsafe {
        ptr::write(buf, bytes);
    }

    len
}

// fixme: better name
/// Convert something bytes-like into a format C understands
///
/// Unlike [`make_bytes`] this function uses an out-param for `buf_size`
///
/// # Safety
/// - `buf` must be non-null and writable.
/// - `buf_size` must be non-null and writable.
pub(crate) unsafe fn make_bytes2<T>(bytes: T, buf: *mut *mut u8, buf_size: *mut size_t)
where
    T: Into<Box<[u8]>>,
{
    unsafe {
        let size = make_bytes(bytes.into(), buf);
        ptr::write(buf_size, size);
    }
}
