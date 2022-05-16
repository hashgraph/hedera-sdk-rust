use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::str::FromStr;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::PublicKey;

/// Parse a Hedera public key from the passed string.
#[no_mangle]
pub extern "C" fn hedera_public_key_from_string(
    s: *const c_char,
    key: *mut *mut PublicKey,
) -> Error {
    assert!(!key.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(PublicKey::from_str(&s));

    unsafe {
        *key = Box::into_raw(Box::new(parsed));
    }

    Error::Ok
}

/// Format a Hedera public key as a string.
#[no_mangle]
pub extern "C" fn hedera_public_key_to_string(key: *mut PublicKey) -> *const c_char {
    thread_local! {
        static PRIVATE_KEY_DISPLAY: RefCell<Option<CString>> = RefCell::new(None);
    }

    assert!(!key.is_null());

    let key = unsafe { &*key };

    PRIVATE_KEY_DISPLAY
        .with(|cell| cell.borrow_mut().insert(CString::new(key.to_string()).unwrap()).as_ptr())
}

/// Releases memory associated with the public key.
#[no_mangle]
pub extern "C" fn hedera_public_key_free(key: *mut PublicKey) {
    assert!(!key.is_null());

    let _key = unsafe { Box::from_raw(key) };
}
