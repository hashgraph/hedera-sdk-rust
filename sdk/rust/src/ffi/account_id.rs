use std::os::raw::{c_char, c_int};
use std::str::FromStr;

use crate::ffi::util::cstr_from_ptr;
use crate::AccountId;

/// Parse a Hedera `AccountId` from the passed string.
#[no_mangle]
pub extern "C" fn hedera_account_id_from_string(s: *const c_char, id: *mut AccountId) -> c_int {
    assert!(!id.is_null());

    let s = unsafe { cstr_from_ptr(s) };

    // TODO: handle errors
    let parsed = AccountId::from_str(&s).unwrap();

    unsafe {
        *id = parsed;
    }

    0
}
