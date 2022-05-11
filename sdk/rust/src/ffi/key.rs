use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::str::FromStr;

use crate::PrivateKey;

/// Parse a Hedera private key from the passed string.
#[no_mangle]
pub extern "C" fn hedera_private_key_from_string(
    s: *const c_char,
    key: *mut *mut PrivateKey,
) -> c_int {
    assert!(!s.is_null());
    assert!(!key.is_null());

    let s = unsafe { CStr::from_ptr(s) };
    let s = match s.to_str() {
        Ok(s) => s,
        Err(error) => {
            todo!()
        }
    };

    let parsed = match PrivateKey::from_str(s) {
        Ok(it) => it,
        Err(error) => {
            todo!();
        }
    };

    unsafe {
        *key = Box::into_raw(Box::new(parsed));
    }

    0
}

/// Releases memory associated with the private key.
#[no_mangle]
pub extern "C" fn hedera_private_key_free(key: *mut PrivateKey) {
    assert!(!key.is_null());

    let _key = unsafe { Box::from_raw(key) };
}
