use std::ffi::c_char;
use std::{
    ptr,
    slice,
};

use libc::size_t;

use super::error::Error;
use crate::ffi::util::cstr_from_ptr;

mod private;
mod public;

/// Parse a `key` with the given function.
///
/// Internal function to reduce boilerplate.
///
/// # Safety
/// - `s` must be a valid string
/// - `key` must be valid for writes according to [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety

#[track_caller]
#[inline]
unsafe fn parse_str<T, F>(s: *const c_char, key: *mut *mut T, f: F) -> Error
where
    F: FnOnce(&str) -> crate::Result<T>,
{
    assert!(!key.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    // perf note: it's perfectly fine to do this, Rust is really smart when you pass function pointers of static functions.
    let parsed = ffi_try!(f(&s));

    // safety: caller promises that `key` is valid for writes.
    unsafe { ptr::write(key, Box::into_raw(Box::new(parsed))) }

    Error::Ok
}

// note: this function fails to compile on platforms where `size_t != usize` or `c_char != u8`.
// very very *very* little can even remotely be done about that without explicitly supporting them.
// those platforms are very esoteric and will very likely not see usage by this library, so, we can live with this solution.
/// Parse a `PrivateKey` with the given function.
///
/// Internal function to reduce boilerplate.
///
/// # Safety
/// - `bytes` must be valid for reads and writes of up to `bytes_size` bytes.
/// - `key` must be valid for according to [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[track_caller]
#[inline]
unsafe fn parse_bytes<T, F>(bytes: *const u8, bytes_size: size_t, key: *mut *mut T, f: F) -> Error
where
    F: FnOnce(&[u8]) -> crate::Result<T>,
{
    assert!(!bytes.is_null());
    assert!(!key.is_null());

    // safety: caller promises that `bytes` is valid for r/w of up to `bytes_size`, which is exactly what `slice::from_raw_parts` wants.
    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    // perf note: it's perfectly fine to do this, Rust is really smart when you pass function pointers of static functions.
    let parsed = ffi_try!(f(bytes));

    // safety: caller promises that `key` is valid for writes.
    unsafe { ptr::write(key, Box::into_raw(Box::new(parsed))) }

    Error::Ok
}

/// Return a `key`, serialized as the specified flavor of bytes.
///
/// Note: the returned buf must be freed via `hedera_bytes_free` in order to prevent a memory leak.
///
/// # Safety
/// - `key` must be valid for reads according to [*Rust* pointer rules]
/// - `buf` must be valid for writes according to [*Rust* pointer rules]
/// - the length of the returned buffer must not be modified.
/// - the returned pointer must NOT be freed with `free`.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[track_caller]
#[inline]
unsafe fn to_bytes<T, F>(key: *const T, buf: *mut *mut u8, f: F) -> size_t
where
    F: FnOnce(&T) -> Vec<u8>,
{
    // todo: use `as_maybe_uninit_ref` once that's stable.
    assert!(!buf.is_null());

    // safety: invariants promise that `key` must be valid for reads.
    let key = unsafe { key.as_ref().unwrap() };
    let bytes = f(key).into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}
