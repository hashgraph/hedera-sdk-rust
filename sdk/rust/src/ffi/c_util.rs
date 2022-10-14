use std::ffi::{
    c_char,
    CString,
};
use std::slice;

use libc::size_t;

/// Free a string returned from a hedera API.
///
/// A function will tell you if the string needs to be freed with this method.
///
/// # Safety:   
/// - `s` must have been allocated by this hedera sdk.
/// - `s` must be valid for reads and writes.
/// - `s` must not be used after this call.
#[no_mangle]
pub unsafe extern "C" fn hedera_string_free(s: *mut c_char) {
    assert!(!s.is_null());

    // safety: function contract promises that we own this.
    drop(unsafe { CString::from_raw(s) });
}

/// Free byte buffer returned from a hedera API.
///
/// A function will tell you if the buffer needs to be freed with this method.
///
/// # Safety
/// - `buf` must have been allocated by this hedera sdk.
/// - `buf` must be valid for reads and writes up to `size`.
/// - `buf` must not be used after this call.
#[no_mangle]
pub unsafe extern "C" fn hedera_bytes_free(buf: *mut u8, size: size_t) {
    assert!(!buf.is_null());

    // safety: function contract promises that we own this `Box<[u8]>`.
    let buf = unsafe {
        let buf = slice::from_raw_parts_mut(buf, size);
        Box::from_raw(buf)
    };

    drop(buf);
}
