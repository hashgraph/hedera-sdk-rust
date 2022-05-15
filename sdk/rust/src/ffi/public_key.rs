use std::os::raw::c_char;
use std::str::FromStr;

use crate::ffi::error::FfiResult;
use crate::ffi::util::cstr_from_ptr;
use crate::PublicKey;

/// Parse a Hedera public key from the passed string.
#[no_mangle]
pub extern "C" fn hedera_public_key_from_string(
    s: *const c_char,
    key: *mut *mut PublicKey,
) -> FfiResult {
    assert!(!key.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(PublicKey::from_str(&s));

    unsafe {
        *key = Box::into_raw(Box::new(parsed));
    }

    FfiResult::Ok
}

/// Releases memory associated with the public key.
#[no_mangle]
pub extern "C" fn hedera_public_key_free(key: *mut PublicKey) {
    assert!(!key.is_null());

    let _key = unsafe { Box::from_raw(key) };
}
